//! Shared rendering utilities for `waka-render`.
//!
//! All functions in this module are **pure** — they accept values and return
//! `String`s.  They never write to stdout or stderr.
//!
//! # Example
//!
//! ```rust
//! use waka_render::{Theme, utils};
//!
//! let theme = Theme::plain();
//! assert_eq!(utils::progress_bar(0.5, 10, &theme), "#####-----");
//! assert_eq!(utils::humanize_duration(3600 + 900), "1h 15m");
//! ```

use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use unicode_width::UnicodeWidthStr as _;

use crate::theme::Theme;

// ─────────────────────────────────────────────────────────────────────────────
// Terminal geometry
// ─────────────────────────────────────────────────────────────────────────────

/// Return the current terminal width in columns.
///
/// Resolution order:
/// 1. `COLUMNS` environment variable (for testing / scripting overrides)
/// 2. Falls back to **80** for piped / non-TTY contexts
///
/// # Note
///
/// `comfy-table` handles its own column sizing when rendering tables.
/// This function is used only for manual layout calculations (e.g. progress
/// bars, box widths).
pub fn terminal_width() -> usize {
    if let Ok(val) = std::env::var("COLUMNS") {
        if let Ok(n) = val.trim().parse::<usize>() {
            return n;
        }
    }
    80
}

// ─────────────────────────────────────────────────────────────────────────────
// String helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Truncate `s` to at most `max_width` **display** columns.
///
/// Uses [`unicode_width`] for correct handling of multi-byte, wide, and
/// zero-width characters.  A `…` suffix is appended when the string is
/// actually truncated.
///
/// Returns an empty string when `max_width == 0`.
pub fn truncate_str(s: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }
    if s.width() <= max_width {
        return s.to_owned();
    }
    // Reserve 1 column for the ellipsis character.
    let target = max_width.saturating_sub(1);
    let mut out = String::new();
    let mut cols: usize = 0;
    for ch in s.chars() {
        let ch_w = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if cols + ch_w > target {
            break;
        }
        out.push(ch);
        cols += ch_w;
    }
    out.push('…');
    out
}

