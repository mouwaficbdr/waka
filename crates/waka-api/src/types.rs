//! `WakaTime` API response types.
//!
//! All types derive [`Debug`], [`Clone`], [`serde::Serialize`], and
//! [`serde::Deserialize`]. Unknown JSON fields are silently ignored via
//! `#[serde(deny_unknown_fields)]` is **not** used — this keeps the client
//! forward-compatible with new API fields.

// The WakaTime API returns several structs with more than 3 boolean fields
// (e.g. Goal has 5, Stats has 8). These fields mirror the upstream API
// exactly and cannot be meaningfully replaced with enums or bitflags.
#![allow(clippy::struct_excessive_bools)]

use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────────
// User
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level envelope returned by `GET /users/current`.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    /// The user object.
    pub data: User,
}

/// A `WakaTime` user account.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier (UUID).
    pub id: String,
    /// The user's login handle.
    pub username: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Full legal name (may be `None` if not set).
    pub full_name: Option<String>,
    /// Email address (only present when the authenticated user requests their
    /// own profile).
    pub email: Option<String>,
    /// URL of the user's avatar image.
    pub photo: Option<String>,
    /// IANA timezone string (e.g. `"America/New_York"`).
    pub timezone: String,
    /// Personal website URL.
    pub website: Option<String>,
    /// Human-readable website URL (without protocol prefix).
    pub human_readable_website: Option<String>,
    /// Geographic location string.
    pub location: Option<String>,
    /// Subscription plan (e.g. `"free"`, `"premium"`).
    pub plan: Option<String>,
    /// Absolute URL of the user's public profile.
    pub profile_url: Option<String>,
    /// Whether the user's email address has been verified.
    #[serde(default)]
    pub is_email_confirmed: bool,
    /// Whether the user is open to work.
    pub is_hireable: Option<bool>,
    /// Whether coding time is visible on the public profile.
    pub logged_time_public: Option<bool>,
    /// Whether this account is in write-only mode.
    #[serde(default)]
    pub writes_only: bool,
    /// Heartbeat timeout in minutes.
    pub timeout: Option<u32>,
    /// Whether the user prefers 24-hour time format.
    pub time_format_24hr: Option<bool>,
    /// ISO 8601 timestamp when the account was created.
    pub created_at: String,
    /// ISO 8601 timestamp when the account was last modified.
    pub modified_at: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Summaries
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level envelope returned by `GET /users/current/summaries`.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryResponse {
    /// Per-day summary entries (one per calendar day in the requested range).
    pub data: Vec<SummaryData>,
    /// ISO 8601 end of the requested range.
    pub end: String,
    /// ISO 8601 start of the requested range.
    pub start: String,
}

/// Coding activity summary for a single calendar day.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SummaryData {
    /// Time broken down by activity category (coding, browsing, etc.).
    pub categories: Vec<SummaryEntry>,
    /// Time broken down by detected dependency / library.
    #[serde(default)]
    pub dependencies: Vec<SummaryEntry>,
    /// Time broken down by editor.
    pub editors: Vec<SummaryEntry>,
    /// Daily grand total across all activity.
    pub grand_total: GrandTotal,
    /// Time broken down by programming language.
    pub languages: Vec<SummaryEntry>,
    /// Time broken down by machine / hostname.
    #[serde(default)]
    pub machines: Vec<MachineEntry>,
    /// Time broken down by operating system.
    pub operating_systems: Vec<SummaryEntry>,
    /// Time broken down by project.
    pub projects: Vec<SummaryEntry>,
    /// The date range this entry covers.
    pub range: SummaryRange,
}

/// A single time-breakdown entry (language, project, editor, OS, etc.).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SummaryEntry {
    /// Human-readable duration in `HH:MM` format.
    pub digital: String,
    /// Whole hours component.
    pub hours: u32,
    /// Whole minutes component (0–59).
    pub minutes: u32,
    /// Entity name (e.g. `"Python"`, `"my-project"`).
    pub name: String,
    /// Percentage of total time for the period (0.0–100.0).
    pub percent: f64,
    /// Whole seconds component (0–59).
    pub seconds: u32,
    /// Full human-readable duration (e.g. `"3 hrs 30 mins"`).
    pub text: String,
    /// Total duration in seconds (fractional).
    pub total_seconds: f64,
}

