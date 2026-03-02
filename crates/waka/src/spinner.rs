//! Contextual progress spinners for the `waka` CLI.
//!
//! All spinners write to **stderr** and are automatically hidden when stderr
//! is not a TTY — no conditional logic needed at call sites.

use std::io::IsTerminal as _;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

/// Creates a spinner with a custom message.
///
/// The spinner writes to stderr and is hidden when stderr is not a TTY.
/// Call [`ProgressBar::finish_and_clear`] on the returned value when done.
pub fn make_spinner(message: impl Into<String>) -> ProgressBar {
    let target = if std::io::stderr().is_terminal() {
        ProgressDrawTarget::stderr()
    } else {
        ProgressDrawTarget::hidden()
    };

    let pb = ProgressBar::with_draw_target(None, target);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}")
            .expect("spinner template is valid — reviewed at compile time"),
    );
    pb.set_message(message.into());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Creates a spinner for a `waka stats` fetch.
///
/// `label` is the human-readable period, e.g. `"today"`, `"last 7 days"`.
///
/// # Example
///
/// ```
/// let pb = waka::spinner::stats_spinner("today");
/// // … fetch …
/// pb.finish_and_clear();
/// ```
// TODO(refactor): replace `commands::stats_spinner(&format!(...))` calls with
// this function once Phase 2 is merged.
#[allow(dead_code)]
pub fn stats_spinner(label: &str) -> ProgressBar {
    make_spinner(format!("Fetching {label} stats\u{2026}"))
}

/// Creates a spinner for a projects list fetch.
// TODO(phase3): used when `waka projects` fetches from the API.
#[allow(dead_code)]
pub fn projects_spinner() -> ProgressBar {
    make_spinner("Loading your projects\u{2026}")
}

/// Creates a spinner for a goals fetch.
// TODO(phase3): used when `waka goals` fetches from the API.
#[allow(dead_code)]
pub fn goals_spinner() -> ProgressBar {
    make_spinner("Fetching your goals\u{2026}")
}

/// Creates a spinner for an authentication operation.
// TODO(phase3): used when `waka auth` performs async checks.
#[allow(dead_code)]
pub fn auth_spinner(action: &str) -> ProgressBar {
    make_spinner(format!("{action}\u{2026}"))
}
