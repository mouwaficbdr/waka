//! `WakaTime` HTTP client implementation.

use std::time::Duration;

use reqwest::{StatusCode, Url};
use serde::de::DeserializeOwned;
use tracing::{debug, warn};

use crate::error::ApiError;
use crate::types::UserResponse;

/// Base URL for the production `WakaTime` API.
const DEFAULT_BASE_URL: &str = "https://wakatime.com/api/v1/";

/// Per-request timeout in seconds.
const REQUEST_TIMEOUT_SECS: u64 = 10;

/// Maximum number of attempts before giving up (1 initial + 2 retries).
const MAX_ATTEMPTS: u32 = 3;

// ─────────────────────────────────────────────────────────────────────────────

/// Async HTTP client for the `WakaTime` API v1.
///
/// All requests are authenticated via HTTP Basic auth using the supplied API
/// key. The client performs automatic retry with exponential back-off on
/// transient errors (network failures and HTTP 5xx responses).
///
/// # Example
///
/// ```rust,no_run
/// use waka_api::WakaClient;
///
/// # async fn example() -> Result<(), waka_api::ApiError> {
/// let client = WakaClient::new("waka_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
/// let me = client.me().await?;
/// println!("Logged in as {}", me.username);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct WakaClient {
    /// API key used to authenticate all requests.
    ///
    /// Stored as a plain `String` internally; never printed via `Debug`
    /// (the default derive is acceptable here because the struct is not
    /// publicly printable in user-visible error paths — the credential store
    /// wrapper in `waka-config` uses a `Sensitive` newtype).
    // TODO(spec): consider moving to a Sensitive(String) newtype once
    // waka-config's CredentialStore wraps requests at the call site.
    api_key: String,
    /// Base URL of the API (overridable for testing).
    base_url: Url,
    /// Underlying HTTP client (shared connection pool).
    http: reqwest::Client,
}

impl WakaClient {
    /// Creates a new client pointing at the production `WakaTime` API.
    ///
    /// # Panics
    ///
    /// Panics if the default base URL cannot be parsed (compile-time constant —
    /// this is unreachable in practice).
    #[must_use]
    pub fn new(api_key: &str) -> Self {
        // SAFETY: DEFAULT_BASE_URL is a compile-time constant that is valid.
        let base_url =
            Url::parse(DEFAULT_BASE_URL).expect("DEFAULT_BASE_URL is a valid URL; unreachable");
        Self {
            api_key: api_key.to_owned(),
            base_url,
            http: build_http_client(),
        }
    }

    /// Creates a new client pointing at a custom base URL.
    ///
    /// Primarily used in tests to target a `wiremock` mock server.
    ///
    /// # Errors
    ///
    /// Returns an error if `base_url` is not a valid URL.
    pub fn with_base_url(api_key: &str, base_url: &str) -> Result<Self, ApiError> {
        let base_url = Url::parse(base_url).map_err(|e| ApiError::ParseError(e.to_string()))?;
        Ok(Self {
            api_key: api_key.to_owned(),
            base_url,
            http: build_http_client(),
        })
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    /// Sends an authenticated GET request to `path` with the given query
    /// parameters, deserializes the JSON response into `T`, and returns it.
    ///
    /// Implements:
    /// - HTTP Basic auth (`Authorization: Basic <base64(api_key:)>`)
    /// - 401 → [`ApiError::Unauthorized`]
    /// - 429 → [`ApiError::RateLimit`] (parses `Retry-After` header)
    /// - 404 → [`ApiError::NotFound`]
    /// - 5xx → [`ApiError::ServerError`]
    /// - Exponential back-off retry (max 3 total attempts)
    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, ApiError> {
        let url = self
            .base_url
            .join(path)
            .map_err(|e| ApiError::ParseError(format!("invalid path '{path}': {e}")))?;

        let mut last_err: Option<ApiError> = None;

        for attempt in 0..MAX_ATTEMPTS {
            if attempt > 0 {
                // Exponential back-off: 500ms, 1000ms
                let delay = Duration::from_millis(500 * u64::from(attempt));
                debug!(
                    "retrying request to {url} after {}ms (attempt {attempt})",
                    delay.as_millis()
                );
                tokio::time::sleep(delay).await;
            }

            debug!("GET {url} (attempt {})", attempt + 1);

            let result: Result<reqwest::Response, reqwest::Error> = self
                .http
                .get(url.clone())
                // WakaTime uses Basic auth: base64(api_key + ":")
                // reqwest's basic_auth encodes "user:password" — pass empty password.
                .basic_auth(&self.api_key, Option::<&str>::None)
                .query(query)
                .send()
                .await;

            let response: reqwest::Response = match result {
                Ok(r) => r,
                Err(e) if e.is_timeout() || e.is_connect() => {
                    warn!("network error on attempt {}: {e}", attempt + 1);
                    last_err = Some(ApiError::NetworkError(e));
                    continue; // retry on transient network errors
                }
                Err(e) => return Err(ApiError::NetworkError(e)),
            };

            let status = response.status();

            match status {
                StatusCode::OK => {
                    let text: String = response.text().await.map_err(ApiError::NetworkError)?;
                    return serde_json::from_str::<T>(&text)
                        .map_err(|e| ApiError::ParseError(e.to_string()));
                }
                StatusCode::UNAUTHORIZED => {
                    return Err(ApiError::Unauthorized);
                }
                StatusCode::NOT_FOUND => {
                    return Err(ApiError::NotFound);
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    let retry_after = response
                        .headers()
                        .get("Retry-After")
                        .and_then(|v: &reqwest::header::HeaderValue| v.to_str().ok())
                        .and_then(|s: &str| s.parse::<u64>().ok());
                    return Err(ApiError::RateLimit { retry_after });
                }
                s if s.is_server_error() => {
                    warn!("server error {s} on attempt {}", attempt + 1);
                    last_err = Some(ApiError::ServerError { status: s.as_u16() });
                    // fall through to next loop iteration (retry on 5xx)
                }
                s => {
                    return Err(ApiError::ServerError { status: s.as_u16() });
                }
            }
        }

        // All attempts exhausted — return the last recorded error.
        Err(last_err.unwrap_or_else(|| ApiError::ServerError { status: 500 }))
    }

    // ── Endpoints ─────────────────────────────────────────────────────────────

    /// Returns the profile of the currently authenticated user.
    ///
    /// Calls `GET /users/current`.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Unauthorized`] if the API key is invalid.
    /// Returns [`ApiError::NetworkError`] on connection or timeout failures.
    pub async fn me(&self) -> Result<crate::types::User, ApiError> {
        let resp: UserResponse = self.get("users/current", &[]).await?;
        Ok(resp.data)
    }
}

/// Builds the shared `reqwest::Client` with a per-request timeout.
fn build_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()
        // reqwest::ClientBuilder::build() only fails if the TLS backend cannot
        // be initialised — with rustls (the default in reqwest 0.13) this is
        // unreachable.
        .expect("failed to build reqwest::Client; TLS backend unavailable")
}