/// A machine / hostname time-breakdown entry.
///
/// This is structurally similar to [`SummaryEntry`] but includes the
/// `machine_name_id` field returned by the API.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MachineEntry {
    /// Human-readable duration in `HH:MM` format.
    pub digital: String,
    /// Whole hours component.
    pub hours: u32,
    /// Disambiguated machine identifier returned by the API.
    pub machine_name_id: String,
    /// Whole minutes component (0–59).
    pub minutes: u32,
    /// Human-readable machine name.
    pub name: String,
    /// Percentage of total time for the period (0.0–100.0).
    pub percent: f64,
    /// Whole seconds component (0–59).
    pub seconds: u32,
    /// Full human-readable duration (e.g. `"1 hr 15 mins"`).
    pub text: String,
    /// Total duration in seconds (fractional).
    pub total_seconds: f64,
}

/// Grand total coding time for a single calendar day.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrandTotal {
    /// Human-readable duration in `HH:MM` format.
    pub digital: String,
    /// Whole hours component.
    pub hours: u32,
    /// Whole minutes component (0–59).
    pub minutes: u32,
    /// Whole seconds component (0–59).
    pub seconds: u32,
    /// Full human-readable duration (e.g. `"6 hrs 42 mins"`).
    pub text: String,
    /// Total duration in seconds (fractional).
    pub total_seconds: f64,
}

/// Date range metadata attached to a [`SummaryData`] entry.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SummaryRange {
    /// Calendar date string (e.g. `"2025-01-13"`). Present on single-day
    /// entries; may be absent on range queries.
    pub date: Option<String>,
    /// ISO 8601 end timestamp.
    pub end: String,
    /// ISO 8601 start timestamp.
    pub start: String,
    /// Human-readable description (e.g. `"today"`, `"yesterday"`).
    pub text: Option<String>,
    /// IANA timezone used when computing the range.
    pub timezone: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Projects list
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level envelope returned by `GET /users/current/projects`.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectsResponse {
    /// The list of projects.
    pub data: Vec<Project>,
}

/// A `WakaTime` project.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Optional badge URL associated with the project.
    pub badge: Option<String>,
    /// Optional hex color code used in the `WakaTime` web UI.
    pub color: Option<String>,
    /// ISO 8601 timestamp when the project was first seen.
    pub created_at: String,
    /// Whether the project has a public shareable URL.
    pub has_public_url: bool,
    /// Human-readable last-heartbeat description (e.g. `"2 hours ago"`).
    pub human_readable_last_heartbeat_at: String,
    /// Unique project identifier (UUID).
    pub id: String,
    /// ISO 8601 timestamp of the last received heartbeat.
    pub last_heartbeat_at: String,
    /// Project name.
    pub name: String,
    /// Linked repository URL (if configured).
    pub repository: Option<String>,
    /// Public project URL (if `has_public_url` is `true`).
    pub url: Option<String>,
    /// URL-encoded version of the project name.
    pub urlencoded_name: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Stats
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level envelope returned by `GET /users/current/stats/{range}`.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    /// The aggregated stats object.
    pub data: Stats,
}

