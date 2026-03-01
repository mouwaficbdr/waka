//! Query parameter builders for `WakaTime` API endpoints.

use chrono::{Local, NaiveDate};

// ─────────────────────────────────────────────────────────────────────────────
// StatsRange
// ─────────────────────────────────────────────────────────────────────────────

/// Predefined time ranges accepted by the `GET /users/current/stats/{range}`
/// endpoint.
///
/// # Example
///
/// ```rust
/// use waka_api::StatsRange;
///
/// let path_segment = StatsRange::Last7Days.as_str();
/// assert_eq!(path_segment, "last_7_days");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatsRange {
    /// The last 7 days.
    Last7Days,
    /// The last 30 days.
    Last30Days,
    /// The last 6 months.
    Last6Months,
    /// The last year.
    LastYear,
    /// All time since the account was created.
    AllTime,
}

impl StatsRange {
    /// Returns the URL path segment for this range as expected by the API.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Last7Days => "last_7_days",
            Self::Last30Days => "last_30_days",
            Self::Last6Months => "last_6_months",
            Self::LastYear => "last_year",
            Self::AllTime => "all_time",
        }
    }
}

/// Query parameters for the `GET /users/current/summaries` endpoint.
///
/// # Example
///
/// ```rust
/// use waka_api::SummaryParams;
///
/// // Fetch today's summary
/// let p = SummaryParams::today();
///
/// // Fetch the last 7 days, filtered to one project
/// use chrono::NaiveDate;
/// let p = SummaryParams::for_range(
///     NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
///     NaiveDate::from_ymd_opt(2025, 1, 12).unwrap(),
/// )
/// .project("my-saas");
/// ```
#[derive(Debug, Clone)]
pub struct SummaryParams {
    /// Inclusive start date (`start_date` query param, `YYYY-MM-DD`).
    pub(crate) start: NaiveDate,
    /// Inclusive end date (`end_date` query param, `YYYY-MM-DD`).
    pub(crate) end: NaiveDate,
    /// Optional project filter.
    pub(crate) project: Option<String>,
    /// Optional comma-separated branch filter.
    pub(crate) branches: Option<String>,
}

impl SummaryParams {
    /// Creates params covering only today's date (local timezone).
    #[must_use]
    pub fn today() -> Self {
        let today = Local::now().date_naive();
        Self {
            start: today,
            end: today,
            project: None,
            branches: None,
        }
    }

    /// Creates params covering the given inclusive date range.
    #[must_use]
    pub fn for_range(start: NaiveDate, end: NaiveDate) -> Self {
        Self {
            start,
            end,
            project: None,
            branches: None,
        }
    }

    /// Filters results to the named project (builder, consumes `self`).
    #[must_use]
    pub fn project(mut self, project: &str) -> Self {
        self.project = Some(project.to_owned());
        self
    }

    /// Filters results to the named branches, comma-separated (builder).
    #[must_use]
    pub fn branches(mut self, branches: &str) -> Self {
        self.branches = Some(branches.to_owned());
        self
    }

    /// Returns a stable cache key string that uniquely identifies this set of
    /// parameters.
    ///
    /// Format: `summaries:{start}:{end}` (with optional `:project:{name}` suffix).
    ///
    /// # Example
    ///
    /// ```rust
    /// use waka_api::SummaryParams;
    /// use chrono::NaiveDate;
    ///
    /// let p = SummaryParams::for_range(
    ///     NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
    ///     NaiveDate::from_ymd_opt(2025, 1, 12).unwrap(),
    /// ).project("my-saas");
    ///
    /// assert_eq!(p.cache_key(), "summaries:2025-01-06:2025-01-12:project:my-saas");
    /// ```
    #[must_use]
    pub fn cache_key(&self) -> String {
        let base = format!(
            "summaries:{}:{}",
            self.start.format("%Y-%m-%d"),
            self.end.format("%Y-%m-%d"),
        );
        match &self.project {
            Some(p) => format!("{base}:project:{p}"),
            None => base,
        }
    }

