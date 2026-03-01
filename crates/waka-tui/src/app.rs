//! Application state for the TUI dashboard.

use std::time::{Duration, Instant};

use waka_api::{GoalsResponse, LeaderboardResponse, SummaryResponse, WakaClient};

/// The active view in the TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// Main dashboard (default).
    Main,
    /// Projects detail view.
    Projects,
    /// Languages breakdown.
    Languages,
    /// Goals detail view.
    Goals,
    /// Activity calendar (GitHub-style heatmap).
    Activity,
}

impl View {
    /// Returns the numeric ID of the view (used for Tab switching).
    pub const fn id(self) -> usize {
        match self {
            Self::Main => 1,
            Self::Projects => 2,
            Self::Languages => 3,
            Self::Goals => 4,
            Self::Activity => 5,
        }
    }

    /// Returns the next view in the rotation.
    pub const fn next(self) -> Self {
        match self {
            Self::Main => Self::Projects,
            Self::Projects => Self::Languages,
            Self::Languages => Self::Goals,
            Self::Goals => Self::Activity,
            Self::Activity => Self::Main,
        }
    }
}

/// Application state for the TUI dashboard.
pub struct App {
    /// The `WakaTime` API client.
    pub client: WakaClient,
    /// Current view.
    pub view: View,
    /// Whether the app is running.
    pub running: bool,
    /// Last successful data fetch timestamp.
    pub last_update: Option<Instant>,
    /// Auto-refresh interval.
    pub refresh_interval: Duration,
    /// Main summary data (today).
    pub summary_today: Option<SummaryResponse>,
    /// Weekly summary data (last 7 days, one entry per day).
    pub summary_week: Option<SummaryResponse>,
    /// 30-day activity history (one entry per day).
    pub activity_30d: Option<SummaryResponse>,
    /// Goals data.
    pub goals: Option<GoalsResponse>,
    /// Leaderboard data (optional — fetched lazily).
    pub leaderboard: Option<LeaderboardResponse>,
    /// Last error message (if any).
    pub error: Option<String>,
    /// Whether a background fetch is in progress.
    pub loading: bool,
    /// Whether the help popup is visible.
    pub show_help: bool,
    /// List selection index (for navigating projects/languages/goals).
    pub list_index: usize,
}

impl App {
    /// Creates a new `App` with the given client and refresh interval.
    pub fn new(client: WakaClient, refresh_interval: Duration) -> Self {
        Self {
            client,
            view: View::Main,
            running: true,
            last_update: None,
            refresh_interval,
            summary_today: None,
            summary_week: None,
            activity_30d: None,
            goals: None,
            leaderboard: None,
            error: None,
            loading: false,
            show_help: false,
            list_index: 0,
        }
    }

    /// Marks the app for shutdown.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Toggles the help popup.
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Switches to the next view (Tab key).
    pub fn next_view(&mut self) {
        self.view = self.view.next();
        self.list_index = 0;
    }

    /// Moves the list selection up.
    pub fn list_up(&mut self) {
        if self.list_index > 0 {
            self.list_index -= 1;
        }
    }

    /// Moves the list selection down.
    pub fn list_down(&mut self, max: usize) {
        if self.list_index + 1 < max {
            self.list_index += 1;
        }
    }

    /// Resets the list selection.
    pub fn reset_list(&mut self) {
        self.list_index = 0;
    }

    /// Returns the time remaining until the next auto-refresh.
    ///
    /// If no data has been fetched yet, returns `Duration::ZERO`.
    pub fn time_until_refresh(&self) -> Duration {
        self.last_update.map_or(Duration::ZERO, |last| {
            let elapsed = last.elapsed();
            self.refresh_interval.saturating_sub(elapsed)
        })
    }
}
