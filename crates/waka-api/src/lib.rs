//! `WakaTime` HTTP client library.
//!
//! This crate provides an async HTTP client for the
//! [`WakaTime` API](https://wakatime.com/developers).
//! It is designed to be usable independently of the `waka` CLI.
//!
//! # Example
//!
//! ```rust,no_run
//! // async fn example() {
//! //     // Client construction and endpoint calls are added in tasks 0.4/0.5.
//! // }
//! ```

pub mod error;
pub mod types;

pub use error::ApiError;
pub use types::{
    BestDay, Goal, GoalChartEntry, GoalChartRange, GoalsResponse, GrandTotal, LeaderboardEntry,
    LeaderboardRange, LeaderboardResponse, LeaderboardUser, MachineEntry, Project,
    ProjectsResponse, RunningTotal, Stats, StatsResponse, SummaryData, SummaryEntry, SummaryRange,
    SummaryResponse, User, UserResponse,
};
