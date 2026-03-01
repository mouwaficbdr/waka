//! Generic breakdown renderer for ranked lists of `(name, total_seconds)` pairs.
//!
//! Used by `waka projects top`, `waka languages list/top`, and `waka editors
//! list/top` to display a ranked table of coding activity.

use std::fmt::Write as _;

use comfy_table::{presets, Cell, ContentArrangement, Table};
use serde::Serialize;

use crate::format::{format_bar, format_duration};
use crate::options::{OutputFormat, RenderOptions};

/// Width (in Unicode characters) of the ASCII progress bar in the table.
const BAR_WIDTH: u8 = 20;
/// Maximum number of entries rendered when no explicit limit is supplied.
const DEFAULT_LIMIT: usize = 20;

// ─────────────────────────────────────────────────────────────────────────────

/// A single entry in the rendered breakdown output.
///
/// Used as the serialised element when rendering as JSON.
#[derive(Debug, Clone, Serialize)]
pub struct BreakdownEntry {
    /// Rank of this entry (1 = highest time).
    pub rank: usize,
    /// Entity name (language, project, editor, …).
    pub name: String,
    /// Total coding seconds during the period.
    pub total_seconds: f64,
    /// Human-readable duration (e.g. `"3 hrs 12 mins"`).
    pub text: String,
    /// Percentage of total time (0.0–100.0).
    pub percent: f64,
}

// ─────────────────────────────────────────────────────────────────────────────

/// Zero-size renderer for ranked entry lists.
///
/// All methods are pure: they take references and return an owned [`String`].
pub struct BreakdownRenderer;

impl BreakdownRenderer {
    /// Renders `entries` in the format specified by `opts`.
    ///
    /// `entries` must be `(name, total_seconds)` pairs **already sorted** by
    /// the caller (highest first). `title_col` is the column header / label
    /// used in table and plain output (e.g. `"Language"`, `"Project"`).
    /// `limit` restricts the number of rows rendered; pass `None` to use the
    /// default cap of 20.
    ///
    /// # Panics
    ///
    /// Does not panic.
    #[must_use]
    pub fn render(
        entries: &[(String, f64)],
        title_col: &str,
        limit: Option<usize>,
        opts: &RenderOptions,
    ) -> String {
        match opts.format {
            OutputFormat::Table => Self::render_table(entries, title_col, limit, opts),
            OutputFormat::Json => Self::render_json(entries, limit),
            OutputFormat::Plain | OutputFormat::Csv | OutputFormat::Tsv => {
                Self::render_plain(entries, title_col, limit)
            }
        }
    }

    // ── table ─────────────────────────────────────────────────────────────────

    fn render_table(
        entries: &[(String, f64)],
        title_col: &str,
        limit: Option<usize>,
        _opts: &RenderOptions,
    ) -> String {
        let cap = limit.unwrap_or(DEFAULT_LIMIT);
        let shown: Vec<_> = entries.iter().take(cap).collect();
        let total: f64 = shown.iter().map(|(_, s)| s).sum();

        let mut table = Table::new();
        table
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Disabled)
            .set_header(vec![
                Cell::new("#"),
                Cell::new(title_col),
                Cell::new("Time"),
                Cell::new("Bar"),
                Cell::new("%"),
            ]);

        for (rank, (name, seconds)) in shown.iter().enumerate() {
            let pct = if total > 0.0 {
                (seconds / total) * 100.0
            } else {
                0.0
            };
            let bar = format_bar(pct, BAR_WIDTH);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let secs_u64 = seconds.round() as u64;
            table.add_row(vec![
                Cell::new(rank + 1),
                Cell::new(name.as_str()),
                Cell::new(format_duration(secs_u64)),
                Cell::new(bar),
                Cell::new(format!("{pct:.1}%")),
            ]);
        }