/// Aggregated coding stats for a predefined time range.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    /// The day with the most coding activity.
    pub best_day: Option<BestDay>,
    /// Time broken down by activity category.
    pub categories: Vec<SummaryEntry>,
    /// ISO 8601 timestamp when this stats snapshot was created.
    pub created_at: String,
    /// Average daily coding time in seconds.
    pub daily_average: f64,
    /// Average daily coding time including "other language" seconds.
    pub daily_average_including_other_language: f64,
    /// Number of days in the range including weekends/holidays.
    pub days_including_holidays: u32,
    /// Number of working days in the range.
    pub days_minus_holidays: u32,
    /// Time broken down by editor.
    pub editors: Vec<SummaryEntry>,
    /// ISO 8601 end of the stats range.
    pub end: String,
    /// Number of holiday days detected in the range.
    pub holidays: u32,
    /// Human-readable average daily coding time.
    pub human_readable_daily_average: String,
    /// Human-readable description of the range (e.g. `"last 7 days"`).
    pub human_readable_range: String,
    /// Human-readable total coding time.
    pub human_readable_total: String,
    /// Unique stats record identifier.
    pub id: String,
    /// Whether the stats snapshot is up to date.
    pub is_up_to_date: bool,
    /// Time broken down by programming language.
    pub languages: Vec<SummaryEntry>,
    /// Time broken down by machine.
    pub machines: Vec<MachineEntry>,
    /// ISO 8601 timestamp when this snapshot was last modified.
    pub modified_at: Option<String>,
    /// Time broken down by operating system.
    pub operating_systems: Vec<SummaryEntry>,
    /// How fully computed the stats are (0–100).
    pub percent_calculated: u32,
    /// Time broken down by project.
    pub projects: Vec<SummaryEntry>,
    /// Named range (e.g. `"last_7_days"`, `"last_30_days"`).
    pub range: String,
    /// ISO 8601 start of the stats range.
    pub start: String,
    /// Computation status (e.g. `"ok"`, `"pending_update"`).
    pub status: String,
    /// Heartbeat timeout in minutes used for this calculation.
    pub timeout: u32,
    /// IANA timezone used when computing the stats.
    pub timezone: String,
    /// Total coding time in seconds.
    pub total_seconds: f64,
    /// Total coding time including "other language" in seconds.
    pub total_seconds_including_other_language: f64,
    /// Owner user identifier (UUID).
    pub user_id: String,
    /// Owner username.
    pub username: String,
    /// Whether the account is in write-only mode.
    pub writes_only: bool,
}

/// The single best (most productive) day in a stats range.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestDay {
    /// Calendar date (e.g. `"2025-01-13"`).
    pub date: String,
    /// Human-readable total for that day.
    pub text: String,
    /// Total coding seconds for that day.
    pub total_seconds: f64,
}

// ─────────────────────────────────────────────────────────────────────────────
// Goals
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level envelope returned by `GET /users/current/goals`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalsResponse {
    /// The list of goals.
    pub data: Vec<Goal>,
    /// Total number of goals.
    pub total: u32,
    /// Total number of pages.
    pub total_pages: u32,
}

/// A single coding goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    /// Per-period chart data (populated when fetching goal details).
    pub chart_data: Option<Vec<GoalChartEntry>>,
    /// ISO 8601 timestamp when the goal was created.
    pub created_at: String,
    /// Goal period (e.g. `"day"`, `"week"`).
    pub delta: String,
    /// Editors this goal is restricted to (empty = all editors).
    #[serde(default)]
    pub editors: Vec<String>,
    /// Unique goal identifier (UUID).
    pub id: String,
    /// Days of the week to ignore (e.g. `["saturday", "sunday"]`).
    #[serde(default)]
    pub ignore_days: Vec<String>,
    /// Whether days with zero activity are excluded from streak calculations.
    pub ignore_zero_days: bool,
    /// Target improvement percentage over baseline.
    pub improve_by_percent: Option<f64>,
    /// Whether the goal is active.
    pub is_enabled: bool,
    /// Whether passing means staying *below* the target.
    pub is_inverse: bool,
    /// Whether the goal is temporarily snoozed.
    pub is_snoozed: bool,
    /// Whether achievements are tweeted automatically.
    pub is_tweeting: bool,
    /// Languages this goal is restricted to (empty = all languages).
    #[serde(default)]
    pub languages: Vec<String>,
    /// ISO 8601 timestamp when the goal was last modified.
    pub modified_at: String,
    /// Projects this goal is restricted to (empty = all projects).
    #[serde(default)]
    pub projects: Vec<String>,
    /// Status for the most recent period.
    pub range_status: String,
    /// Human-readable explanation of the status.
    pub range_status_reason: String,
    /// Target coding seconds per period.
    pub seconds: f64,
    /// Overall goal status (e.g. `"success"`, `"fail"`, `"ignored"`).
    pub status: String,
    /// Human-readable goal title.
    pub title: String,
    /// Goal type (e.g. `"coding"`).
    #[serde(rename = "type")]
    pub goal_type: String,
}

