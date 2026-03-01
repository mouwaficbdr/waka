//! Renderers for `WakaTime` summary API responses.
//!
//! The [`SummaryRenderer`] struct is a zero-size type whose methods convert a
//! [`waka_api::SummaryResponse`] into a `String` in one of several formats.

use std::fmt::Write as _;

use comfy_table::{presets, Cell, ContentArrangement, Table};
use waka_api::{SummaryEntry, SummaryResponse};

use crate::format::{format_bar, format_duration};
use crate::options::{OutputFormat, RenderOptions};

/// Width (in Unicode characters) of the ASCII progress bar in the table.
const BAR_WIDTH: u8 = 20;

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Aggregates a list of per-day [`SummaryEntry`] slices into a single sorted
/// `(name, total_seconds)` list, highest seconds first.
///
/// When `data` spans multiple days (week/range query), time for the same
/// entity (language, project, …) is summed across all days.
fn aggregate(
    entries_per_day: impl Iterator<Item = impl IntoIterator<Item = SummaryEntry>>,
) -> Vec<(String, f64)> {
    let mut map: Vec<(String, f64)> = Vec::new();

    for day_entries in entries_per_day {
        for entry in day_entries {
            if let Some(existing) = map.iter_mut().find(|(n, _)| n == &entry.name) {
                existing.1 += entry.total_seconds;
            } else {
                map.push((entry.name, entry.total_seconds));
            }
        }
    }

    map.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    map
}

/// Returns the grand total of seconds across all days in a summary response.
fn grand_total_seconds(resp: &SummaryResponse) -> f64 {
    resp.data.iter().map(|d| d.grand_total.total_seconds).sum()
}

// ─────────────────────────────────────────────────────────────────────────────
// SummaryRenderer
// ─────────────────────────────────────────────────────────────────────────────

/// Zero-size type that groups all summary rendering methods.
///
/// All methods are pure: they take a reference to a [`SummaryResponse`] and
/// return an owned `String`; they do not perform any I/O.
pub struct SummaryRenderer;

impl SummaryRenderer {
    /// Renders a [`SummaryResponse`] as a Unicode bordered table.
    ///
    /// The table shows the top languages aggregated across all days in the
    /// response, with a duration, ASCII progress bar, and percentage column.
    ///
    /// `opts.color` is not currently used by this renderer (colour is handled
    /// by the caller via `owo-colors`). `opts.format` is ignored; the caller
    /// is responsible for dispatching to the correct renderer.
    #[must_use]
    pub fn render_table(resp: &SummaryResponse, _opts: &RenderOptions) -> String {
        let total = grand_total_seconds(resp);
        let langs = aggregate(resp.data.iter().map(|d| d.languages.clone()));

        let mut table = Table::new();
        table
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Disabled)
            .set_header(vec![
                Cell::new("Language"),
                Cell::new("Time"),
                Cell::new("Bar"),
                Cell::new("%"),
            ]);

        for (name, secs) in &langs {
            let ratio = if total > 0.0 { secs / total } else { 0.0 };
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let secs_u64 = secs.round() as u64;
            table.add_row(vec![
                Cell::new(name.as_str()),
                Cell::new(format_duration(secs_u64)),
                Cell::new(format_bar(ratio, BAR_WIDTH)),
                Cell::new(format!("{:.1}%", ratio * 100.0)),
            ]);
        }

        // Footer row with grand total.
        if total > 0.0 {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let total_secs_u64 = total.round() as u64;
            table.add_row(vec![
                Cell::new("Total"),
                Cell::new(format_duration(total_secs_u64)),
                Cell::new(String::new()),
                Cell::new(String::new()),
            ]);
        }

