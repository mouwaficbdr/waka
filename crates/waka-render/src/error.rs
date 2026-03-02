//! Premium error rendering for the `waka` CLI.
//!
//! [`render_error`] formats an error into a styled panel suitable for
//! display on stderr.  When `color` is `true` it uses Unicode box-drawing
//! characters and ANSI colour codes; when `false` (or `NO_COLOR=1`) it falls
//! back to plain ASCII text with the same structure.

use std::fmt::Write as _;

use owo_colors::OwoColorize as _;

use crate::theme::Theme;

/// Renders a rich error panel as a `String`.
///
/// # Arguments
///
/// * `title` — short error message (e.g. `"Unauthorized"`).
/// * `cause` — optional longer description of the root cause.
/// * `suggestions` — zero or more actionable hints for the user.
/// * `color` — whether to emit ANSI escape codes and Unicode box-drawing.
///
/// # Example
///
/// ```
/// use waka_render::render_error;
///
/// let msg = render_error(
///     "Unauthorized",
///     Some("API key is missing or invalid."),
///     &["Run `waka auth login` to authenticate."],
///     false,
/// );
/// assert!(msg.contains("Unauthorized"));
/// ```
#[must_use]
pub fn render_error(title: &str, cause: Option<&str>, suggestions: &[&str], color: bool) -> String {
    if color {
        render_colored(title, cause, suggestions)
    } else {
        render_plain(title, cause, suggestions)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Unicode box-drawn coloured variant.
fn render_colored(title: &str, cause: Option<&str>, suggestions: &[&str]) -> String {
    let theme = Theme::colored();

    // Width of the inner content area.
    let max_content = {
        let t = title.len() + 3; // "✖  " prefix
        let c = cause.map_or(0, |s| s.len() + 3);
        let s = suggestions.iter().map(|s| s.len() + 4).max().unwrap_or(0); // "  · " prefix
        t.max(c).max(s).max(40)
    };
    let inner = max_content + 2; // 1-space left + 1-space right padding

    let top = format!(
        "\u{256D}\u{2500} Error {}\u{256E}",
        "\u{2500}".repeat(inner - 8)
    );
    let bot = format!("\u{2570}{}\u{256F}", "\u{2500}".repeat(inner + 2));

    let mut out = String::new();
    writeln!(out, "{}", top.style(theme.error)).expect("infallible");

    // Title line: "│  ✖  Unauthorized                  │"
    let pad = inner.saturating_sub(title.len() + 4);
    writeln!(
        out,
        "{pipe}  {cross}  {title}{spaces}{pipe}",
        pipe = "\u{2502}".style(theme.error),
        cross = "\u{2716}".style(theme.error),
        title = title.style(theme.bold),
        spaces = " ".repeat(pad),
    )
    .expect("infallible");

    // Cause line (if any).
    if let Some(c) = cause {
        let pad = inner.saturating_sub(c.len() + 3);
        writeln!(
            out,
            "{pipe}  {label} {cause}{spaces}{pipe}",
            pipe = "\u{2502}".style(theme.error),
            label = "Reason:".style(theme.muted),
            cause = c,
            spaces = " ".repeat(pad),
        )
        .expect("infallible");
    }

    // Suggestions.
    if !suggestions.is_empty() {
        // Separator.
        writeln!(
            out,
            "{pipe}  {sep}{pipe}",
            pipe = "\u{2502}".style(theme.error),
            sep = "\u{2500}".repeat(inner - 1).style(theme.muted),
        )
        .expect("infallible");
        for s in suggestions {
            let pad = inner.saturating_sub(s.len() + 5);
            writeln!(
                out,
                "{pipe}  {bullet} {hint}{spaces}{pipe}",
                pipe = "\u{2502}".style(theme.error),
                bullet = "\u{00B7}".style(theme.accent),
                hint = s,
                spaces = " ".repeat(pad),
            )
            .expect("infallible");
        }
    }

    writeln!(out, "{}", bot.style(theme.error)).expect("infallible");
    out
}

/// Plain-text ASCII variant for `NO_COLOR` environments.
fn render_plain(title: &str, cause: Option<&str>, suggestions: &[&str]) -> String {
    let mut out = String::new();
    writeln!(out, "Error: {title}").expect("infallible");
    if let Some(c) = cause {
        writeln!(out, "Reason: {c}").expect("infallible");
    }
    if !suggestions.is_empty() {
        writeln!(out, "Try:").expect("infallible");
        for s in suggestions {
            writeln!(out, "  · {s}").expect("infallible");
        }
    }
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_contains_title() {
        let out = render_error("Unauthorized", None, &[], false);
        assert!(out.contains("Unauthorized"));
        assert!(out.starts_with("Error: "));
    }

    #[test]
    fn plain_contains_cause() {
        let out = render_error("Unauthorized", Some("API key invalid"), &[], false);
        assert!(out.contains("Reason: API key invalid"));
    }

    #[test]
    fn plain_contains_suggestions() {
        let out = render_error(
            "Unauthorized",
            None,
            &["Run `waka auth login`", "Check your config"],
            false,
        );
        assert!(out.contains("Run `waka auth login`"));
        assert!(out.contains("Check your config"));
        assert!(out.contains("Try:"));
    }

    #[test]
    fn plain_no_suggestions_no_try() {
        let out = render_error("Some error", None, &[], false);
        assert!(!out.contains("Try:"), "no suggestions → no Try: line");
    }

    #[test]
    fn plain_no_box_chars() {
        let out = render_error("Err", Some("cause"), &["hint"], false);
        assert!(!out.contains('╭'), "plain must not use box-drawing");
        assert!(!out.contains('│'), "plain must not use box-drawing");
    }

    #[test]
    fn colored_contains_box_chars() {
        let out = render_error("Err", Some("cause"), &["hint"], true);
        assert!(out.contains('╭'), "colored should use box-drawing top-left");
        assert!(
            out.contains('╯'),
            "colored should use box-drawing bottom-right"
        );
    }

    #[test]
    fn colored_contains_title() {
        let out = render_error("Unauthorized", None, &[], true);
        assert!(out.contains("Unauthorized"));
    }

    #[test]
    fn snapshot_plain_full() {
        let out = render_error(
            "Unauthorized",
            Some("The API key is missing or expired."),
            &[
                "Run `waka auth login` to authenticate.",
                "Check that WAKATIME_API_KEY is set correctly.",
            ],
            false,
        );
        insta::assert_snapshot!(out);
    }

    #[test]
    fn snapshot_colored_full() {
        // Strip ANSI codes for a stable snapshot across terminals.
        let out = render_error(
            "Unauthorized",
            Some("The API key is missing or expired."),
            &[
                "Run `waka auth login` to authenticate.",
                "Check that WAKATIME_API_KEY is set correctly.",
            ],
            true,
        );
        // Strip ANSI escape sequences.
        let stripped = strip_ansi(&out);
        insta::assert_snapshot!(stripped);
    }

    /// Minimal ANSI stripping — removes `ESC[...m` sequences.
    fn strip_ansi(s: &str) -> String {
        let mut out = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\x1B' {
                // consume until 'm'
                for ch in chars.by_ref() {
                    if ch == 'm' {
                        break;
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }
}
