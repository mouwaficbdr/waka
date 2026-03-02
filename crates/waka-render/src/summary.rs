//! Renderers for `WakaTime` summary API responses.
//!
//! The [`SummaryRenderer`] struct is a zero-size type whose methods convert a
//! [`waka_api::SummaryResponse`] into a `String` in one of several formats.

use std::fmt::Write as _;

use chrono::{DateTime, Datelike as _, Utc};
use owo_colors::OwoColorize as _;
use waka_api::{SummaryEntry, SummaryResponse};

use crate::format::format_duration;
use crate::options::{OutputFormat, RenderOptions};
use crate::theme::Theme;
use crate::utils::humanize_duration;

/// Width of the language-bar in the rich layout (columns).
const RICH_BAR_WIDTH: usize = 22;

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

/// Format an ISO 8601 date-time string as "Month Day, Year" (e.g. "January 13, 2025").
///
/// Falls back to the first 10 characters (YYYY-MM-DD) if parsing fails.
fn fmt_date_long(iso: &str) -> String {
    iso.parse::<DateTime<Utc>>().map_or_else(
        |_| iso.get(..10).unwrap_or(iso).to_owned(),
        |dt| format!("{} {}, {}", dt.format("%B"), dt.day(), dt.year()),
    )
}

/// Build the date-range string for the header.
///
/// - Single day → `"January 13, 2025"`
/// - Same year  → `"Jan 24 – Mar 2, 2026"`
/// - Cross-year → `"Dec 30, 2025 – Jan 5, 2026"`
fn fmt_date_range(start_iso: &str, end_iso: &str) -> String {
    let start = start_iso.parse::<DateTime<Utc>>().ok();
    let end = end_iso.parse::<DateTime<Utc>>().ok();

    match (start, end) {
        (Some(s), Some(e)) => {
            let sd = s.date_naive();
            let ed = e.date_naive();
            if sd == ed {
                format!("{} {}, {}", s.format("%B"), s.day(), s.year())
            } else if s.year() == e.year() {
                format!(
                    "{} {} \u{2013} {} {}, {}",
                    s.format("%b"),
                    s.day(),
                    e.format("%b"),
                    e.day(),
                    e.year(),
                )
            } else {
                format!(
                    "{} {}, {} \u{2013} {} {}, {}",
                    s.format("%b"),
                    s.day(),
                    s.year(),
                    e.format("%b"),
                    e.day(),
                    e.year(),
                )
            }
        }
        _ => fmt_date_long(start_iso),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Rich layout renderer (Table format)
// ─────────────────────────────────────────────────────────────────────────────

/// Renders the new rich visual layout.
///
/// With colour:
/// ```text
///   ╭────────────────────────────────────────╮
///   │  Today · January 13, 2025              │
///   │  6h 42m  total                         │
///   ╰────────────────────────────────────────╯
///
///   Rust         ██████████████░░░░░░░░  2h 15m   49.5%
///   ...
///                ─────────────────────────────────────
///   Total                                   6h 42m
/// ```
///
/// Without colour (ASCII fall-back, no box):
/// ```text
///   Today · January 13, 2025
///   6h 42m  total
///
///   Rust         ##############--------  2h 15m   49.5%
///   ...
///                #############################
///   Total                                   6h 42m
/// ```
fn render_rich(resp: &SummaryResponse, opts: &RenderOptions) -> String {
    let color = opts.color;
    let theme = if color {
        Theme::colored()
    } else {
        Theme::plain()
    };

    let total = grand_total_seconds(resp);
    let langs = aggregate(resp.data.iter().map(|d| d.languages.clone()));

    // --- Header strings (unstyled, for length calculations) ---
    let date_str = fmt_date_range(&resp.start, &resp.end);
    let period_str: &str = opts
        .period_label
        .as_deref()
        .or_else(|| {
            resp.data
                .first()
                .and_then(|d| d.range.text.as_deref())
                .filter(|t| !t.is_empty())
        })
        .unwrap_or(&date_str);

    let header_line = if period_str == date_str {
        date_str.clone()
    } else {
        format!("{period_str} \u{00B7} {date_str}")
    };

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let total_secs = total.round() as u64;
    let total_time = humanize_duration(total_secs);
    let total_line = format!("{total_time}  total");

    // --- Column layout ---
    let max_name_w = langs.iter().map(|(n, _)| n.len()).max().unwrap_or(8).max(8);
    let label_col = max_name_w + 3; // 3-space right padding

    let term_w = usize::from(opts.width).max(40);
    // Box inner width: enough for the header, capped at 66.
    let min_for_header =
        header_line.len().max(total_line.len()) + 2 /* inner side padding */;
    let box_inner = (term_w.saturating_sub(6)).min(66).max(min_for_header);

    let mut out = String::new();

    // --- Header block ---
    if color {
        let top_bar = "\u{2500}".repeat(box_inner + 2); // ─ repeated
        writeln!(out, "  \u{256D}{top_bar}\u{256E}").expect("infallible"); // ╭─╮
        let pad1 = (box_inner + 2).saturating_sub(header_line.len() + 2);
        writeln!(
            out,
            "  \u{2502}  {}{}\u{2502}",
            header_line.style(theme.bold),
            " ".repeat(pad1),
        )
        .expect("infallible");
        let pad2 = (box_inner + 2).saturating_sub(total_line.len() + 2);
        writeln!(
            out,
            "  \u{2502}  {}{}\u{2502}",
            total_line.style(theme.accent),
            " ".repeat(pad2),
        )
        .expect("infallible");
        writeln!(out, "  \u{2570}{top_bar}\u{256F}").expect("infallible"); // ╰─╯
    } else {
        writeln!(out, "  {header_line}").expect("infallible");
        writeln!(out, "  {total_line}").expect("infallible");
    }
    out.push('\n'); // blank line between header and rows

    // --- Language rows ---
    for (name, secs) in &langs {
        let ratio = if total > 0.0 { secs / total } else { 0.0 };
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::cast_precision_loss
        )]
        let row_secs = secs.round() as u64;

        let time_str = humanize_duration(row_secs);
        let pct_str = format!("{:.1}%", ratio * 100.0);

        // Build coloured bar: filled portion uses lang colour, empty is muted.
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::cast_precision_loss
        )]
        let filled = ((ratio * RICH_BAR_WIDTH as f64).round() as usize).min(RICH_BAR_WIDTH);
        let empty = RICH_BAR_WIDTH - filled;
        let (fill_ch, empty_ch) = theme.bar_chars();
        let bar = if color {
            let lang_style = theme.lang_color(name);
            format!(
                "{}{}",
                fill_ch.repeat(filled).style(lang_style),
                empty_ch.repeat(empty).style(theme.muted),
            )
        } else {
            fill_ch.repeat(filled) + &empty_ch.repeat(empty)
        };

        writeln!(
            out,
            "  {name:<label_col$}{bar}  {time_str:>6}   {pct_str:>5}"
        )
        .expect("infallible");
    }

    // Separator — aligned with bar start position.
    let sep_width = RICH_BAR_WIDTH + 2 + 6 + 3 + 5; // bar + "  " + time6 + "   " + pct5
    let sep = if color {
        "\u{2500}".repeat(sep_width) // ─
    } else {
        "-".repeat(sep_width)
    };
    writeln!(out, "  {:<label_col$}{sep}", "").expect("infallible");

    // Total footer row.
    let total_col = label_col + RICH_BAR_WIDTH + 2;
    let total_time_str = humanize_duration(total_secs);
    writeln!(out, "  {:<total_col$}{total_time_str:>6}", "Total",).expect("infallible");

    out
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
    /// Renders a [`SummaryResponse`] using the rich visual layout.
    ///
    /// When `opts.color` is `true`, output includes Unicode box-drawing
    /// characters, language-coloured progress bars, and ANSI colour codes.
    /// When `false`, output uses ASCII bars and plain text — suitable for
    /// `NO_COLOR=1` environments while retaining the new layout.
    ///
    /// JSON, CSV and TSV formats bypass this renderer; see [`SummaryRenderer::render`].
    #[must_use]
    pub fn render_table(resp: &SummaryResponse, opts: &RenderOptions) -> String {
        render_rich(resp, opts)
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

    /// Renders a [`SummaryResponse`] as CSV (comma-separated values).
    ///
    /// Columns: `date, language, total_seconds, time, percent`.
    /// One row is emitted per language per day.
    /// An optional UTF-8 BOM is prepended when `bom` is `true`.
    #[must_use]
    pub fn render_csv(resp: &SummaryResponse, bom: bool) -> String {
        Self::render_dsv(resp, ',', bom)
    }

    /// Renders a [`SummaryResponse`] as TSV (tab-separated values).
    ///
    /// Columns: `date, language, total_seconds, time, percent`.
    #[must_use]
    pub fn render_tsv(resp: &SummaryResponse) -> String {
        Self::render_dsv(resp, '\t', false)
    }

    /// Shared delimiter-separated-values builder.
    fn render_dsv(resp: &SummaryResponse, sep: char, bom: bool) -> String {
        let mut out = String::new();
        if bom {
            out.push('\u{FEFF}');
        }

        // Header
        writeln!(
            out,
            "date{sep}language{sep}total_seconds{sep}time{sep}percent"
        )
        .expect("write to String is infallible");

        for day in &resp.data {
            let date = day
                .range
                .date
                .as_deref()
                .or(day.range.start.get(..10))
                .unwrap_or("unknown");

            let day_total: f64 = day.languages.iter().map(|e| e.total_seconds).sum();

            for entry in &day.languages {
                let pct = if day_total > 0.0 {
                    (entry.total_seconds / day_total) * 100.0
                } else {
                    0.0
                };
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let secs_u64 = entry.total_seconds.round() as u64;
                writeln!(
                    out,
                    "{date}{s}{name}{s}{secs}{s}{time}{s}{pct:.1}",
                    s = sep,
                    name = entry.name,
                    secs = secs_u64,
                    time = format_duration(secs_u64),
                    pct = pct,
                )
                .expect("write to String is infallible");
            }
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
            OutputFormat::Csv => Self::render_csv(resp, opts.csv_bom),
            OutputFormat::Tsv => Self::render_tsv(resp),
            OutputFormat::Plain => Self::render_plain(resp, opts),
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
            width: 100,
            period_label: Some("Today".to_owned()),
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
    fn render_table_contains_rust_language() {
        let resp = fixture_today();
        let opts = RenderOptions {
            color: false,
            ..RenderOptions::default()
        };
        let out = SummaryRenderer::render_table(&resp, &opts);
        assert!(out.contains("Rust"), "rich table output should list Rust");
    }

    #[test]
    fn render_table_contains_total() {
        let resp = fixture_today();
        let opts = RenderOptions {
            color: false,
            ..RenderOptions::default()
        };
        let out = SummaryRenderer::render_table(&resp, &opts);
        assert!(
            out.contains("Total"),
            "rich table output should include Total row"
        );
    }

    #[test]
    fn render_table_no_color_uses_ascii_bar() {
        let resp = fixture_today();
        let opts = RenderOptions {
            color: false,
            period_label: Some("Today".to_owned()),
            ..RenderOptions::default()
        };
        let out = SummaryRenderer::render_table(&resp, &opts);
        assert!(out.contains('#'), "NO_COLOR should use ASCII # for filled");
        assert!(out.contains('-'), "NO_COLOR should use ASCII - for empty");
        assert!(
            !out.contains('█'),
            "NO_COLOR must not emit Unicode block chars"
        );
    }

    #[test]
    fn render_table_no_color_has_no_box_chars() {
        let resp = fixture_today();
        let opts = RenderOptions {
            color: false,
            ..RenderOptions::default()
        };
        let out = SummaryRenderer::render_table(&resp, &opts);
        assert!(
            !out.contains('╭'),
            "NO_COLOR must not have box-drawing corners"
        );
        assert!(
            !out.contains('│'),
            "NO_COLOR must not have box-drawing sides"
        );
    }

    #[test]
    fn render_table_colored_has_box_chars() {
        let resp = fixture_today();
        let opts = RenderOptions {
            color: true,
            width: 100,
            period_label: Some("Today".to_owned()),
            ..RenderOptions::default()
        };
        let out = SummaryRenderer::render_table(&resp, &opts);
        assert!(
            out.contains('╭'),
            "colored output should have box-drawing corners"
        );
        assert!(
            out.contains('│'),
            "colored output should have box-drawing sides"
        );
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
            ai_additions: 0,
            ai_deletions: 0,
            human_additions: 0,
            human_deletions: 0,
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

    // ── CSV / TSV tests ────────────────────────────────────────────────────────

    #[test]
    fn render_csv_has_header_row() {
        let resp = fixture_today();
        let out = SummaryRenderer::render_csv(&resp, false);
        let first = out.lines().next().expect("at least one line");
        assert!(
            first.starts_with("date,language,"),
            "CSV header mismatch: {first}"
        );
    }

    #[test]
    fn render_csv_uses_comma_separator() {
        let resp = fixture_today();
        let out = SummaryRenderer::render_csv(&resp, false);
        for line in out.lines() {
            assert!(!line.contains('\t'), "CSV must not contain tab characters");
            let field_count = line.matches(',').count();
            assert_eq!(
                field_count, 4,
                "CSV must have 5 fields (4 commas) per line: {line}"
            );
        }
    }

    #[test]
    fn render_csv_with_bom_starts_with_bom() {
        let resp = fixture_today();
        let out = SummaryRenderer::render_csv(&resp, true);
        assert!(
            out.starts_with('\u{FEFF}'),
            "CSV with BOM must start with UTF-8 BOM"
        );
    }

    #[test]
    fn render_csv_without_bom_does_not_start_with_bom() {
        let resp = fixture_today();
        let out = SummaryRenderer::render_csv(&resp, false);
        assert!(
            !out.starts_with('\u{FEFF}'),
            "CSV without BOM must not start with BOM"
        );
    }

    #[test]
    fn render_tsv_uses_tab_separator() {
        let resp = fixture_today();
        let out = SummaryRenderer::render_tsv(&resp);
        let header = out.lines().next().expect("at least one line");
        assert!(
            header.contains('\t'),
            "TSV header must contain tab character"
        );
    }

    #[test]
    fn render_tsv_does_not_start_with_bom() {
        let resp = fixture_today();
        let out = SummaryRenderer::render_tsv(&resp);
        assert!(!out.starts_with('\u{FEFF}'), "TSV must never have a BOM");
    }

    #[test]
    fn snapshot_render_csv() {
        let resp = fixture_today();
        let out = SummaryRenderer::render_csv(&resp, false);
        insta::assert_snapshot!(out);
    }

    // ── fmt helpers ──────────────────────────────────────────────────────────

    #[test]
    fn fmt_date_long_parses_iso() {
        let result = fmt_date_long("2025-01-13T00:00:00Z");
        assert_eq!(result, "January 13, 2025");
    }

    #[test]
    fn fmt_date_range_single_day() {
        let result = fmt_date_range("2025-01-13T00:00:00Z", "2025-01-13T23:00:00Z");
        assert_eq!(result, "January 13, 2025");
    }

    #[test]
    fn fmt_date_range_multi_day_same_year() {
        let result = fmt_date_range("2026-02-24T00:00:00Z", "2026-03-02T23:00:00Z");
        assert!(result.contains("Feb"), "should contain abbreviated month");
        assert!(result.contains("Mar"), "should contain abbreviated month");
        assert!(result.contains("2026"), "should contain year");
    }
}
