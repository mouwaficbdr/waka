//! Renderer for `WakaTime` goal API responses.
//!
//! The [`GoalRenderer`] struct converts a [`waka_api::GoalsResponse`] or
//! individual [`waka_api::Goal`] into human-readable strings in various
//! output formats.

use std::fmt::Write as _;

use comfy_table::{presets, Cell, ContentArrangement, Table};
use waka_api::{Goal, GoalsResponse};

use crate::format::{format_bar, format_duration};
use crate::options::{OutputFormat, RenderOptions};

/// Width of the ASCII progress bar in the goals table.
const BAR_WIDTH: u8 = 16;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Returns a status symbol string for a goal status string.
fn status_symbol(status: &str) -> &'static str {
    match status {
        "success" => "✓",
        "fail" => "✗",
        "ignored" => "-",
        _ => "?",
    }
}

/// Formats the scope column (languages / projects restriction).
fn format_scope(goal: &Goal) -> String {
    if !goal.projects.is_empty() {
        goal.projects.join(", ")
    } else if !goal.languages.is_empty() {
        goal.languages.join(", ")
    } else {
        String::new()
    }
}

/// Returns the target duration for a goal in seconds.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn target_secs(goal: &Goal) -> u64 {
    goal.seconds.max(0.0).floor() as u64
}

/// Computes a progress ratio (0.0–1.0) for a goal from its most recent
/// `chart_data` entry, if available.
///
/// Returns `None` if no chart data is present.
fn progress_ratio(goal: &Goal) -> Option<f64> {
    let chart = goal.chart_data.as_ref()?.last()?;
    let target = chart.goal_seconds;
    if target <= 0.0 {
        return None;
    }
    Some((chart.actual_seconds / target).clamp(0.0, 1.0))
}

// ─────────────────────────────────────────────────────────────────────────────
// GoalRenderer
// ─────────────────────────────────────────────────────────────────────────────

/// Zero-size type that groups all goal rendering methods.
///
/// All methods are pure: they take goal data and return owned `String`s
/// without performing any I/O.
pub struct GoalRenderer;

impl GoalRenderer {
    /// Renders a [`GoalsResponse`] using the format specified in `opts`.
    #[must_use]
    pub fn render_list(resp: &GoalsResponse, opts: &RenderOptions) -> String {
        match opts.format {
            OutputFormat::Json => Self::render_json(resp),
            OutputFormat::Csv => Self::render_csv(resp, opts),
            OutputFormat::Tsv => Self::render_tsv(resp),
            OutputFormat::Plain => Self::render_plain_list(resp),
            OutputFormat::Table => Self::render_table(resp, opts),
        }
    }

    /// Renders a single [`Goal`] (detail view) using the format in `opts`.
    #[must_use]
    pub fn render_detail(goal: &Goal, opts: &RenderOptions) -> String {
        match opts.format {
            OutputFormat::Json => serde_json::to_string_pretty(goal).unwrap_or_default(),
            OutputFormat::Plain | OutputFormat::Table => Self::render_plain_detail(goal),
            OutputFormat::Csv | OutputFormat::Tsv => {
                // Wrap single goal in a minimal response for CSV consistency.
                let resp = GoalsResponse {
                    data: vec![goal.clone()],
                    total: 1,
                    total_pages: 1,
                };
                if opts.format == OutputFormat::Csv {
                    Self::render_csv(&resp, opts)
                } else {
                    Self::render_tsv(&resp)
                }
            }
        }
    }

    // ── table ─────────────────────────────────────────────────────────────────

    /// Renders the goal list as a Unicode bordered table.
    #[must_use]
    pub fn render_table(resp: &GoalsResponse, _opts: &RenderOptions) -> String {
        if resp.data.is_empty() {
            return "No goals found.".to_owned();
        }

        let mut table = Table::new();
        table
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Disabled)
            .set_header(vec![
                Cell::new("#"),
                Cell::new("Status"),
                Cell::new("Title"),
                Cell::new("Target"),
                Cell::new("Period"),
                Cell::new("Bar"),
                Cell::new("Scope"),
            ]);

        for (i, goal) in resp.data.iter().enumerate() {
            let rs = goal.range_status.as_deref().unwrap_or("");
            let status = format!("{} {}", status_symbol(rs), rs);
            let target = format_duration(target_secs(goal));
            let bar = if let Some(ratio) = progress_ratio(goal) {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let pct = (ratio * 100.0).round() as u64;
                format!("{} {}%", format_bar(ratio, BAR_WIDTH), pct)
            } else {
                String::new()
            };
            let scope = format_scope(goal);

            table.add_row(vec![
                Cell::new(i + 1),
                Cell::new(&status),
                Cell::new(&goal.title),
                Cell::new(&target),
                Cell::new(&goal.delta),
                Cell::new(&bar),
                Cell::new(&scope),
            ]);
        }

