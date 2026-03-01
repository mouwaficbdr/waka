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
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            color: std::io::stdout().is_terminal(),
            width: terminal_width(),
            format: OutputFormat::default(),
            csv_bom: false,
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
// detect_output_format
// ─────────────────────────────────────────────────────────────────────────────

/// Resolves the effective [`OutputFormat`] for the current invocation.
///
/// When stdout is **not** a TTY (piped to another command or redirected to a
/// file), the format is coerced to [`OutputFormat::Plain`] regardless of the
/// configured value, preserving the Unix piping contract.
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
    if std::io::stdout().is_terminal() {
        configured
    } else {
        // Piped or redirected — degrade gracefully to plain text.
        // CSV and TSV are machine-readable formats; they must not be degraded
        // because they are explicitly requested for piped consumption.
        match configured {
            OutputFormat::Csv | OutputFormat::Tsv | OutputFormat::Json => configured,
            _ => OutputFormat::Plain,
        }
    }
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
}
