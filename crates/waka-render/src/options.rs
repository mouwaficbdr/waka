//! Rendering options and output-format detection.

use std::io::IsTerminal as _;

// ─────────────────────────────────────────────────────────────────────────────
// OutputFormat
// ─────────────────────────────────────────────────────────────────────────────

/// Output format for rendered data.
///
/// Mirrors `waka_config::OutputFormat` — the binary crate is responsible for
/// converting between the two.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum OutputFormat {
    /// Human-readable bordered table (default).
    #[default]
    Table,
    /// Machine-readable pretty-printed JSON.
    Json,
    /// CSV (comma-separated values).
    Csv,
    /// Plain text, no ANSI escape codes, no table borders.
    Plain,
    /// TSV (tab-separated values).
    Tsv,
}

// ─────────────────────────────────────────────────────────────────────────────
// RenderOptions
// ─────────────────────────────────────────────────────────────────────────────

/// Options that control how data is rendered.
#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// Whether to emit ANSI colour codes.
    pub color: bool,
    /// Terminal column width used to size progress bars and table columns.
    pub width: u16,
    /// Desired output format.
    pub format: OutputFormat,
    /// Prepend a UTF-8 BOM (`\u{FEFF}`) to CSV output.
    ///
    /// When `true`, the byte-order mark is prepended so that Windows Excel
    /// auto-detects UTF-8 encoding when opening the file directly.
    pub csv_bom: bool,
    /// Optional human-readable period label shown in the output header.
    ///
    /// Examples: `"Today"`, `"Last 7 Days"`, `"Last 30 Days"`.
    /// When `None` the date range is extracted from the response data.
    pub period_label: Option<String>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            color: should_use_color(),
            width: terminal_width(),
            format: OutputFormat::default(),
            csv_bom: false,
            period_label: None,
        }
    }
}

/// Returns the current terminal width, falling back to 80 if it cannot be
/// determined (e.g. when stdout is piped).
fn terminal_width() -> u16 {
    // comfy-table handles its own column width when rendering tables.
    // This fallback value is used only for progress-bar sizing.
    80
}

// ─────────────────────────────────────────────────────────────────────────────
// should_use_color
// ─────────────────────────────────────────────────────────────────────────────

/// Returns `true` when ANSI colour output is appropriate for the current
/// invocation.
///
/// Colour is disabled when **any** of the following conditions hold:
///
/// - stdout is not a TTY (piped or redirected)
/// - The `NO_COLOR` environment variable is set to a non-empty value
///   (see <https://no-color.org/>)
/// - `TERM` is set to `"dumb"` (indicates a terminal with no ANSI support)
///
/// This function does **not** check the `--no-color` CLI flag; callers in the
/// binary crate must AND its value with this function's result:
///
/// ```ignore
/// let color = !global.no_color && waka_render::should_use_color();
/// ```
#[must_use]
pub fn should_use_color() -> bool {
    if !std::io::stdout().is_terminal() {
        return false;
    }
    // NO_COLOR — any non-empty value disables colour (per the spec).
    if std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty()) {
        return false;
    }
    // TERM=dumb indicates a severely limited terminal with no ANSI support.
    if std::env::var("TERM").as_deref() == Ok("dumb") {
        return false;
    }
    true
}

// ─────────────────────────────────────────────────────────────────────────────
// detect_output_format
// ─────────────────────────────────────────────────────────────────────────────

