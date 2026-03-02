//! Output renderers for the `waka` CLI.
//!
//! Converts [`waka_api`] response types into human-readable strings.
//! Supports table, JSON, plain-text, CSV, and TSV formats.
//!
//! This crate has **no** knowledge of the network or cache layer.
//!
//! # Theming
//!
//! Build a [`Theme`] once per process using [`Theme::from_env`] and pass it to
//! all render functions.  This ensures `NO_COLOR`, pipe detection, and
//! `FORCE_COLOR` are handled uniformly.

pub mod breakdown;
pub mod error;
pub mod format;
pub mod goals;
pub mod leaderboard;
pub mod options;
pub mod summary;
pub mod theme;
pub mod utils;

// Flatten the most-used items to the crate root for ergonomic imports.
pub use breakdown::BreakdownRenderer;
pub use error::render_error;
pub use format::{format_bar, format_duration};
pub use goals::GoalRenderer;
pub use leaderboard::LeaderboardRenderer;
pub use options::{detect_output_format, should_use_color, OutputFormat, RenderOptions};
pub use summary::SummaryRenderer;
pub use theme::Theme;