/// One data point in a goal's progress chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalChartEntry {
    /// Actual coding seconds logged during this period.
    pub actual_seconds: f64,
    /// Human-readable actual coding time.
    pub actual_seconds_text: String,
    /// Target coding seconds for this period.
    pub goal_seconds: f64,
    /// Human-readable target coding time.
    pub goal_seconds_text: String,
    /// Date range this data point covers.
    pub range: GoalChartRange,
    /// Status for this period (e.g. `"success"`, `"fail"`).
    pub range_status: String,
    /// Human-readable explanation of the status.
    pub range_status_reason: String,
}

/// Date range metadata for a [`GoalChartEntry`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalChartRange {
    /// Calendar date (e.g. `"2025-01-13"`).
    pub date: String,
    /// ISO 8601 end timestamp.
    pub end: String,
    /// ISO 8601 start timestamp.
    pub start: String,
    /// Human-readable description (e.g. `"Mon, Jan 13"`).
    pub text: String,
    /// IANA timezone used when computing the range.
    pub timezone: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Leaderboard
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level envelope returned by `GET /users/current/leaderboards`.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    /// The current authenticated user's entry, if they appear in this page.
    pub current_user: Option<LeaderboardEntry>,
    /// The leaderboard entries for this page.
    pub data: Vec<LeaderboardEntry>,
    /// Language filter applied (if any).
    pub language: Option<String>,
    /// ISO 8601 timestamp when the leaderboard was last updated.
    pub modified_at: String,
    /// Current page number (1-based).
    pub page: u32,
    /// Date range this leaderboard covers.
    pub range: LeaderboardRange,
    /// Heartbeat timeout in minutes used for ranking.
    pub timeout: u32,
    /// Total number of pages available.
    pub total_pages: u32,
    /// Whether the leaderboard is restricted to write-only accounts.
    pub writes_only: bool,
}

/// A single entry on the leaderboard.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    /// The rank of this user (1 = top coder).
    pub rank: u32,
    /// Aggregated coding totals for this user.
    pub running_total: RunningTotal,
    /// Public user information.
    pub user: LeaderboardUser,
}

/// Aggregated coding totals used on the leaderboard.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningTotal {
    /// Average daily coding time in seconds.
    pub daily_average: f64,
    /// Human-readable average daily coding time.
    pub human_readable_daily_average: String,
    /// Human-readable total coding time for the period.
    pub human_readable_total: String,
    /// Top languages for this user (may be empty if privacy settings prevent
    /// disclosure).
    #[serde(default)]
    pub languages: Vec<SummaryEntry>,
    /// ISO 8601 timestamp when this total was last computed.
    pub modified_at: String,
    /// Total coding seconds for the leaderboard period.
    pub total_seconds: f64,
}

/// Public user information exposed on the leaderboard.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardUser {
    /// Human-readable display name.
    pub display_name: String,
    /// Public email (empty string if not shared).
    pub email: Option<String>,
    /// Full legal name.
    pub full_name: Option<String>,
    /// Human-readable website URL.
    pub human_readable_website: Option<String>,
    /// Unique user identifier (UUID).
    pub id: String,
    /// Whether the email address is publicly visible.
    pub is_email_public: bool,
    /// Whether the user is open to work.
    pub is_hireable: bool,
    /// Geographic location string.
    pub location: Option<String>,
    /// Avatar image URL.
    pub photo: Option<String>,
    /// Whether the avatar is publicly visible.
    pub photo_public: bool,
    /// Absolute URL of the user's public profile.
    pub profile_url: Option<String>,
    /// Public email address (distinct from account email).
    pub public_email: Option<String>,
    /// Login handle.
    pub username: Option<String>,
    /// Personal website URL.
    pub website: Option<String>,
}