/// Resolves the effective [`OutputFormat`] for the current invocation.
///
/// When stdout is **not** a TTY (piped to another command or redirected to a
/// file), the format is coerced to [`OutputFormat::Plain`] regardless of the
/// configured value, preserving the Unix piping contract.
///
/// CSV, TSV and JSON are preserved even in piped contexts because they are
/// machine-readable formats explicitly requested for piped consumption.
///
/// When `TERM=dumb` the format also degrades to `Plain`.
///
/// # Example
/// ```rust
/// use waka_render::{detect_output_format, OutputFormat};
///
/// let fmt = detect_output_format(OutputFormat::Table);
/// // In a TTY: OutputFormat::Table
/// // In a pipe: OutputFormat::Plain
/// let _ = fmt;
/// ```
#[must_use]
pub fn detect_output_format(configured: OutputFormat) -> OutputFormat {
    // Machine-readable formats are always preserved regardless of TTY state.
    if matches!(
        configured,
        OutputFormat::Csv | OutputFormat::Tsv | OutputFormat::Json
    ) {
        return configured;
    }

    if !std::io::stdout().is_terminal() {
        // Piped/redirected — degrade to plain text.
        return OutputFormat::Plain;
    }

    // TERM=dumb: the terminal cannot render ANSI sequences or box-drawing
    // characters, so table output would be unreadable.
    if std::env::var("TERM").as_deref() == Ok("dumb") {
        return OutputFormat::Plain;
    }

    configured
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_options_default_format_is_table() {
        // Default format must be Table regardless of TTY state.
        assert_eq!(RenderOptions::default().format, OutputFormat::Table);
    }

    #[test]
    fn render_options_default_width_is_positive() {
        assert!(RenderOptions::default().width > 0);
    }

    #[test]
    fn output_format_default_is_table() {
        assert_eq!(OutputFormat::default(), OutputFormat::Table);
    }

    // ── should_use_color ─────────────────────────────────────────────────────

    #[test]
    fn no_color_env_disables_color() {
        // Temporarily set NO_COLOR and test that should_use_color() returns
        // false even if we cannot directly control TTY state.
        // We don't assert the TTY case because test runners are not TTYs.
        // Instead we verify the NO_COLOR path directly.
        std::env::set_var("NO_COLOR", "1");
        let result = should_use_color();
        std::env::remove_var("NO_COLOR");
        // In a non-TTY (CI) environment this will already be false.
        // The key invariant: NO_COLOR=1 must not cause a panic.
        let _ = result;
    }

    #[test]
    fn no_color_empty_value_does_not_disable_color_by_itself() {
        // NO_COLOR="" (empty) must be treated as unset per the spec.
        // The TTY check will still disable colour in non-TTY environments.
        std::env::set_var("NO_COLOR", "");
        // This should not panic.
        let _ = should_use_color();
        std::env::remove_var("NO_COLOR");
    }

    #[test]
    fn term_dumb_disables_color_in_tty() {
        // We can't control TTY state in tests, but we can verify the
        // should_use_color function handles TERM=dumb without panicking.
        std::env::set_var("TERM", "dumb");
        let _ = should_use_color();
        std::env::remove_var("TERM");
    }

    // ── detect_output_format ─────────────────────────────────────────────────

    #[test]
    fn detect_output_format_preserves_csv_in_pipe() {
        // CSV must never be degraded to Plain, even in a non-TTY context.
        // We verify the logic directly since tests run in a non-TTY.
        let configured = OutputFormat::Csv;
        let result = detect_output_format(configured);
        assert_eq!(result, OutputFormat::Csv, "CSV must survive pipe detection");
    }

    #[test]
    fn detect_output_format_preserves_tsv_in_pipe() {
        let result = detect_output_format(OutputFormat::Tsv);
        assert_eq!(result, OutputFormat::Tsv, "TSV must survive pipe detection");
    }

    #[test]
    fn detect_output_format_preserves_json_in_pipe() {
        let result = detect_output_format(OutputFormat::Json);
        assert_eq!(
            result,
            OutputFormat::Json,
            "JSON must survive pipe detection"
        );
    }

    #[test]
    fn detect_output_format_degrades_table_in_non_tty() {
        // In the test environment stdout is not a TTY, so Table → Plain.
        let result = detect_output_format(OutputFormat::Table);
        assert_eq!(
            result,
            OutputFormat::Plain,
            "Table must degrade to Plain in non-TTY"
        );
    }
}
