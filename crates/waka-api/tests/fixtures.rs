//! Integration tests that validate fixture JSON files against the API types.
//!
//! These tests ensure that the fixture files are valid and parseable
//! by the `waka-api` types.

use waka_api::types::{StatsResponse, SummaryResponse, UserResponse};

fn fixture(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests/fixtures")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

#[test]
fn fixture_user_current_parses() {
    let json = fixture("user_current.json");
    let resp: UserResponse = serde_json::from_str(&json).expect("user_current.json must parse");
    assert_eq!(resp.data.username, "janedeveloper");
    assert_eq!(resp.data.timezone, "Europe/Berlin");
}

#[test]
fn fixture_summaries_today_parses() {
    let json = fixture("summaries_today.json");
    let resp: SummaryResponse =
        serde_json::from_str(&json).expect("summaries_today.json must parse");
    assert_eq!(resp.data.len(), 1);
    let day = &resp.data[0];
    assert_eq!(day.grand_total.hours, 6);
    assert_eq!(day.grand_total.minutes, 42);
    assert!(!day.languages.is_empty());
    assert_eq!(day.languages[0].name, "Rust");
}

#[test]
fn fixture_summaries_week_parses() {
    let json = fixture("summaries_week.json");
    let resp: SummaryResponse =
        serde_json::from_str(&json).expect("summaries_week.json must parse");
    assert_eq!(resp.data.len(), 7, "week fixture should have 7 days");
}

#[test]
fn fixture_stats_last_7_days_parses() {
    let json = fixture("stats_last_7_days.json");
    let resp: StatsResponse =
        serde_json::from_str(&json).expect("stats_last_7_days.json must parse");
    assert_eq!(resp.data.range, "last_7_days");
    assert_eq!(resp.data.username, "janedeveloper");
    assert!(resp.data.best_day.is_some());
}
