//! Primitive formatting helpers used by all renderers.

// ─────────────────────────────────────────────────────────────────────────────
// format_duration
// ─────────────────────────────────────────────────────────────────────────────

/// Formats a duration given in **seconds** into a compact human-readable
/// string using the `Xh Ym` pattern used by the `waka` CLI.
///
/// Only hours and minutes are displayed; sub-minute precision is truncated.
///
/// | Input (seconds) | Output    |
/// |-----------------|-----------|
/// | 0               | `"0m"`    |
/// | 45              | `"0m"`    |
/// | 2 700           | `"45m"`   |
/// | 3 600           | `"1h 0m"` |
/// | 24 120          | `"6h 42m"`|
///
/// # Example
/// ```rust
/// use waka_render::format_duration;
///
/// assert_eq!(format_duration(0),     "0m");
/// assert_eq!(format_duration(2880),  "48m");
/// assert_eq!(format_duration(24120), "6h 42m");
/// ```
#[must_use]
pub fn format_duration(seconds: u64) -> String {
    let h = seconds / 3_600;
    let m = (seconds % 3_600) / 60;

    match (h, m) {
        (0, 0) => "0m".to_owned(),
        (0, m) => format!("{m}m"),
        (h, m) => format!("{h}h {m}m"),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// format_bar
// ─────────────────────────────────────────────────────────────────────────────

/// Renders a Unicode block-character progress bar.
///
/// `ratio` is clamped to `[0.0, 1.0]`. `width` is the total number of
/// characters in the output string.  Filled cells use `█` (U+2588) and empty
/// cells use `░` (U+2591).
///
/// | ratio | width | Output              |
/// |-------|-------|---------------------|
/// | 0.0   | 5     | `"░░░░░"`           |
/// | 1.0   | 5     | `"█████"`           |
/// | 0.5   | 10    | `"█████░░░░░"`      |
/// | 0.523 | 20    | `"██████████░░░░░░░░░░"` |
///
/// # Example
/// ```rust
/// use waka_render::format_bar;
///
/// assert_eq!(format_bar(0.0, 5),  "░░░░░");
/// assert_eq!(format_bar(1.0, 5),  "█████");
/// assert_eq!(format_bar(0.5, 10), "█████░░░░░");
/// ```
#[must_use]
pub fn format_bar(ratio: f64, width: u8) -> String {
    let ratio = ratio.clamp(0.0, 1.0);
    // Round to nearest cell so bars look natural at any width.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let filled = (ratio * f64::from(width)).round() as usize;
    let empty = usize::from(width) - filled;

    let mut bar = String::with_capacity(filled + empty);
    for _ in 0..filled {
        bar.push('█');
    }
    for _ in 0..empty {
        bar.push('░');
    }
    bar
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── format_duration ──────────────────────────────────────────────────────

    #[test]
    fn duration_zero_seconds() {
        assert_eq!(format_duration(0), "0m");
    }

    #[test]
    fn duration_only_minutes() {
        assert_eq!(format_duration(2_880), "48m"); // 48 × 60
        assert_eq!(format_duration(2_700), "45m"); // 45 × 60
        assert_eq!(format_duration(3_540), "59m"); // 59 × 60
    }

    #[test]
    fn duration_sub_minute_rounds_to_zero() {
        // Durations under 60 s have no minute component — display as "0m".
        assert_eq!(format_duration(45), "0m");
        assert_eq!(format_duration(59), "0m");
    }

    #[test]
    fn duration_only_hours_with_zero_minutes() {
        assert_eq!(format_duration(3_600), "1h 0m");
        assert_eq!(format_duration(7_200), "2h 0m");
    }

    #[test]
    fn duration_hours_and_minutes() {
        assert_eq!(format_duration(24_120), "6h 42m"); // fixture value
        assert_eq!(format_duration(9_900), "2h 45m");
        assert_eq!(format_duration(3_660), "1h 1m");
    }

    #[test]
    fn duration_just_under_one_hour() {
        assert_eq!(format_duration(3_599), "59m");
    }

    // ── format_bar ───────────────────────────────────────────────────────────

    #[test]
    fn bar_empty() {
        assert_eq!(format_bar(0.0, 5), "░░░░░");
    }

    #[test]
    fn bar_full() {
        assert_eq!(format_bar(1.0, 5), "█████");
    }

    #[test]
    fn bar_half() {
        assert_eq!(format_bar(0.5, 10), "█████░░░░░");
    }

    #[test]
    fn bar_clamps_above_one() {
        assert_eq!(format_bar(1.5, 4), "████");
    }

    #[test]
    fn bar_clamps_below_zero() {
        assert_eq!(format_bar(-0.5, 4), "░░░░");
    }

    #[test]
    fn bar_width_zero() {
        assert_eq!(format_bar(0.5, 0), "");
    }

    #[test]
    fn bar_fixture_rust_language() {
        // Rust: 52.3% in a 20-char bar → round(0.523 * 20) = round(10.46) = 10
        assert_eq!(format_bar(0.523, 20), "██████████░░░░░░░░░░");
    }
}