        format!("{table}\n")
    }

    // ── json ──────────────────────────────────────────────────────────────────

    fn render_json(entries: &[(String, f64)], limit: Option<usize>) -> String {
        let cap = limit.unwrap_or(DEFAULT_LIMIT);
        let total: f64 = entries.iter().map(|(_, s)| s).sum();

        let items: Vec<BreakdownEntry> = entries
            .iter()
            .take(cap)
            .enumerate()
            .map(|(i, (name, seconds))| {
                let pct = if total > 0.0 {
                    (seconds / total) * 100.0
                } else {
                    0.0
                };
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let secs_u64 = seconds.round() as u64;
                BreakdownEntry {
                    rank: i + 1,
                    name: name.clone(),
                    total_seconds: *seconds,
                    text: format_duration(secs_u64),
                    percent: (pct * 10.0).round() / 10.0,
                }
            })
            .collect();

        serde_json::to_string_pretty(&items).unwrap_or_else(|_| "[]".to_owned())
    }

    // ── plain ─────────────────────────────────────────────────────────────────

    fn render_plain(entries: &[(String, f64)], title_col: &str, limit: Option<usize>) -> String {
        let cap = limit.unwrap_or(DEFAULT_LIMIT);
        let shown: Vec<_> = entries.iter().take(cap).collect();
        let total: f64 = entries.iter().map(|(_, s)| s).sum();

        let mut out = format!("{title_col}\t Time\t %\n");
        for (name, seconds) in &shown {
            let pct = if total > 0.0 {
                (seconds / total) * 100.0
            } else {
                0.0
            };
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let secs_u64 = seconds.round() as u64;
            // `writeln!` to a `String` is infallible.
            writeln!(out, "{name}\t {}\t {pct:.1}%", format_duration(secs_u64))
                .expect("write to String is infallible");
        }
        out
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn entries() -> Vec<(String, f64)> {
        vec![
            ("Rust".to_owned(), 10800.0),      // 3h
            ("Python".to_owned(), 7200.0),     // 2h
            ("TypeScript".to_owned(), 3600.0), // 1h
        ]
    }

    fn opts() -> RenderOptions {
        RenderOptions {
            color: false,
            format: OutputFormat::Table,
            ..RenderOptions::default()
        }
    }

    #[test]
    fn render_table_contains_entry_name() {
        let out = BreakdownRenderer::render(&entries(), "Language", None, &opts());
        assert!(out.contains("Rust"), "expected 'Rust' in table output");
    }

    #[test]
    fn render_table_contains_rank_column() {
        let out = BreakdownRenderer::render(&entries(), "Language", None, &opts());
        assert!(out.contains('#'), "expected '#' rank column header");
    }

    #[test]
    fn render_table_limit_restricts_rows() {
        let out = BreakdownRenderer::render(&entries(), "Language", Some(1), &opts());
        assert!(out.contains("Rust"));
        assert!(!out.contains("Python"), "Python should be cut by limit=1");
    }

    #[test]
    fn render_json_is_valid_json_array() {
        let opts = RenderOptions {
            format: OutputFormat::Json,
            ..RenderOptions::default()
        };
        let out = BreakdownRenderer::render(&entries(), "Language", None, &opts);
        let v: serde_json::Value = serde_json::from_str(&out).expect("should be valid JSON");
        assert!(v.is_array());
        assert_eq!(v.as_array().unwrap().len(), 3);
    }

    #[test]
    fn render_json_contains_total_seconds() {
        let opts = RenderOptions {
            format: OutputFormat::Json,
            ..RenderOptions::default()
        };
        let out = BreakdownRenderer::render(&entries(), "Language", None, &opts);
        assert!(
            out.contains("total_seconds"),
            "JSON output must include total_seconds"
        );
    }

    #[test]
    fn render_plain_contains_entry_name() {
        let opts = RenderOptions {
            format: OutputFormat::Plain,
            ..RenderOptions::default()
        };
        let out = BreakdownRenderer::render(&entries(), "Language", None, &opts);
        assert!(out.contains("Rust"));
        assert!(out.contains("Python"));
    }

    #[test]
    fn render_table_zero_entries_does_not_panic() {
        let out = BreakdownRenderer::render(&[], "Language", None, &opts());
        // Just check it doesn't panic
        let _ = out;
    }

    // ── snapshot tests ─────────────────────────────────────────────────────────

    #[test]
    fn snapshot_breakdown_table() {
        let out = BreakdownRenderer::render(&entries(), "Language", None, &opts());
        insta::assert_snapshot!(out);
    }

    #[test]
    fn snapshot_breakdown_json() {
        let opts = RenderOptions {
            format: OutputFormat::Json,
            ..RenderOptions::default()
        };
        let out = BreakdownRenderer::render(&entries(), "Language", None, &opts);
        insta::assert_snapshot!(out);
    }

    #[test]
    fn snapshot_breakdown_plain() {
        let opts = RenderOptions {
            format: OutputFormat::Plain,
            ..RenderOptions::default()
        };
        let out = BreakdownRenderer::render(&entries(), "Language", None, &opts);
        insta::assert_snapshot!(out);
    }
}
