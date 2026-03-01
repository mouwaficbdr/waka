//! Application-level error type and exit code mapping for `waka`.
//!
//! Command handlers use `anyhow::Result<()>`. When they need to signal a
//! specific exit code they return an [`AppError`], which is downcast in
//! `main()` to select the right process exit code (SPEC.md Annexe B).

use std::fmt;

/// Structured error categories for the `waka` binary.
///
/// Exit code mapping (SPEC.md Annexe B):
/// - `Usage`    → 2
/// - `Auth`     → 3
/// - `Network`  → 4
/// - `Config`   → 5
/// - `NotFound` → 6
/// - anything else → 1
// Variants are constructed in command handlers added in later phases.
#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    /// Exit code 2 — bad CLI arguments or option combination.
    Usage(String),
    /// Exit code 3 — no API key found or authentication rejected.
    Auth(String),
    /// Exit code 4 — network or API error.
    Network(String),
    /// Exit code 5 — config file missing, unreadable, or invalid.
    Config(String),
    /// Exit code 6 — requested resource does not exist.
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // All variants carry a human-readable message — forwarded directly.
        // Or-pattern merges all arms since the body is identical.
        match self {
            Self::Usage(msg)
            | Self::Auth(msg)
            | Self::Network(msg)
            | Self::Config(msg)
            | Self::NotFound(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AppError {}

/// Returns the process exit code for a given [`anyhow::Error`] (SPEC.md Annexe B).
///
/// Inspects the error chain for known types; defaults to `1`.
#[must_use]
pub fn exit_code(err: &anyhow::Error) -> i32 {
    if let Some(app) = err.downcast_ref::<AppError>() {
        return match app {
            AppError::Usage(_) => 2,
            AppError::Auth(_) => 3,
            AppError::Network(_) => 4,
            AppError::Config(_) => 5,
            AppError::NotFound(_) => 6,
        };
    }
    if err.downcast_ref::<waka_config::CredentialError>().is_some() {
        return 3;
    }
    if err.downcast_ref::<waka_api::ApiError>().is_some() {
        return 4;
    }
    if err.downcast_ref::<waka_config::ConfigError>().is_some() {
        return 5;
    }
    1
}