/// Pad `s` on the right with spaces to exactly `width` **display** columns.
///
/// If `s` is already at least `width` columns wide, it is returned unchanged.
/// Uses [`unicode_width`] for correct multi-byte handling.
pub fn pad_str(s: &str, width: usize) -> String {
    let current = s.width();
    if current >= width {
        return s.to_owned();
    }
    let mut out = s.to_owned();
    for _ in 0..width.saturating_sub(current) {
        out.push(' ');
    }
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Progress bars
// ─────────────────────────────────────────────────────────────────────────────

/// Render a progress bar of exactly `width` display columns.
///
/// `pct` is clamped to `[0.0, 1.0]` before rendering.  The fill/empty
/// characters are taken from the theme so that `NO_COLOR` environments
/// automatically receive ASCII fall-backs.
///
/// Returns an empty string when `width == 0`.
///
/// # Example
///
/// ```rust
/// use waka_render::{Theme, utils};
///
/// let theme = Theme::plain();
/// assert_eq!(utils::progress_bar(0.5, 10, &theme), "#####-----");
/// assert_eq!(utils::progress_bar(1.0, 4, &theme),  "####");
/// assert_eq!(utils::progress_bar(0.0, 4, &theme),  "----");
/// ```
pub fn progress_bar(pct: f64, width: usize, theme: &Theme) -> String {
    if width == 0 {
        return String::new();
    }
    let pct = pct.clamp(0.0, 1.0);
    // `width` is at most 200 on a real terminal — precision loss is acceptable.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    let filled = (pct * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    let (fill_char, empty_char) = theme.bar_chars();
    fill_char.repeat(filled) + &empty_char.repeat(empty)
}

// ─────────────────────────────────────────────────────────────────────────────
// Duration formatting
// ─────────────────────────────────────────────────────────────────────────────

/// Format a duration in seconds as a compact human-readable string.
///
/// Examples:
///
/// | Input (seconds)         | Output      |
/// |-------------------------|-------------|
/// | `0`                     | `< 1m`      |
/// | `45 * 60`               | `45m`        |
/// | `3600`                  | `1h`         |
/// | `3600 + 900`            | `1h 15m`    |
/// | `7200`                  | `2h`         |
///
/// Rounds down to the nearest minute (matching the online dashboard behaviour).
pub fn humanize_duration(total_secs: u64) -> String {
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    match (hours, mins) {
        (0, 0) => "< 1m".to_owned(),
        (0, m) => format!("{m}m"),
        (h, 0) => format!("{h}h"),
        (h, m) => format!("{h}h {m}m"),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Relative timestamps
// ─────────────────────────────────────────────────────────────────────────────

/// Format a UTC timestamp as a relative human-readable string.
///
/// Examples: `"just now"`, `"2 hours ago"`, `"yesterday"`, `"3 days ago"`.
///
/// Uses [`chrono_humanize`] for locale-neutral English output.
pub fn humanize_relative(dt: &DateTime<Utc>) -> String {
    let duration = Utc::now().signed_duration_since(*dt);
    HumanTime::from(duration).to_string()
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // --- humanize_duration ---------------------------------------------------

    #[test]
    fn humanize_duration_zero() {
        assert_eq!(humanize_duration(0), "< 1m");
    }

    #[test]
    fn humanize_duration_minutes_only() {
        assert_eq!(humanize_duration(45 * 60), "45m");
    }

    #[test]
    fn humanize_duration_hours_and_minutes() {
        assert_eq!(humanize_duration(3600 + 900), "1h 15m");
    }

    #[test]
    fn humanize_duration_hours_only() {
        assert_eq!(humanize_duration(7200), "2h");
    }

    #[test]
    fn humanize_duration_one_minute() {
        assert_eq!(humanize_duration(60), "1m");
    }

    #[test]
    fn humanize_duration_rounding() {
        // 1h 59m 59s → rounds down to 1h 59m
        assert_eq!(humanize_duration(3600 + 59 * 60 + 59), "1h 59m");
    }

    // --- progress_bar --------------------------------------------------------

    #[test]
    fn progress_bar_plain_half() {
        let bar = progress_bar(0.5, 10, &Theme::plain());
        assert_eq!(bar, "#####-----");
    }

    #[test]
    fn progress_bar_full() {
        let bar = progress_bar(1.0, 22, &Theme::plain());
        assert_eq!(bar, "######################");
    }

    #[test]
    fn progress_bar_empty() {
        let bar = progress_bar(0.0, 22, &Theme::plain());
        assert_eq!(bar, "----------------------");
    }

    #[test]
    fn progress_bar_clamped_above_one() {
        let bar = progress_bar(2.0, 10, &Theme::plain());
        assert_eq!(bar, "##########");
    }

    #[test]
    fn progress_bar_clamped_below_zero() {
        let bar = progress_bar(-1.0, 10, &Theme::plain());
        assert_eq!(bar, "----------");
    }

    #[test]
    fn progress_bar_zero_width() {
        let bar = progress_bar(0.5, 0, &Theme::plain());
        assert_eq!(bar, "");
    }

    #[test]
    fn progress_bar_colored_uses_unicode() {
        let bar = progress_bar(0.5, 4, &Theme::colored());
        assert_eq!(bar, "██░░");
    }

    // --- pad_str -------------------------------------------------------------

    #[test]
    fn pad_str_pads_short_string() {
        assert_eq!(pad_str("hi", 5), "hi   ");
    }

    #[test]
    fn pad_str_no_op_on_equal_width() {
        assert_eq!(pad_str("hello", 5), "hello");
    }

    #[test]
    fn pad_str_no_op_on_wider_string() {
        assert_eq!(pad_str("hello world", 5), "hello world");
    }

    // --- truncate_str --------------------------------------------------------

    #[test]
    fn truncate_str_no_op_short() {
        assert_eq!(truncate_str("hello", 10), "hello");
    }

    #[test]
    fn truncate_str_no_op_exact_width() {
        assert_eq!(truncate_str("hello", 5), "hello");
    }

    #[test]
    fn truncate_str_truncates() {
        // "hello world" = 11 cols → truncate to 7 → "hello " + "…" = "hello …"
        let result = truncate_str("hello world", 7);
        assert_eq!(result, "hello …");
    }

    #[test]
    fn truncate_str_zero_width() {
        assert_eq!(truncate_str("hello", 0), "");
    }

    #[test]
    fn truncate_str_one_width() {
        // Only room for the ellipsis itself (target = 0)
        let result = truncate_str("hello", 1);
        assert_eq!(result, "…");
    }

    // --- terminal_width ------------------------------------------------------

    #[test]
    fn terminal_width_fallback_is_reasonable() {
        // In test context, COLUMNS is normally unset → fallback 80.
        // Just ensure it doesn't panic and returns a usable value.
        let w = terminal_width();
        assert!(w >= 40, "terminal_width returned {w}, expected >= 40");
    }
}
