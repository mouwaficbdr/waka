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
            OutputFormat::Csv => Self::render_csv(entries, title_col, limit, opts.csv_bom),
            OutputFormat::Tsv => Self::render_tsv(entries, title_col, limit),
            OutputFormat::Plain => Self::render_plain(entries, title_col, limit),
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

    // ── csv / tsv ─────────────────────────────────────────────────────────────

    fn render_csv_or_tsv(
        entries: &[(String, f64)],
        title_col: &str,
        limit: Option<usize>,
        sep: char,
        bom: bool,
    ) -> String {
        let cap = limit.unwrap_or(DEFAULT_LIMIT);
        let total: f64 = entries.iter().map(|(_, s)| s).sum();

        let mut out = String::new();
        if bom {
            out.push('\u{FEFF}');
        }

        writeln!(
            out,
            "rank{s}{col}{s}total_seconds{s}time{s}percent",
            s = sep,
            col = title_col.to_lowercase(),
        )
        .expect("write to String is infallible");

        for (i, (name, seconds)) in entries.iter().take(cap).enumerate() {
            let pct = if total > 0.0 {
                (seconds / total) * 100.0
            } else {
                0.0
            };
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let secs_u64 = seconds.round() as u64;
            writeln!(
                out,
                "{rank}{s}{name}{s}{secs}{s}{time}{s}{pct:.1}",
                s = sep,
                rank = i + 1,
                secs = secs_u64,
                time = format_duration(secs_u64),
            )
            .expect("write to String is infallible");
        }
        out
    }

    /// Renders `entries` as CSV with an optional UTF-8 BOM.
    ///
    /// Columns: `rank, <title_col>, total_seconds, time, percent`.
    #[must_use]
    pub fn render_csv(
        entries: &[(String, f64)],
        title_col: &str,
        limit: Option<usize>,
        bom: bool,
    ) -> String {
        Self::render_csv_or_tsv(entries, title_col, limit, ',', bom)
    }

    /// Renders `entries` as TSV (tab-separated values).
    ///
    /// Columns: `rank, <title_col>, total_seconds, time, percent`.
    #[must_use]
    pub fn render_tsv(entries: &[(String, f64)], title_col: &str, limit: Option<usize>) -> String {
        Self::render_csv_or_tsv(entries, title_col, limit, '\t', false)
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

    // ── CSV / TSV tests ────────────────────────────────────────────────────────

    #[test]
    fn render_csv_has_header_row() {
        let out = BreakdownRenderer::render_csv(&entries(), "Language", None, false);
        let first = out.lines().next().expect("at least one line");
        assert_eq!(
            first, "rank,language,total_seconds,time,percent",
            "CSV header mismatch: {first}"
        );
    }

    #[test]
    fn render_csv_uses_comma_separator() {
        let out = BreakdownRenderer::render_csv(&entries(), "Language", None, false);
        for line in out.lines() {
            assert!(!line.contains('\t'), "CSV must not contain tabs");
            assert_eq!(
                line.matches(',').count(),
                4,
                "each CSV line must have 5 fields"
            );
        }
    }

    #[test]
    fn render_csv_with_bom_starts_with_bom() {
        let out = BreakdownRenderer::render_csv(&entries(), "Language", None, true);
        assert!(out.starts_with('\u{FEFF}'), "expected UTF-8 BOM");
    }

    #[test]
    fn render_csv_without_bom_has_no_bom() {
        let out = BreakdownRenderer::render_csv(&entries(), "Language", None, false);
        assert!(!out.starts_with('\u{FEFF}'), "unexpected BOM");
    }

    #[test]
    fn render_tsv_uses_tab_separator() {
        let out = BreakdownRenderer::render_tsv(&entries(), "Language", None);
        let header = out.lines().next().expect("at least one line");
        assert!(header.contains('\t'), "TSV header must use tab delimiter");
    }

    #[test]
    fn render_tsv_does_not_start_with_bom() {
        let out = BreakdownRenderer::render_tsv(&entries(), "Language", None);
        assert!(!out.starts_with('\u{FEFF}'), "TSV must never have a BOM");
    }

    #[test]
    fn snapshot_breakdown_csv() {
        let out = BreakdownRenderer::render_csv(&entries(), "Language", None, false);
        insta::assert_snapshot!(out);
    }
}
