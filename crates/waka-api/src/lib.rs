//! `WakaTime` HTTP client library.
//!
//! This crate provides an async HTTP client for the
//! [`WakaTime` API](https://wakatime.com/developers).
//! It is designed to be usable independently of the `waka` CLI.
//!
//! # Example
//!
//! ```rust,no_run
//! use waka_api::WakaClient;
//!
//! async fn example() -> Result<(), waka_api::ApiError> {
//!     let client = WakaClient::new("waka_xxxxxx");
//!     let me = client.me().await?;
//!     println!("Hello {}", me.username);
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod params;
pub mod types;

pub use client::WakaClient;
pub use error::ApiError;
pub use params::SummaryParams;
pub use types::{
    BestDay, Goal, GoalChartEntry, GoalChartRange, GoalsResponse, GrandTotal, LeaderboardEntry,
    LeaderboardRange, LeaderboardResponse, LeaderboardUser, MachineEntry, Project,
    ProjectsResponse, RunningTotal, Stats, StatsResponse, SummaryData, SummaryEntry, SummaryRange,
    SummaryResponse, User, UserResponse,
};
