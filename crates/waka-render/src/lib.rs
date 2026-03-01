//! Output renderers for the `waka` CLI.
//!
//! Converts [`waka_api`] response types into human-readable strings.
//! Supports table, JSON, plain-text, CSV, and TSV formats.
//!
//! This crate has **no** knowledge of the network or cache layer.

pub mod breakdown;
pub mod format;
pub mod options;
pub mod summary;

// Flatten the most-used items to the crate root for ergonomic imports.
pub use breakdown::BreakdownRenderer;
pub use format::{format_bar, format_duration};
pub use options::{detect_output_format, OutputFormat, RenderOptions};
pub use summary::SummaryRenderer;
