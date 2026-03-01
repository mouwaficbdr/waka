//! Renderer for `WakaTime` leaderboard API responses.
//!
//! The [`LeaderboardRenderer`] struct converts a [`waka_api::LeaderboardResponse`]
//! into human-readable strings in various output formats.

use std::fmt::Write as _;

use comfy_table::{presets, Cell, ContentArrangement, Table};
use waka_api::LeaderboardResponse;

use crate::format::format_duration;
use crate::options::{OutputFormat, RenderOptions};

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Returns seconds as u64 (clamped at 0).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn secs(f: f64) -> u64 {
    f.max(0.0).floor() as u64
}

// ─────────────────────────────────────────────────────────────────────────────
// LeaderboardRenderer
// ─────────────────────────────────────────────────────────────────────────────

/// Zero-size type that groups all leaderboard rendering methods.
///
/// All methods are pure: they take leaderboard data and return owned `String`s
/// without performing any I/O.
pub struct LeaderboardRenderer;

impl LeaderboardRenderer {
    /// Renders a [`LeaderboardResponse`] using the format specified in `opts`.
    #[must_use]
    pub fn render(resp: &LeaderboardResponse, opts: &RenderOptions) -> String {
        match opts.format {
            OutputFormat::Json => Self::render_json(resp),
            OutputFormat::Csv => Self::render_csv(resp, opts),
            OutputFormat::Tsv => Self::render_tsv(resp),
            OutputFormat::Plain => Self::render_plain(resp),
            OutputFormat::Table => Self::render_table(resp, opts),
        }
    }

    // ── table ─────────────────────────────────────────────────────────────────

    /// Renders the leaderboard as a Unicode bordered table.
    #[must_use]
    pub fn render_table(resp: &LeaderboardResponse, _opts: &RenderOptions) -> String {
        if resp.data.is_empty() {
            return "No leaderboard entries found.".to_owned();
        }

        let mut table = Table::new();
        table
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Disabled)
            .set_header(vec![
                Cell::new("Rank"),
                Cell::new("User"),
                Cell::new("Total"),
                Cell::new("Daily Avg"),
                Cell::new("Top Language"),
            ]);

        for entry in &resp.data {
            let total = format_duration(secs(entry.running_total.total_seconds));
            let avg = format_duration(secs(entry.running_total.daily_average));

            let top_lang = entry
                .running_total
                .languages
                .first()
                .map_or("—".to_owned(), |l| {
                    format!("{} ({:.1}%)", l.name, l.percent)
                });

            let user_name = &entry.user.display_name;

            table.add_row(vec![
                Cell::new(entry.rank.unwrap_or(0)),
                Cell::new(user_name),
                Cell::new(&total),
                Cell::new(&avg),
                Cell::new(&top_lang),
            ]);
        }

        let mut output = format!("{table}\n");

        // Append current user highlight if present and outside the page data.
        if let Some(cu) = &resp.current_user {
            if !resp.data.iter().any(|e| e.rank == cu.rank) {
                let total = format_duration(secs(cu.running_total.total_seconds));
                let avg = format_duration(secs(cu.running_total.daily_average));
                let top_lang = cu
                    .running_total
                    .languages
                    .first()
                    .map_or("—".to_owned(), |l| {
                        format!("{} ({:.1}%)", l.name, l.percent)
                    });

                writeln!(
                    output,
                    "\nYour rank: #{} — {} (daily avg: {}) — {}",
                    cu.rank.unwrap_or(0),
                    total,
                    avg,
                    top_lang
                )
                .unwrap_or_default();
            }
        }

