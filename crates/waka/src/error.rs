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

/// Formats an error for display to the user according to SPEC.md §10.2 format.
///
/// Output format:
/// ```text
/// Error: <title>
///
/// Reason: <reason>
///
/// Try:
///   · <action 1>
///   · <action 2>
///
/// If the problem persists: https://github.com/you/waka/issues
/// ```
#[must_use]
pub fn format_error(err: &anyhow::Error) -> String {
    const SUPPORT_URL: &str = "https://github.com/mouwaficbdr/waka/issues";

    // Try to downcast to our structured error types
    if let Some(app) = err.downcast_ref::<AppError>() {
        match app {
            AppError::Usage(msg) => {
                format!(
                    "  Error: Invalid command usage\n\n  Reason: {msg}\n\n  Try:\n    · Run `waka --help` to see available commands\n    · Check the command syntax in the help text\n\n  If the problem persists: {SUPPORT_URL}"
                )
            }
            AppError::Auth(msg) => {
                format!(
                    "  Error: Authentication failed\n\n  Reason: {msg}\n\n  Try:\n    · Run `waka login` to set up your API key\n    · Check your API key at https://wakatime.com/settings/api-key\n    · Verify your credentials with `waka config doctor`\n\n  If the problem persists: {SUPPORT_URL}"
                )
            }
            AppError::Network(msg) => {
                format!(
                    "  Error: Could not connect to WakaTime API\n\n  Reason: {msg}\n\n  Try:\n    · Check your internet connection\n    · Run `waka config doctor` for a full diagnostic\n    · Use `--no-cache` to bypass the cache\n\n  If the problem persists: {SUPPORT_URL}"
                )
            }
            AppError::Config(msg) => {
                format!(
                    "  Error: Configuration error\n\n  Reason: {msg}\n\n  Try:\n    · Check your config file at ~/.config/waka/config.toml\n    · Run `waka config doctor` to validate your configuration\n    · Delete the config file and run `waka login` to start fresh\n\n  If the problem persists: {SUPPORT_URL}"
                )
            }
            AppError::NotFound(msg) => {
                format!(
                    "  Error: Resource not found\n\n  Reason: {msg}\n\n  Try:\n    · Check the spelling of resource names\n    · Run `waka stats today` to see available data\n    · Verify you have recent coding activity on WakaTime\n\n  If the problem persists: {SUPPORT_URL}"
                )
            }
        }
    } else if let Some(cred_err) = err.downcast_ref::<waka_config::CredentialError>() {
        format!(
            "  Error: Credential storage error\n\n  Reason: {cred_err}\n\n  Try:\n    · Run `waka login` to set up your API key\n    · Check file permissions on ~/.config/waka/\n    · Verify your keyring/keychain is accessible\n\n  If the problem persists: {SUPPORT_URL}"
        )
    } else if let Some(api_err) = err.downcast_ref::<waka_api::ApiError>() {
        format!(
            "  Error: WakaTime API error\n\n  Reason: {api_err}\n\n  Try:\n    · Check your internet connection\n    · Verify your API key is valid at https://wakatime.com/settings/api-key\n    · Wait a moment and try again if rate limited\n\n  If the problem persists: {SUPPORT_URL}"
        )
    } else if let Some(cfg_err) = err.downcast_ref::<waka_config::ConfigError>() {
        format!(
            "  Error: Configuration error\n\n  Reason: {cfg_err}\n\n  Try:\n    · Check your config file at ~/.config/waka/config.toml\n    · Run `waka config doctor` to validate configuration\n    · Delete the config and start fresh with `waka login`\n\n  If the problem persists: {SUPPORT_URL}"
        )
    } else {
        // Generic error format for unknown error types
        format!(
            "  Error: Unexpected error\n\n  Reason: {err:#}\n\n  Try:\n    · Run with `--verbose` for more details\n    · Check if WakaTime services are operational\n    · Report this issue if it persists\n\n  If the problem persists: {SUPPORT_URL}"
        )
    }
}