        table.to_string()
    }

    /// Renders a [`SummaryResponse`] as pretty-printed JSON.
    ///
    /// The output is the full API response serialized to JSON, suitable for
    /// piping into `jq` or other JSON tooling.
    ///
    /// # Panics
    ///
    /// This method panics if serialization fails. In practice this cannot
    /// happen because [`SummaryResponse`] only contains JSON-safe types.
    #[must_use]
    pub fn render_json(resp: &SummaryResponse) -> String {
        // SAFETY: SummaryResponse only contains primitive JSON-compatible
        // types (String, f64, u32, bool, Option<_>, Vec<_>). Serialization
        // is infallible for this type.
        serde_json::to_string_pretty(resp).expect("SummaryResponse is always JSON-serializable")
    }

    /// Renders a [`SummaryResponse`] as a plain-text language breakdown.
    ///
    /// No ANSI escape codes, no table borders — safe to pipe to files and
    /// other tools.  Output is fixed-width with space-padded columns.
    #[must_use]
    pub fn render_plain(resp: &SummaryResponse, _opts: &RenderOptions) -> String {
        let total = grand_total_seconds(resp);
        let langs = aggregate(resp.data.iter().map(|d| d.languages.clone()));

        let name_width = langs.iter().map(|(n, _)| n.len()).max().unwrap_or(8).max(8);

        let mut out = String::new();

        // Header
        writeln!(
            out,
            "{:<width$}  {:<10}  {:>6}",
            "Language",
            "Time",
            "%",
            width = name_width,
        )
        .expect("writing to String is infallible");
        out.push_str(&"-".repeat(name_width + 22));
        out.push('\n');

        for (name, secs) in &langs {
            let ratio = if total > 0.0 { secs / total } else { 0.0 };
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let secs_u64 = secs.round() as u64;
            writeln!(
                out,
                "{:<width$}  {:<10}  {:>5.1}%",
                name,
                format_duration(secs_u64),
                ratio * 100.0,
                width = name_width,
            )
            .expect("writing to String is infallible");
        }

        // Footer
        if total > 0.0 {
            out.push_str(&"-".repeat(name_width + 22));
            out.push('\n');
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let total_secs_u64 = total.round() as u64;
            writeln!(
                out,
                "{:<width$}  {:<10}",
                "Total",
                format_duration(total_secs_u64),
                width = name_width,
            )
            .expect("writing to String is infallible");
        }

        out
    }

    /// Dispatches to the appropriate renderer based on `opts.format`.
    ///
    /// This is a convenience method for the binary crate — it avoids a
    /// `match` expression at every call site.
    #[must_use]
    pub fn render(resp: &SummaryResponse, opts: &RenderOptions) -> String {
        match opts.format {
            OutputFormat::Json => Self::render_json(resp),
            OutputFormat::Plain | OutputFormat::Csv | OutputFormat::Tsv => {
                // TODO(spec): CSV and TSV formats are not yet fully specified.
                // Fall back to plain text until spec §output clarifies the
                // column layout for machine-readable formats.
                Self::render_plain(resp, opts)
            }
            OutputFormat::Table => Self::render_table(resp, opts),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Load the `summaries_today.json` fixture used for snapshot testing.
    fn fixture_today() -> SummaryResponse {
        let json = include_str!("../../../tests/fixtures/summaries_today.json");
        serde_json::from_str(json).expect("fixture is valid JSON")
    }

    #[test]
    fn snapshot_render_table() {
        let resp = fixture_today();
        let opts = RenderOptions {
            color: false,
            ..RenderOptions::default()
        };
        let output = SummaryRenderer::render_table(&resp, &opts);
        insta::assert_snapshot!(output);
    }

    #[test]
    fn snapshot_render_json() {
        let resp = fixture_today();
        let output = SummaryRenderer::render_json(&resp);
        insta::assert_snapshot!(output);
    }

    #[test]
    fn snapshot_render_plain() {
        let resp = fixture_today();
        let opts = RenderOptions {
            color: false,
            ..RenderOptions::default()
        };
        let output = SummaryRenderer::render_plain(&resp, &opts);
        insta::assert_snapshot!(output);
    }

    #[test]
    fn render_plain_contains_rust_language() {
        let resp = fixture_today();
        let opts = RenderOptions::default();
        let out = SummaryRenderer::render_plain(&resp, &opts);
        assert!(out.contains("Rust"), "plain output should list Rust");
    }

    #[test]
    fn render_plain_contains_total() {
        let resp = fixture_today();
        let opts = RenderOptions::default();
        let out = SummaryRenderer::render_plain(&resp, &opts);
        assert!(
            out.contains("Total"),
            "plain output should include a total row"
        );
    }

    #[test]
    fn render_json_is_valid_json() {
        let resp = fixture_today();
        let out = SummaryRenderer::render_json(&resp);
        let parsed: serde_json::Value =
            serde_json::from_str(&out).expect("render_json must return valid JSON");
        assert!(parsed.get("data").is_some(), "JSON must have a `data` key");
    }

    #[test]
    fn render_table_contains_language_header() {
        let resp = fixture_today();
        let opts = RenderOptions::default();
        let out = SummaryRenderer::render_table(&resp, &opts);
        assert!(
            out.contains("Language"),
            "table should have a Language column"
        );
    }

    #[test]
    fn aggregate_sums_across_days() {
        use waka_api::SummaryEntry;

        let entry = |name: &str, secs: f64| SummaryEntry {
            digital: String::new(),
            hours: 0,
            minutes: 0,
            name: name.to_owned(),
            percent: 0.0,
            seconds: 0,
            text: String::new(),
            total_seconds: secs,
        };

        let day1 = vec![entry("Rust", 3_600.0), entry("Python", 1_800.0)];
        let day2 = vec![entry("Rust", 1_800.0), entry("Go", 900.0)];

        let result = aggregate([day1, day2].into_iter());

        // Rust should be first (5400 s), Python second (1800 s), Go third (900 s).
        assert_eq!(result[0].0, "Rust");
        assert!((result[0].1 - 5_400.0).abs() < f64::EPSILON);
        assert_eq!(result[1].0, "Python");
        assert_eq!(result[2].0, "Go");
    }
}