        output
    }

    // ── plain ─────────────────────────────────────────────────────────────────

    /// Renders the leaderboard as plain text (one entry per line).
    #[must_use]
    pub fn render_plain(resp: &LeaderboardResponse) -> String {
        if resp.data.is_empty() {
            return "No leaderboard entries found.\n".to_owned();
        }

        let mut out = String::new();
        for entry in &resp.data {
            let total = format_duration(secs(entry.running_total.total_seconds));
            let avg = format_duration(secs(entry.running_total.daily_average));
            let top_lang = entry
                .running_total
                .languages
                .first()
                .map_or("—", |l| l.name.as_str());

            writeln!(
                out,
                "{:2}. {} — {} (daily avg: {}) — {}",
                entry.rank.unwrap_or(0),
                entry.user.display_name,
                total,
                avg,
                top_lang
            )
            .unwrap_or_default();
        }

        if let Some(cu) = &resp.current_user {
            if !resp.data.iter().any(|e| e.rank == cu.rank) {
                let total = format_duration(secs(cu.running_total.total_seconds));
                let avg = format_duration(secs(cu.running_total.daily_average));
                writeln!(
                    out,
                    "\nYour rank: #{} — {} (daily avg: {})",
                    cu.rank.unwrap_or(0),
                    total,
                    avg
                )
                .unwrap_or_default();
            }
        }

        out
    }

    // ── json ──────────────────────────────────────────────────────────────────

    /// Renders the leaderboard as a pretty-printed JSON string.
    #[must_use]
    pub fn render_json(resp: &LeaderboardResponse) -> String {
        serde_json::to_string_pretty(resp).unwrap_or_default()
    }

    // ── csv / tsv ─────────────────────────────────────────────────────────────

    /// Renders the leaderboard as CSV.
    #[must_use]
    pub fn render_csv(resp: &LeaderboardResponse, opts: &RenderOptions) -> String {
        Self::render_delimited(resp, ',', opts.csv_bom)
    }

    /// Renders the leaderboard as TSV.
    #[must_use]
    pub fn render_tsv(resp: &LeaderboardResponse) -> String {
        Self::render_delimited(resp, '\t', false)
    }

    fn render_delimited(resp: &LeaderboardResponse, sep: char, bom: bool) -> String {
        let mut out = String::new();
        if bom {
            out.push('\u{FEFF}');
        }
        writeln!(
            out,
            "rank{sep}user{sep}total_seconds{sep}daily_average{sep}top_language"
        )
        .unwrap_or_default();
        for entry in &resp.data {
            let top_lang = entry
                .running_total
                .languages
                .first()
                .map_or("", |l| l.name.as_str());
            writeln!(
                out,
                "{}{sep}{}{sep}{}{sep}{}{sep}{}",
                entry.rank.unwrap_or(0),
                entry.user.display_name,
                secs(entry.running_total.total_seconds),
                secs(entry.running_total.daily_average),
                top_lang,
            )
            .unwrap_or_default();
        }
        out
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use waka_api::LeaderboardResponse;

    use super::*;

    #[test]
    fn render_plain_shows_ranks() {
        let resp: LeaderboardResponse =
            serde_json::from_str(include_str!("../../../tests/fixtures/leaderboard.json"))
                .expect("fixture must parse");
        let out = LeaderboardRenderer::render_plain(&resp);
        assert!(out.contains("1."), "should show rank 1");
        assert!(out.contains("Alice Coder"));
    }

    #[test]
    fn render_plain_shows_current_user_if_not_in_page() {
        let resp: LeaderboardResponse =
            serde_json::from_str(include_str!("../../../tests/fixtures/leaderboard.json"))
                .expect("fixture must parse");
        let out = LeaderboardRenderer::render_plain(&resp);
        // Current user rank=4 IS in page data, so "Your rank" message should NOT appear.
        assert!(
            !out.contains("Your rank: #4"),
            "should not append when rank is in table"
        );
    }

    #[test]
    fn render_csv_has_header() {
        let resp: LeaderboardResponse =
            serde_json::from_str(include_str!("../../../tests/fixtures/leaderboard.json"))
                .expect("fixture must parse");
        let opts = RenderOptions {
            format: OutputFormat::Csv,
            ..RenderOptions::default()
        };
        let out = LeaderboardRenderer::render_csv(&resp, &opts);
        assert!(out.contains("rank,user,total_seconds"));
    }

    #[test]
    fn render_tsv_uses_tab() {
        let resp: LeaderboardResponse =
            serde_json::from_str(include_str!("../../../tests/fixtures/leaderboard.json"))
                .expect("fixture must parse");
        let out = LeaderboardRenderer::render_tsv(&resp);
        assert!(out.contains('\t'));
        assert!(!out.contains(','));
    }

    #[test]
    fn snapshot_render_leaderboard_plain() {
        let resp: LeaderboardResponse =
            serde_json::from_str(include_str!("../../../tests/fixtures/leaderboard.json"))
                .expect("fixture must parse");
        let out = LeaderboardRenderer::render_plain(&resp);
        insta::assert_snapshot!(out);
    }

    #[test]
    fn snapshot_render_leaderboard_csv() {
        let resp: LeaderboardResponse =
            serde_json::from_str(include_str!("../../../tests/fixtures/leaderboard.json"))
                .expect("fixture must parse");
        let opts = RenderOptions {
            format: OutputFormat::Csv,
            ..RenderOptions::default()
        };
        let out = LeaderboardRenderer::render_csv(&resp, &opts);
        insta::assert_snapshot!(out);
    }
}