    /// Converts to a list of `(key, value)` pairs suitable for a query string.
    ///
    /// Dates are formatted as `YYYY-MM-DD` as required by the `WakaTime` API.
    #[must_use]
    pub(crate) fn to_query_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = vec![
            (
                "start".to_owned(),
                self.start.format("%Y-%m-%d").to_string(),
            ),
            ("end".to_owned(), self.end.format("%Y-%m-%d").to_string()),
        ];
        if let Some(p) = &self.project {
            pairs.push(("project".to_owned(), p.clone()));
        }
        if let Some(b) = &self.branches {
            pairs.push(("branches".to_owned(), b.clone()));
        }
        pairs
    }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn today_has_equal_start_and_end() {
        let p = SummaryParams::today();
        assert_eq!(p.start, p.end);
        assert_eq!(p.start, Local::now().date_naive());
    }

    #[test]
    fn for_range_stores_dates() {
        let p = SummaryParams::for_range(date(2025, 1, 6), date(2025, 1, 12));
        assert_eq!(p.start, date(2025, 1, 6));
        assert_eq!(p.end, date(2025, 1, 12));
    }

    #[test]
    fn to_query_pairs_formats_dates_as_yyyy_mm_dd() {
        let p = SummaryParams::for_range(date(2025, 1, 6), date(2025, 1, 12));
        let pairs = p.to_query_pairs();
        assert_eq!(pairs[0], ("start".to_owned(), "2025-01-06".to_owned()));
        assert_eq!(pairs[1], ("end".to_owned(), "2025-01-12".to_owned()));
    }

    #[test]
    fn to_query_pairs_omits_optional_fields_when_none() {
        let p = SummaryParams::for_range(date(2025, 1, 6), date(2025, 1, 12));
        let pairs = p.to_query_pairs();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn to_query_pairs_includes_project_when_set() {
        let p = SummaryParams::for_range(date(2025, 1, 6), date(2025, 1, 12)).project("my-saas");
        let pairs = p.to_query_pairs();
        assert!(pairs.iter().any(|(k, v)| k == "project" && v == "my-saas"));
    }

    #[test]
    fn to_query_pairs_includes_branches_when_set() {
        let p = SummaryParams::for_range(date(2025, 1, 6), date(2025, 1, 12)).branches("main,dev");
        let pairs = p.to_query_pairs();
        assert!(pairs
            .iter()
            .any(|(k, v)| k == "branches" && v == "main,dev"));
    }

    #[test]
    fn project_builder_sets_project() {
        let p = SummaryParams::today().project("acme");
        assert_eq!(p.project.as_deref(), Some("acme"));
    }

    #[test]
    fn branches_builder_sets_branches() {
        let p = SummaryParams::today().branches("main");
        assert_eq!(p.branches.as_deref(), Some("main"));
    }

    #[test]
    fn full_params_produce_four_pairs() {
        let p = SummaryParams::for_range(date(2025, 1, 6), date(2025, 1, 12))
            .project("proj")
            .branches("main");
        let pairs = p.to_query_pairs();
        assert_eq!(pairs.len(), 4);
    }

    // ── cache_key ──────────────────────────────────────────────────────────────

    #[test]
    fn cache_key_no_project() {
        let p = SummaryParams::for_range(date(2025, 1, 6), date(2025, 1, 12));
        assert_eq!(p.cache_key(), "summaries:2025-01-06:2025-01-12");
    }

    #[test]
    fn cache_key_with_project() {
        let p = SummaryParams::for_range(date(2025, 1, 6), date(2025, 1, 12)).project("my-saas");
        assert_eq!(
            p.cache_key(),
            "summaries:2025-01-06:2025-01-12:project:my-saas"
        );
    }

    // ── StatsRange ─────────────────────────────────────────────────────────────

    #[test]
    fn stats_range_as_str_last_7_days() {
        assert_eq!(StatsRange::Last7Days.as_str(), "last_7_days");
    }

    #[test]
    fn stats_range_as_str_all_time() {
        assert_eq!(StatsRange::AllTime.as_str(), "all_time");
    }

    #[test]
    fn stats_range_all_variants_have_distinct_str() {
        use std::collections::HashSet;
        let variants = [
            StatsRange::Last7Days,
            StatsRange::Last30Days,
            StatsRange::Last6Months,
            StatsRange::LastYear,
            StatsRange::AllTime,
        ];
        let strs: HashSet<&str> = variants.iter().map(|v| v.as_str()).collect();
        assert_eq!(strs.len(), variants.len(), "all variants must be distinct");
    }
}
