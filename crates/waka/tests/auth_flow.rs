//! Integration tests for auth command flows using a mock `WakaTime` API server.
//!
//! These tests validate the full auth command handler behavior against a
//! `wiremock` mock server without touching the system keychain.

use waka_api::WakaClient;
use wiremock::matchers::{header_exists, method, path};
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

// ─── WakaClient::me() validates API key ───────────────────────────────────────

/// Validates that the `WakaClient` authenticates using HTTP Basic auth and
/// returns the user on a 200 response — simulating the login validation step.
#[tokio::test]
async fn login_validation_succeeds_on_valid_key() {
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

    let client =
        WakaClient::with_base_url("waka_validkey123", &format!("{}/api/v1/", server.uri()))
            .expect("base URL must be valid");

    let user = client.me().await.expect("validation should succeed");
    assert_eq!(user.username, "janedeveloper");
    assert_eq!(user.display_name, "Jane Developer");
}

/// Validates that an invalid API key causes `me()` to return
/// [`waka_api::ApiError::Unauthorized`], which is the error that the login
/// handler maps to a user-friendly message.
#[tokio::test]
async fn login_validation_fails_on_invalid_key() {
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

    let client = WakaClient::with_base_url("waka_badkey", &format!("{}/api/v1/", server.uri()))
        .expect("base URL must be valid");

    let err = client.me().await.expect_err("should reject invalid key");
    assert!(
        matches!(err, waka_api::ApiError::Unauthorized),
        "expected Unauthorized, got {err:?}"
    );
}

/// Validates that `me()` returns a useful error on a server-side 500, which
/// the status and login handlers surface as a network/server error.
#[tokio::test]
async fn status_check_fails_gracefully_on_server_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/users/current"))
        .respond_with(
            ResponseTemplate::new(500)
                .set_body_string(fixture("errors/500_server_error.json"))
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    let client = WakaClient::with_base_url("waka_key", &format!("{}/api/v1/", server.uri()))
        .expect("base URL must be valid");

    // After MAX_ATTEMPTS (3) retries the client should give up with a server error.
    let err = client.me().await.expect_err("should fail on 500");
    assert!(
        matches!(err, waka_api::ApiError::ServerError { .. }),
        "expected ServerError, got {err:?}"
    );
}
