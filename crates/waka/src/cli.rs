//! CLI argument definitions for `waka`.
//!
//! All commands are defined here using `clap` derive macros.
//! Individual command handlers live in `commands.rs`.

use clap::{Args, Parser, Subcommand, ValueEnum};

// ─── Top-level CLI ────────────────────────────────────────────────────────────

/// `WakaTime` CLI — track your coding time from the terminal.
#[derive(Debug, Parser)]
#[command(
    name = "waka",
    version,
    about,
    long_about = None,
    propagate_version = true,
    subcommand_required = true,
    arg_required_else_help = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub global: GlobalOpts,
}

// ─── Global options ───────────────────────────────────────────────────────────

/// Options available on every command (SPEC.md §6.2).
// Four bool flags are required by SPEC.md §6.2 — a state machine would obscure the CLI API.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, Args)]
pub struct GlobalOpts {
    /// Use a specific profile.
    #[arg(short = 'p', long, global = true, value_name = "PROFILE")]
    pub profile: Option<String>,

    /// Output format: table, json, csv, plain.
    #[arg(short = 'f', long, global = true, value_name = "FORMAT")]
    pub format: Option<OutputFormat>,

    /// Skip the cache and force a fresh API request.
    #[arg(long, global = true)]
    pub no_cache: bool,

    /// Disable colors (equivalent to `NO_COLOR=1`).
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Suppress non-essential output.
    #[arg(long, global = true)]
    pub quiet: bool,

    /// Enable verbose mode (shows HTTP requests).
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Prepend a UTF-8 BOM to CSV output (for Windows Excel compatibility).
    #[arg(long, global = true)]
    pub csv_bom: bool,
}

/// Output format for tabular commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
    Plain,
}

// ─── Command tree ─────────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage API key and authentication.
    Auth {
        // `WakaTime` keychain + env var priority
        #[command(subcommand)]
        cmd: AuthCommands,
    },

    /// Show coding statistics.
    Stats {
        #[command(subcommand)]
        cmd: StatsCommands,
    },

    /// Browse and filter projects.
    Projects {
        #[command(subcommand)]
        cmd: ProjectsCommands,
    },

    /// Browse coding languages.
    Languages {
        #[command(subcommand)]
        cmd: LanguagesCommands,
    },

    /// Browse editors and IDEs.
    Editors {
        #[command(subcommand)]
        cmd: EditorsCommands,
    },

    /// View and watch coding goals.
    Goals {
        #[command(subcommand)]
        cmd: GoalsCommands,
    },

    /// View the `WakaTime` leaderboard.
    Leaderboard {
        #[command(subcommand)]
        cmd: LeaderboardCommands,
    },

    /// Generate productivity reports.
    Report {
        #[command(subcommand)]
        cmd: ReportCommands,
    },

    /// Launch the interactive TUI dashboard.
    Dashboard(DashboardArgs),

    /// Shell prompt integration (reads from cache only, no network).
    Prompt(PromptArgs),

    /// Generate shell completions.
    Completions {
        /// Target shell.
        shell: CompletionShell,
    },

    /// Manage waka configuration.
    Config {
        #[command(subcommand)]
        cmd: ConfigCommands,
    },

    /// Manage the local response cache.
    Cache {
        #[command(subcommand)]
        cmd: CacheCommands,
    },

    /// Update waka to the latest version.
    Update,

    /// Show the changelog from the installed version to the latest.
    Changelog,
}

// ─── auth ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum AuthCommands {
    /// Log in with your `WakaTime` API key.
    Login(AuthLoginArgs),

    /// Remove the stored API key.
    Logout {
        /// Log out a specific profile.
        #[arg(long, value_name = "NAME")]
        profile: Option<String>,
    },

    /// Show whether you are currently logged in.
    Status,

    /// Display the stored API key (masked by default).
    ShowKey,

    /// Switch to a different profile.
    Switch {
        /// Profile name to activate.
        profile: String,
    },
}

#[derive(Debug, Args)]
pub struct AuthLoginArgs {
    /// Provide the API key directly (non-interactive).
    #[arg(long, value_name = "KEY")]
    pub api_key: Option<String>,

    /// Profile to store credentials under.
    #[arg(long, value_name = "NAME")]
    pub profile: Option<String>,
}

// ─── stats ────────────────────────────────────────────────────────────────────

/// Options shared by all stats subcommands.
#[derive(Debug, Args)]
pub struct StatsFilterOpts {
    /// Filter by project name.
    #[arg(long, value_name = "NAME")]
    pub project: Option<String>,

    /// Filter by language.
    #[arg(long, value_name = "LANG")]
    pub language: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum StatsCommands {
    /// Show today's coding activity.
    Today(StatsFilterOpts),

    /// Show yesterday's coding activity.
    Yesterday(StatsFilterOpts),

    /// Show the last 7 days of activity.
    Week(StatsFilterOpts),

    /// Show the last 30 days of activity.
    Month(StatsFilterOpts),

    /// Show the last 365 days of activity.
    Year(StatsFilterOpts),

    /// Show activity for a custom date range.
    Range {
        /// Start date (YYYY-MM-DD).
        #[arg(long, value_name = "DATE")]
        from: String,

        /// End date (YYYY-MM-DD).
        #[arg(long, value_name = "DATE")]
        to: String,

        #[command(flatten)]
        filter: StatsFilterOpts,
    },
}

// ─── projects ─────────────────────────────────────────────────────────────────

/// Sort field for project listings.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ProjectSortBy {
    Time,
    Name,
}

