//! Renderer for `WakaTime` project list API responses.
//!
//! The [`ProjectRenderer`] converts a [`waka_api::ProjectsResponse`] into a
//! human-readable string in several output formats.

use std::fmt::Write as _;

use chrono::{DateTime, Utc};
use owo_colors::OwoColorize as _;
use waka_api::{Project, ProjectsResponse};

use crate::options::{OutputFormat, RenderOptions};
use crate::theme::Theme;

// ─────────────────────────────────────────────────────────────────────────────
// ProjectRenderer
// ─────────────────────────────────────────────────────────────────────────────

/// Zero-size type that groups all project-list rendering methods.
///
/// All methods are pure: they take project data and return owned `String`s
/// without performing any I/O.
pub struct ProjectRenderer;

impl ProjectRenderer {
    /// Renders a [`ProjectsResponse`] using the format specified in `opts`.
    ///
    /// `limit` caps the number of entries shown (`None` = no cap).  When
    /// `sort_by_name` is `true` entries are sorted alphabetically; otherwise
    /// they are displayed in the order returned by the API (most-recently
    /// active first).
    #[must_use]
    pub fn render_list(
        resp: &ProjectsResponse,
        limit: Option<usize>,
        sort_by_name: bool,
        opts: &RenderOptions,
    ) -> String {
        match opts.format {
            OutputFormat::Json => Self::render_json(resp),
            OutputFormat::Csv => Self::render_csv(resp, opts.csv_bom),
            OutputFormat::Tsv => Self::render_tsv(resp),
            OutputFormat::Plain => Self::render_plain(resp, limit, sort_by_name),
            OutputFormat::Table => Self::render_rich(resp, limit, sort_by_name, opts),
        }
    }

    /// Renders as pretty-printed JSON.
    #[must_use]
    pub fn render_json(resp: &ProjectsResponse) -> String {
        serde_json::to_string_pretty(resp).unwrap_or_default()
    }

    /// Renders as CSV (RFC 4180).
    ///
    /// Columns: `name`, `active`, `last_heartbeat`, `id`.
    #[must_use]
    pub fn render_csv(resp: &ProjectsResponse, bom: bool) -> String {
        let mut out = String::new();
        if bom {
            out.push('\u{FEFF}');
        }
        writeln!(out, "name,active,last_heartbeat,id").unwrap_or_default();
        for p in &resp.data {
            let active = is_active_today(&p.last_heartbeat_at);
            writeln!(
                out,
                "{},{},{},{}",
                p.name, active, p.human_readable_last_heartbeat_at, p.id
            )
            .unwrap_or_default();
        }
        out
    }

    /// Renders as TSV.
    ///
    /// Columns: `name`, `active`, `last_heartbeat`, `id`.
    #[must_use]
    pub fn render_tsv(resp: &ProjectsResponse) -> String {
        let mut out = String::new();
        writeln!(out, "name\tactive\tlast_heartbeat\tid").unwrap_or_default();
        for p in &resp.data {
            let active = is_active_today(&p.last_heartbeat_at);
            writeln!(
                out,
                "{}\t{}\t{}\t{}",
                p.name, active, p.human_readable_last_heartbeat_at, p.id
            )
            .unwrap_or_default();
        }
        out
    }

    /// Renders as plain text (no ANSI, no box).
    #[must_use]
    pub fn render_plain(
        resp: &ProjectsResponse,
        limit: Option<usize>,
        sort_by_name: bool,
    ) -> String {
        if resp.data.is_empty() {
            return "No projects found.\n".to_owned();
        }

        let mut projects: Vec<&Project> = resp.data.iter().collect();
        if sort_by_name {
            projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }

        let cap = limit.unwrap_or(usize::MAX);
        let mut out = String::new();
        for p in projects.iter().take(cap) {
            let dot = if is_active_today(&p.last_heartbeat_at) {
                "●"
            } else {
                "○"
            };
            writeln!(
                out,
                "{dot}  {}  ({})",
                p.name, p.human_readable_last_heartbeat_at
            )
            .unwrap_or_default();
        }
        out
    }