        format!("{table}\nTotal: {}\n", resp.total)
    }

    // ── plain list ────────────────────────────────────────────────────────────

    /// Renders the goal list as plain text (one goal per line).
    #[must_use]
    pub fn render_plain_list(resp: &GoalsResponse) -> String {
        if resp.data.is_empty() {
            return "No goals found.\n".to_owned();
        }

        let mut out = String::new();
        for (i, goal) in resp.data.iter().enumerate() {
            let rs = goal.range_status.as_deref().unwrap_or("");
            let sym = status_symbol(rs);
            let target = format_duration(target_secs(goal));
            let scope = format_scope(goal);
            let scope_part = if scope.is_empty() {
                String::new()
            } else {
                format!(" [{scope}]")
            };
            writeln!(
                out,
                "{}. {}  {} — {} / {}{}",
                i + 1,
                sym,
                goal.title,
                target,
                goal.delta,
                scope_part
            )
            .unwrap_or_default();
        }
        out
    }

    // ── plain detail ──────────────────────────────────────────────────────────

    /// Renders a single goal's full detail as plain text.
    #[must_use]
    pub fn render_plain_detail(goal: &Goal) -> String {
        let mut out = String::new();
        writeln!(out, "Title:   {}", goal.title).unwrap_or_default();
        writeln!(out, "ID:      {}", goal.id).unwrap_or_default();
        let rs = goal.range_status.as_deref().unwrap_or("");
        let rsr = goal.range_status_reason.as_deref().unwrap_or("");
        writeln!(out, "Status:  {rs} ({rsr})").unwrap_or_default();
        writeln!(
            out,
            "Target:  {} / {}",
            format_duration(target_secs(goal)),
            goal.delta
        )
        .unwrap_or_default();
        writeln!(out, "Type:    {}", goal.goal_type).unwrap_or_default();
        if !goal.languages.is_empty() {
            writeln!(out, "Languages: {}", goal.languages.join(", ")).unwrap_or_default();
        }
        if !goal.projects.is_empty() {
            writeln!(out, "Projects:  {}", goal.projects.join(", ")).unwrap_or_default();
        }
        if let Some(chart) = goal.chart_data.as_ref() {
            writeln!(out, "\nProgress ({} periods):", chart.len()).unwrap_or_default();
            for entry in chart {
                let sym = status_symbol(&entry.range_status);
                writeln!(
                    out,
                    "  {}  {} — {}/{} ({})",
                    sym,
                    entry.range.text,
                    entry.actual_seconds_text,
                    entry.goal_seconds_text,
                    entry.range_status_reason
                )
                .unwrap_or_default();
            }
        }
        out
    }

    // ── json ──────────────────────────────────────────────────────────────────

    /// Renders the goal list as a pretty-printed JSON string.
    #[must_use]
    pub fn render_json(resp: &GoalsResponse) -> String {
        serde_json::to_string_pretty(resp).unwrap_or_default()
    }

    // ── csv / tsv ─────────────────────────────────────────────────────────────

    /// Renders the goal list as CSV.
    #[must_use]
    pub fn render_csv(resp: &GoalsResponse, opts: &RenderOptions) -> String {
        Self::render_delimited(resp, ',', opts.csv_bom)
    }

    /// Renders the goal list as TSV.
    #[must_use]
    pub fn render_tsv(resp: &GoalsResponse) -> String {
        Self::render_delimited(resp, '\t', false)
    }

    fn render_delimited(resp: &GoalsResponse, sep: char, bom: bool) -> String {
        let mut out = String::new();
        if bom {
            out.push('\u{FEFF}');
        }
        writeln!(
            out,
            "id{sep}title{sep}status{sep}target_seconds{sep}period{sep}scope"
        )
        .unwrap_or_default();
        for goal in &resp.data {
            let scope = format_scope(goal);
            writeln!(
                out,
                "{}{sep}{}{sep}{}{sep}{}{sep}{}{sep}{}",
                goal.id,
                goal.title,
                goal.range_status.as_deref().unwrap_or(""),
                target_secs(goal),
                goal.delta,
                scope,
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
    use waka_api::{Goal, GoalChartEntry, GoalChartRange, GoalsResponse};

    use super::*;

    fn make_goal(id: &str, title: &str, delta: &str, seconds: f64, status: &str) -> Goal {
        Goal {
            average_status: None,
            chart_data: None,
            created_at: "2025-01-01T00:00:00Z".to_owned(),
            cumulative_status: None,
            custom_title: None,
            delta: delta.to_owned(),
            editors: vec![],
            id: id.to_owned(),
            ignore_days: vec![],
            ignore_zero_days: false,
            improve_by_percent: None,
            is_current_user_owner: false,
            is_enabled: true,
            is_inverse: false,
            is_snoozed: false,
            is_tweeting: false,
            languages: vec![],
            modified_at: Some("2025-01-10T00:00:00Z".to_owned()),
            owner: None,
            projects: vec![],
            range_status: Some(status.to_owned()),
            range_status_reason: Some(format!("{status} reason")),
            range_text: None,
            seconds,
            shared_with: vec![],
            snooze_until: None,
            status: status.to_owned(),
            status_percent_calculated: 0,
            subscribers: vec![],
            title: title.to_owned(),
            goal_type: "coding".to_owned(),
        }
    }

    fn make_response(goals: Vec<Goal>) -> GoalsResponse {
        let total = goals.len();
        #[allow(clippy::cast_possible_truncation)]
        let total = total as u32;
        GoalsResponse {
            data: goals,
            total,
            total_pages: 1,
        }
    }

    // ── status_symbol ─────────────────────────────────────────────────────────

    #[test]
    fn status_symbol_success() {
        assert_eq!(status_symbol("success"), "✓");
    }

    #[test]
    fn status_symbol_fail() {
        assert_eq!(status_symbol("fail"), "✗");
    }

    #[test]
    fn status_symbol_ignored() {
        assert_eq!(status_symbol("ignored"), "-");
    }

    #[test]
    fn status_symbol_unknown() {
        assert_eq!(status_symbol("pending"), "?");
    }

    // ── render_plain_list ─────────────────────────────────────────────────────

    #[test]
    fn render_plain_list_empty() {
        let resp = make_response(vec![]);
        let out = GoalRenderer::render_plain_list(&resp);
        assert!(out.contains("No goals found"));
    }

    #[test]
    fn render_plain_list_shows_symbol_and_title() {
        let resp = make_response(vec![make_goal(
            "id1",
            "Daily coding",
            "day",
            28800.0,
            "success",
        )]);
        let out = GoalRenderer::render_plain_list(&resp);
        assert!(out.contains("✓"), "should show success symbol");
        assert!(out.contains("Daily coding"));
        assert!(out.contains("8h 0m"));
        assert!(out.contains("day"));
    }

    #[test]
    fn render_plain_list_fail_goal() {
        let resp = make_response(vec![make_goal(
            "id2",
            "Python weekly",
            "week",
            36000.0,
            "fail",
        )]);
        let out = GoalRenderer::render_plain_list(&resp);
        assert!(out.contains("✗"), "should show fail symbol");
    }

    // ── render_csv ────────────────────────────────────────────────────────────

    #[test]
    fn render_csv_has_header() {
        let resp = make_response(vec![]);
        let opts = RenderOptions {
            format: OutputFormat::Csv,
            ..RenderOptions::default()
        };
        let out = GoalRenderer::render_csv(&resp, &opts);
        assert!(out.contains("id,title,status"));
    }

    #[test]
    fn render_csv_with_bom() {
        let resp = make_response(vec![]);
        let opts = RenderOptions {
            format: OutputFormat::Csv,
            csv_bom: true,
            ..RenderOptions::default()
        };
        let out = GoalRenderer::render_csv(&resp, &opts);
        assert!(out.starts_with('\u{FEFF}'));
    }

    #[test]
    fn render_tsv_uses_tab() {
        let resp = make_response(vec![make_goal(
            "id1",
            "Daily coding",
            "day",
            28800.0,
            "success",
        )]);
        let out = GoalRenderer::render_tsv(&resp);
        assert!(out.contains('\t'));
        assert!(!out.contains(','));
    }

    // ── progress_ratio ────────────────────────────────────────────────────────

    #[test]
    fn progress_ratio_with_chart_data() {
        let mut goal = make_goal("id1", "Daily", "day", 28800.0, "success");
        goal.chart_data = Some(vec![GoalChartEntry {
            actual_seconds: 14_400.0,
            actual_seconds_text: "4h".to_owned(),
            goal_seconds: 28800.0,
            goal_seconds_text: "8h".to_owned(),
            range: GoalChartRange {
                date: Some("2025-01-13".to_owned()),
                end: "2025-01-13T23:59:59Z".to_owned(),
                start: "2025-01-13T00:00:00Z".to_owned(),
                text: "Mon, Jan 13".to_owned(),
                timezone: "UTC".to_owned(),
            },
            range_status: "fail".to_owned(),
            range_status_reason: "not enough".to_owned(),
        }]);
        let ratio = progress_ratio(&goal).expect("should have ratio");
        assert!((ratio - 0.5).abs() < 0.01, "ratio should be ~0.5");
    }

    #[test]
    fn progress_ratio_without_chart_data() {
        let goal = make_goal("id1", "Daily", "day", 28800.0, "success");
        assert!(progress_ratio(&goal).is_none());
    }

    // ── snapshot ──────────────────────────────────────────────────────────────

    #[test]
    fn snapshot_render_goals_plain() {
        let resp: GoalsResponse =
            serde_json::from_str(include_str!("../../../tests/fixtures/goals.json"))
                .expect("fixture must parse");
        let out = GoalRenderer::render_plain_list(&resp);
        insta::assert_snapshot!(out);
    }

    #[test]
    fn snapshot_render_goals_csv() {
        let resp: GoalsResponse =
            serde_json::from_str(include_str!("../../../tests/fixtures/goals.json"))
                .expect("fixture must parse");
        let opts = RenderOptions {
            format: OutputFormat::Csv,
            ..RenderOptions::default()
        };
        let out = GoalRenderer::render_csv(&resp, &opts);
        insta::assert_snapshot!(out);
    }
}
