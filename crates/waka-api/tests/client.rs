//! Integration tests for [`WakaClient`] using a `wiremock` mock server.

use std::time::Duration;

use waka_api::{ApiError, SummaryParams, WakaClient};
use wiremock::matchers::{header, header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ─── helpers ──────────────────────────────────────────────────────────────────

/// Loads the fixture at `tests/fixtures/<name>`.
fn fixture(name: &str) -> String {
    let base = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // crates/waka-api -> crates
        .unwrap() // safe: always has a parent
        .parent() // crates -> workspace root
        .unwrap() // safe: always has a parent
        .join("tests/fixtures")
        .join(name);
    std::fs::read_to_string(&base)
        .unwrap_or_else(|e| panic!("failed to read fixture {}: {e}", base.display()))
}

/// Returns a [`WakaClient`] pointed at the mock server.
fn client(server: &MockServer) -> WakaClient {
    WakaClient::with_base_url("test_key", &format!("{}/api/v1/", server.uri()))
        .expect("with_base_url should succeed for a valid URL")
}

// ─── happy-path ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn me_returns_user_on_200() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current"))
        .and(header_exists("authorization"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(fixture("user_current.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let c = client(&server);
    let user = c.me().await.expect("should succeed on 200");

    assert_eq!(user.username, "janedeveloper");
}

// ─── 401 ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn me_returns_unauthorized_on_401() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_string(fixture("errors/401_unauthorized.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let c = client(&server);
    let err = c.me().await.expect_err("should fail on 401");

    assert!(
        matches!(err, ApiError::Unauthorized),
        "expected Unauthorized, got {err:?}"
    );
}

// ─── 429 ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn me_returns_rate_limit_on_429_with_retry_after() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current"))
        .respond_with(
            ResponseTemplate::new(429)
                .set_body_string(fixture("errors/429_rate_limit.json"))
                .insert_header("content-type", "application/json")
                .insert_header("Retry-After", "60"),
        )
        .mount(&server)
        .await;

    let c = client(&server);
    let err = c.me().await.expect_err("should fail on 429");

    assert!(
        matches!(
            err,
            ApiError::RateLimit {
                retry_after: Some(60)
            }
        ),
        "expected RateLimit{{retry_after: Some(60)}}, got {err:?}"
    );
}

#[tokio::test]
async fn me_returns_rate_limit_on_429_without_retry_after() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current"))
        .respond_with(
            ResponseTemplate::new(429)
                .set_body_string(fixture("errors/429_rate_limit.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let c = client(&server);
    let err = c.me().await.expect_err("should fail on 429");

    assert!(
        matches!(err, ApiError::RateLimit { retry_after: None }),
        "expected RateLimit{{retry_after: None}}, got {err:?}"
    );
}

// ─── 5xx retry ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn me_returns_server_error_after_retries_on_500() {
    let server = MockServer::start().await;

    // Return 500 on all 3 attempts.
    Mock::given(method("GET"))
        .and(path("/api/v1/users/current"))
        .respond_with(
            ResponseTemplate::new(500)
                .set_body_string(fixture("errors/500_server_error.json"))
                .insert_header("content-type", "application/json")
                .set_delay(Duration::from_millis(0)),
        )
        .expect(3) // must be called exactly MAX_ATTEMPTS (3) times
        .mount(&server)
        .await;

    let c = client(&server);
    let err = c.me().await.expect_err("should fail after 3 attempts");

    assert!(
        matches!(err, ApiError::ServerError { status: 500 }),
        "expected ServerError{{status: 500}}, got {err:?}"
    );
}

// ─── auth header ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn me_sends_basic_auth_header() {
    let server = MockServer::start().await;

    // Authorization header for Basic auth: base64("test_key:") = "dGVzdF9rZXk6"
    Mock::given(method("GET"))
        .and(path("/api/v1/users/current"))
        .and(header("authorization", "Basic dGVzdF9rZXk6"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(fixture("user_current.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let c = client(&server);
    c.me()
        .await
        .expect("should succeed with correct Basic auth header");
}

// ─── with_base_url validation ────────────────────────────────────────────────

#[test]
fn with_base_url_rejects_invalid_url() {
    let err = WakaClient::with_base_url("key", "not a url");
    assert!(err.is_err(), "expected Err for invalid URL, got Ok");
}

// ─── summaries ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn summaries_returns_response_on_200() {
    use chrono::NaiveDate;
    use wiremock::matchers::query_param;

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current/summaries"))
        .and(query_param("start", "2025-01-13"))
        .and(query_param("end", "2025-01-13"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(fixture("summaries_today.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let c = client(&server);
    let date = NaiveDate::from_ymd_opt(2025, 1, 13).unwrap();
    let params = SummaryParams::for_range(date, date);
    let resp = c.summaries(params).await.expect("should succeed on 200");

    assert!(!resp.data.is_empty(), "response data should not be empty");
}

#[tokio::test]
async fn summaries_sends_project_query_param() {
    use chrono::NaiveDate;
    use wiremock::matchers::query_param;

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current/summaries"))
        .and(query_param("project", "my-saas"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(fixture("summaries_today.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let c = client(&server);
    let date = NaiveDate::from_ymd_opt(2025, 1, 13).unwrap();
    let params = SummaryParams::for_range(date, date).project("my-saas");
    c.summaries(params)
        .await
        .expect("should forward project param");
}

#[tokio::test]
async fn summaries_returns_unauthorized_on_401() {
    use chrono::NaiveDate;

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current/summaries"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_string(fixture("errors/401_unauthorized.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let c = client(&server);
    let date = NaiveDate::from_ymd_opt(2025, 1, 13).unwrap();
    let params = SummaryParams::for_range(date, date);
    let err = c.summaries(params).await.expect_err("should fail on 401");

    assert!(
        matches!(err, ApiError::Unauthorized),
        "expected Unauthorized, got {err:?}"
    );
}