/// Date range metadata for a [`LeaderboardResponse`].
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardRange {
    /// End date string (e.g. `"2025-01-13"`).
    pub end_date: String,
    /// Short human-readable end date (e.g. `"Jan 13"`).
    pub end_text: String,
    /// Named range identifier (e.g. `"last_7_days"`).
    pub name: String,
    /// Start date string (e.g. `"2025-01-07"`).
    pub start_date: String,
    /// Short human-readable start date (e.g. `"Jan 7"`).
    pub start_text: String,
    /// Full human-readable range description (e.g. `"Last 7 Days"`).
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deserializing a minimal [`GrandTotal`] from JSON must succeed and
    /// `total_seconds` must be preserved exactly.
    #[test]
    fn grand_total_deserialize() {
        let json = r#"{
            "digital": "6:42",
            "hours": 6,
            "minutes": 42,
            "seconds": 0,
            "text": "6 hrs 42 mins",
            "total_seconds": 24120.0
        }"#;
        let gt: GrandTotal = serde_json::from_str(json).unwrap();
        assert_eq!(gt.hours, 6);
        assert_eq!(gt.minutes, 42);
        #[allow(clippy::float_cmp)]
        {
            assert_eq!(gt.total_seconds, 24120.0);
        }
        assert_eq!(gt.text, "6 hrs 42 mins");
    }

    /// Unknown fields in the JSON must be silently ignored.
    #[test]
    fn summary_entry_ignores_unknown_fields() {
        let json = r#"{
            "digital": "3:30",
            "hours": 3,
            "minutes": 30,
            "name": "Python",
            "percent": 52.3,
            "seconds": 0,
            "text": "3 hrs 30 mins",
            "total_seconds": 12600.0,
            "some_future_field": "ignored"
        }"#;
        let entry: SummaryEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.name, "Python");
        assert_eq!(entry.hours, 3);
    }

    /// `SummaryData::dependencies` must default to an empty vec when absent.
    #[test]
    fn summary_data_default_dependencies() {
        let json = r#"{
            "categories": [],
            "editors": [],
            "grand_total": {
                "digital": "0:00",
                "hours": 0,
                "minutes": 0,
                "seconds": 0,
                "text": "0 secs",
                "total_seconds": 0.0
            },
            "languages": [],
            "operating_systems": [],
            "projects": [],
            "range": {
                "end": "2025-01-14T00:00:00Z",
                "start": "2025-01-13T00:00:00Z"
            }
        }"#;
        let data: SummaryData = serde_json::from_str(json).unwrap();
        assert!(data.dependencies.is_empty());
        assert!(data.machines.is_empty());
    }

    /// A serialized `User` must round-trip through JSON without loss.
    #[test]
    fn user_roundtrip() {
        let user = User {
            id: "abc-123".to_owned(),
            username: "johndoe".to_owned(),
            display_name: "John Doe".to_owned(),
            full_name: Some("John Doe".to_owned()),
            email: None,
            photo: None,
            timezone: "UTC".to_owned(),
            website: None,
            human_readable_website: None,
            location: None,
            plan: Some("free".to_owned()),
            profile_url: Some("https://wakatime.com/@johndoe".to_owned()),
            is_email_confirmed: true,
            is_hireable: Some(false),
            logged_time_public: Some(false),
            writes_only: false,
            timeout: Some(15),
            time_format_24hr: Some(false),
            created_at: "2024-01-01T00:00:00Z".to_owned(),
            modified_at: None,
        };
        let json = serde_json::to_string(&user).unwrap();
        let user2: User = serde_json::from_str(&json).unwrap();
        assert_eq!(user.id, user2.id);
        assert_eq!(user.username, user2.username);
        assert_eq!(user.timezone, user2.timezone);
    }
}