/// Time period for aggregated queries.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Period {
    #[value(name = "7d")]
    SevenDays,
    #[value(name = "30d")]
    ThirtyDays,
    #[value(name = "1y")]
    OneYear,
}

#[derive(Debug, Subcommand)]
pub enum ProjectsCommands {
    /// List all projects with coding time.
    List {
        /// Sort field.
        #[arg(long, value_name = "FIELD", default_value = "time")]
        sort_by: ProjectSortBy,

        /// Maximum number of results.
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
    },

    /// Show the most active projects.
    Top {
        /// Time period to aggregate over.
        #[arg(long, value_name = "PERIOD", default_value = "7d")]
        period: Period,
    },

    /// Show detailed stats for a project.
    Show {
        /// Project name.
        project_name: String,

        /// Start date (YYYY-MM-DD).
        #[arg(long, value_name = "DATE")]
        from: Option<String>,

        /// End date (YYYY-MM-DD).
        #[arg(long, value_name = "DATE")]
        to: Option<String>,
    },
}

// ─── languages ────────────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum LanguagesCommands {
    /// List all languages with coding time.
    List {
        /// Time period to aggregate over.
        #[arg(long, value_name = "PERIOD", default_value = "7d")]
        period: Period,
    },

    /// Show the top languages.
    Top {
        /// Maximum number of results.
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
    },
}

// ─── editors ──────────────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum EditorsCommands {
    /// List all editors with coding time.
    List {
        /// Time period to aggregate over.
        #[arg(long, value_name = "PERIOD", default_value = "7d")]
        period: Period,
    },

    /// Show the top editors.
    Top {
        /// Maximum number of results.
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
    },
}

// ─── goals ────────────────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum GoalsCommands {
    /// List all active goals.
    List,

    /// Show details for a specific goal.
    Show {
        /// Goal ID.
        goal_id: String,
    },

    /// Watch goals and refresh periodically.
    Watch {
        /// Send a desktop notification when a goal is reached.
        #[arg(long)]
        notify: bool,

        /// Refresh interval in seconds.
        #[arg(long, value_name = "SECONDS", default_value = "300")]
        interval: u64,
    },
}

// ─── leaderboard ──────────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum LeaderboardCommands {
    /// Show the public leaderboard.
    Show {
        /// Page number.
        #[arg(long, value_name = "N", default_value = "1")]
        page: u32,
    },
}

// ─── report ───────────────────────────────────────────────────────────────────

/// Output format for generated reports.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ReportFormat {
    Md,
    Html,
    Json,
    Csv,
}

/// Summary period.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SummaryPeriod {
    Week,
    Month,
}

#[derive(Debug, Subcommand)]
pub enum ReportCommands {
    /// Generate a productivity report for a date range.
    Generate {
        /// Start date (YYYY-MM-DD).
        #[arg(long, value_name = "DATE")]
        from: String,

        /// End date (YYYY-MM-DD).
        #[arg(long, value_name = "DATE")]
        to: String,

        /// Output file path.
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<std::path::PathBuf>,

        /// Report format.
        #[arg(short = 'f', long, value_name = "FORMAT", default_value = "md")]
        format: ReportFormat,
    },

    /// Show a brief productivity summary.
    Summary {
        /// Period to summarise.
        #[arg(long, value_name = "PERIOD", default_value = "week")]
        period: SummaryPeriod,
    },
}

// ─── dashboard ────────────────────────────────────────────────────────────────

/// Launch the interactive TUI dashboard.
#[derive(Debug, Args)]
pub struct DashboardArgs {
    /// Auto-refresh interval in seconds.
    #[arg(long, value_name = "SECONDS", default_value = "60")]
    pub refresh: u64,
}

// ─── prompt ───────────────────────────────────────────────────────────────────

/// Prompt format.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PromptStyle {
    Simple,
    Detailed,
}

/// Shell prompt integration.
#[derive(Debug, Args)]
pub struct PromptArgs {
    /// Output style.
    #[arg(long, value_name = "STYLE", default_value = "simple")]
    pub format: PromptStyle,
}

// ─── config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Get the value of a config key.
    Get {
        /// Config key (e.g. `core.profile`).
        key: String,
    },

    /// Set the value of a config key.
    Set {
        /// Config key.
        key: String,
        /// New value.
        value: String,
    },

    /// Open the config file in $EDITOR.
    Edit,

    /// Print the path to the config file.
    Path,

    /// Reset config to defaults.
    Reset {
        /// Skip the confirmation prompt.
        #[arg(long)]
        confirm: bool,
    },

    /// Run a full diagnostic check.
    Doctor,
}

// ─── cache ────────────────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum CacheCommands {
    /// Clear all cached entries (or only those older than a duration).
    Clear {
        /// Remove only entries older than this duration (e.g. `1h`, `24h`, `7d`).
        #[arg(long, value_name = "DURATION")]
        older: Option<String>,
    },

    /// Show cache statistics (entry count, disk size, last write).
    Info,

    /// Print the path to the cache directory.
    Path,
}

// ─── completions ──────────────────────────────────────────────────────────────

/// Shell for which to generate tab-completion scripts.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CompletionShell {
    /// Bash (append to `~/.bashrc`)
    Bash,
    /// Zsh (place in a directory on `$fpath`)
    Zsh,
    /// Fish (place in `~/.config/fish/completions/`)
    Fish,
    /// `PowerShell` (dot-source in `$PROFILE`)
    #[value(name = "powershell")]
    PowerShell,
    /// Elvish
    Elvish,
}
