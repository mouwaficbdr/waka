//! Integration tests for stats command flows using a mock `WakaTime` API server.
//!
//! These tests validate `WakaClient::summaries()` round-trips against fixture
//! data, mirroring the level of coverage provided for the auth flow.

use chrono::Local;
use waka_api::{SummaryParams, WakaClient};
use wiremock::matchers::{header_exists, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ─── helpers ──────────────────────────────────────────────────────────────────

fn fixture(name: &str) -> String {
    let base = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // crates/waka -> crates
        .and_then(|p| p.parent()) // crates -> workspace root
        .expect("workspace root must exist")
        .join("tests/fixtures")
        .join(name);
    std::fs::read_to_string(&base)
        .unwrap_or_else(|e| panic!("failed to read fixture {}: {e}", base.display()))
}

fn mock_client(server: &MockServer) -> WakaClient {
    WakaClient::with_base_url("waka_testkey123", &format!("{}/api/v1/", server.uri()))
        .expect("test base URL must be valid")
}

// ─── happy path ───────────────────────────────────────────────────────────────

/// `summaries()` deserialises a real fixture and returns at least one day of
/// data with a non-zero `grand_total.total_seconds`.
#[tokio::test]
async fn summaries_today_returns_data() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current/summaries"))
        .and(header_exists("authorization"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(fixture("summaries_today.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let resp = client
        .summaries(SummaryParams::today())
        .await
        .expect("summaries should succeed");

    assert!(!resp.data.is_empty(), "expected at least one summary day");
    let gt = &resp.data[0].grand_total;
    assert!(
        gt.total_seconds > 0.0,
        "grand_total.total_seconds must be positive, got {:.1}",
        gt.total_seconds
    );
}

/// `summaries()` for a 7-day range also returns data using the week fixture.
#[tokio::test]
async fn summaries_week_range_returns_data() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current/summaries"))
        .and(header_exists("authorization"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(fixture("summaries_week.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let today = Local::now().date_naive();
    let start = today
        .checked_sub_days(chrono::Days::new(6))
        .expect("7-day range must be computable");

    let client = mock_client(&server);
    let resp = client
        .summaries(SummaryParams::for_range(start, today))
        .await
        .expect("week summaries should succeed");

    assert!(
        !resp.data.is_empty(),
        "expected at least one day in the week range"
    );
}

/// `summaries()` with a project filter sends the `project` query parameter.
#[tokio::test]
async fn summaries_project_filter_sends_query_param() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current/summaries"))
        .and(query_param("project", "my-project"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(fixture("summaries_today.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let params = SummaryParams::today().project("my-project");
    let resp = client
        .summaries(params)
        .await
        .expect("filtered summaries should succeed");

    // Server only responds if the query param matches — success implies correct param.
    assert!(!resp.data.is_empty());
}

// ─── error cases ──────────────────────────────────────────────────────────────

/// An invalid API key causes `summaries()` to return `Unauthorized`.
#[tokio::test]
async fn summaries_unauthorized_on_invalid_key() {
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

    let client = WakaClient::with_base_url("waka_badkey", &format!("{}/api/v1/", server.uri()))
        .expect("base URL must be valid");

    let err = client
        .summaries(SummaryParams::today())
        .await
        .expect_err("should fail with invalid key");

    assert!(
        matches!(err, waka_api::ApiError::Unauthorized),
        "expected Unauthorized, got {err:?}"
    );
}

/// A 500 server error causes `summaries()` to return a server error after
/// exhausting retries.
#[tokio::test]
async fn summaries_fails_gracefully_on_server_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current/summaries"))
        .respond_with(
            ResponseTemplate::new(500)
                .set_body_string(fixture("errors/500_server_error.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let err = client
        .summaries(SummaryParams::today())
        .await
        .expect_err("should fail on 500");

    assert!(
        matches!(err, waka_api::ApiError::ServerError { .. }),
        "expected ServerError after retries, got {err:?}"
    );
}