    /// Renders as rich visual layout (coloured dots + aligned columns).
    fn render_rich(
        resp: &ProjectsResponse,
        limit: Option<usize>,
        sort_by_name: bool,
        opts: &RenderOptions,
    ) -> String {
        if resp.data.is_empty() {
            return "  No projects found.\n".to_owned();
        }

        let color = opts.color;
        let theme = if color {
            Theme::colored()
        } else {
            Theme::plain()
        };

        let mut projects: Vec<&Project> = resp.data.iter().collect();
        if sort_by_name {
            projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }

        let cap = limit.unwrap_or(usize::MAX);
        let entries: Vec<&Project> = projects.into_iter().take(cap).collect();

        let max_name_w = entries
            .iter()
            .map(|p| p.name.len())
            .max()
            .unwrap_or(8)
            .max(8);
        let total = entries.len();
        let active_count = entries
            .iter()
            .filter(|p| is_active_today(&p.last_heartbeat_at))
            .count();

        let mut out = String::new();

        // ── Header ─────────────────────────────────────────────────────────────
        write!(out, "  ").unwrap_or_default();
        if color {
            write!(out, "{}", "Projects".style(theme.bold)).unwrap_or_default();
        } else {
            write!(out, "Projects").unwrap_or_default();
        }
        let s_suffix = if total == 1 { "" } else { "s" };
        writeln!(
            out,
            "  {total} project{s_suffix}, {active_count} active today"
        )
        .unwrap_or_default();
        out.push('\n');

        // ── Rows ───────────────────────────────────────────────────────────────
        for p in &entries {
            let active = is_active_today(&p.last_heartbeat_at);
            let dot = if active { "●" } else { "○" };
            let time = &p.human_readable_last_heartbeat_at;
            // Compute manual padding so ANSI codes don't break alignment.
            let pad = " ".repeat(max_name_w.saturating_sub(p.name.len()));

            if color {
                let dot_s = if active {
                    dot.style(theme.accent).to_string()
                } else {
                    dot.style(theme.muted).to_string()
                };
                let name_s = if active {
                    format!("{}", p.name.style(theme.bold))
                } else {
                    p.name.clone()
                };
                let time_s = format!("{}", time.style(theme.muted));
                writeln!(out, "  {dot_s}  {name_s}{pad}  {time_s}").unwrap_or_default();
            } else {
                let name = &p.name;
                writeln!(out, "  {dot}  {name}{pad}  {time}").unwrap_or_default();
            }
        }

        out
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Returns `true` if the ISO 8601 timestamp falls within the last 24 hours.
fn is_active_today(last_heartbeat_at: &str) -> bool {
    last_heartbeat_at
        .parse::<DateTime<Utc>>()
        .map(|dt| (Utc::now() - dt).num_hours() < 24)
        .unwrap_or(false)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Constructs a [`waka_api::Project`] via JSON deserialization so that the
    /// `#[non_exhaustive]` restriction on struct-literal expressions is
    /// respected even from outside `waka-api`.
    fn make_project(name: &str, last_heartbeat_at: &str, human: &str) -> waka_api::Project {
        serde_json::from_value(serde_json::json!({
            "badge": null,
            "color": null,
            "created_at": "2024-01-01T00:00:00Z",
            "has_public_url": false,
            "human_readable_last_heartbeat_at": human,
            "id": format!("id-{name}"),
            "last_heartbeat_at": last_heartbeat_at,
            "name": name,
            "repository": null,
            "url": null,
            "urlencoded_name": name,
        }))
        .expect("valid project JSON")
    }

    fn make_response(projects: &[waka_api::Project]) -> waka_api::ProjectsResponse {
        let json_projects = serde_json::to_value(projects).expect("serializable");
        serde_json::from_value(serde_json::json!({ "data": json_projects }))
            .expect("valid projects response JSON")
    }

    // ── is_active_today ───────────────────────────────────────────────────────

    #[test]
    fn is_active_today_recent() {
        // A timestamp 1 hour ago should be active.
        let ts = (Utc::now() - chrono::Duration::hours(1))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        assert!(is_active_today(&ts));
    }

    #[test]
    fn is_active_today_old() {
        // A timestamp 2 days ago should NOT be active.
        let ts = (Utc::now() - chrono::Duration::days(2))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        assert!(!is_active_today(&ts));
    }

    #[test]
    fn is_active_today_invalid() {
        // An invalid timestamp should return false, never panic.
        assert!(!is_active_today("not-a-date"));
    }

    // ── render_plain ──────────────────────────────────────────────────────────

    #[test]
    fn render_plain_empty() {
        let resp = make_response(&[]);
        let out = ProjectRenderer::render_plain(&resp, None, false);
        assert!(out.contains("No projects found"));
    }

    #[test]
    fn render_plain_shows_dot_and_name() {
        let now = (Utc::now() - chrono::Duration::minutes(30))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let resp = make_response(&[make_project("waka", &now, "30 minutes ago")]);
        let out = ProjectRenderer::render_plain(&resp, None, false);
        assert!(out.contains("●"), "active project should show ●");
        assert!(out.contains("waka"));
        assert!(out.contains("30 minutes ago"));
    }

    #[test]
    fn render_plain_inactive_dot() {
        let old = (Utc::now() - chrono::Duration::days(5))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let resp = make_response(&[make_project("old-project", &old, "5 days ago")]);
        let out = ProjectRenderer::render_plain(&resp, None, false);
        assert!(out.contains("○"), "inactive project should show ○");
    }

    #[test]
    fn render_plain_limit() {
        let old = (Utc::now() - chrono::Duration::days(5))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let resp = make_response(&[
            make_project("a", &old, "5 days ago"),
            make_project("b", &old, "5 days ago"),
            make_project("c", &old, "5 days ago"),
        ]);
        let out = ProjectRenderer::render_plain(&resp, Some(2), false);
        assert!(out.contains('a'));
        assert!(out.contains('b'));
        assert!(!out.contains('c'), "limit should exclude third project");
    }

    #[test]
    fn render_plain_sort_by_name() {
        let old = (Utc::now() - chrono::Duration::days(5))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let resp = make_response(&[
            make_project("zeta", &old, "5 days ago"),
            make_project("alpha", &old, "5 days ago"),
        ]);
        let out = ProjectRenderer::render_plain(&resp, None, true);
        let alpha_pos = out.find("alpha").unwrap_or(usize::MAX);
        let zeta_pos = out.find("zeta").unwrap_or(usize::MAX);
        assert!(
            alpha_pos < zeta_pos,
            "sort_by_name should put alpha before zeta"
        );
    }

    // ── render_csv ────────────────────────────────────────────────────────────

    #[test]
    fn render_csv_has_header() {
        let resp = make_response(&[]);
        let out = ProjectRenderer::render_csv(&resp, false);
        assert!(out.contains("name,active,last_heartbeat,id"));
    }

    #[test]
    fn render_csv_with_bom() {
        let resp = make_response(&[]);
        let out = ProjectRenderer::render_csv(&resp, true);
        assert!(out.starts_with('\u{FEFF}'));
    }

    #[test]
    fn render_tsv_uses_tab() {
        let old = "2020-01-01T00:00:00Z";
        let resp = make_response(&[make_project("p", old, "long ago")]);
        let out = ProjectRenderer::render_tsv(&resp);
        assert!(out.contains('\t'));
    }

    // ── render_rich ───────────────────────────────────────────────────────────

    #[test]
    fn render_rich_empty() {
        let resp = make_response(&[]);
        let opts = RenderOptions::default();
        let out = ProjectRenderer::render_list(&resp, None, false, &opts);
        assert!(out.contains("No projects found"));
    }

    #[test]
    fn render_rich_includes_header_counts() {
        let now = (Utc::now() - chrono::Duration::minutes(10))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let old = (Utc::now() - chrono::Duration::days(3))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let resp = make_response(&[
            make_project("active-proj", &now, "10 minutes ago"),
            make_project("idle-proj", &old, "3 days ago"),
        ]);
        let opts = RenderOptions {
            color: false,
            ..RenderOptions::default()
        };
        let out = ProjectRenderer::render_list(&resp, None, false, &opts);
        assert!(out.contains("2 projects"), "should show total count");
        assert!(out.contains("1 active today"), "should show active count");
    }

    // ── snapshot ──────────────────────────────────────────────────────────────

    #[test]
    fn snapshot_render_projects_plain() {
        // Timestamps in the fixture are from 2025-01-12/13 — always inactive
        // by the time tests run, so output is deterministic.
        let resp: waka_api::ProjectsResponse =
            serde_json::from_str(include_str!("../../../tests/fixtures/projects.json"))
                .expect("fixture must parse");
        let out = ProjectRenderer::render_plain(&resp, None, false);
        insta::assert_snapshot!(out);
    }

    #[test]
    fn snapshot_render_projects_rich() {
        let resp: waka_api::ProjectsResponse =
            serde_json::from_str(include_str!("../../../tests/fixtures/projects.json"))
                .expect("fixture must parse");
        let opts = RenderOptions {
            color: false,
            ..RenderOptions::default()
        };
        let out = ProjectRenderer::render_list(&resp, None, false, &opts);
        insta::assert_snapshot!(out);
    }
}
