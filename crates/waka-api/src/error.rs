//! API error types for `waka-api`.

/// All errors that can be returned by the `WakaTime` API client.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApiError {
    /// The API key is invalid or was not provided.
    /// Maps to HTTP 401.
    #[error("Unauthorized: invalid or missing API key")]
    Unauthorized,

    /// The API rate limit has been exceeded.
    /// Maps to HTTP 429.
    ///
    /// `retry_after` contains the number of seconds to wait before retrying,
    /// if the server included a `Retry-After` header.
    #[error("Rate limited — retry after {retry_after:?} seconds")]
    RateLimit {
        /// Seconds to wait before retrying, if known.
        retry_after: Option<u64>,
    },

    /// The requested resource was not found.
    /// Maps to HTTP 404.
    #[error("Not found")]
    NotFound,

    /// An unexpected server-side error occurred.
    /// Maps to HTTP 5xx.
    #[error("Server error (HTTP {status})")]
    ServerError {
        /// The raw HTTP status code returned by the server.
        status: u16,
    },

    /// A network-level error occurred (connection refused, timeout, DNS, etc.).
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// The server returned a response that could not be deserialized.
    #[error("Failed to parse API response: {0}")]
    ParseError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthorized_display() {
        let err = ApiError::Unauthorized;
        assert_eq!(err.to_string(), "Unauthorized: invalid or missing API key");
    }

    #[test]
    fn rate_limit_with_retry_after() {
        let err = ApiError::RateLimit {
            retry_after: Some(60),
        };
        assert!(err.to_string().contains("60"));
    }

    #[test]
    fn rate_limit_without_retry_after() {
        let err = ApiError::RateLimit { retry_after: None };
        assert!(err.to_string().contains("None"));
    }

    #[test]
    fn not_found_display() {
        let err = ApiError::NotFound;
        assert_eq!(err.to_string(), "Not found");
    }

    #[test]
    fn server_error_display() {
        let err = ApiError::ServerError { status: 503 };
        assert!(err.to_string().contains("503"));
    }

    #[test]
    fn parse_error_display() {
        let err = ApiError::ParseError("missing field `id`".to_owned());
        assert!(err.to_string().contains("missing field `id`"));
    }
}
