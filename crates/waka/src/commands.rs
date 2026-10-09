//! Command handlers for `waka`.
//!
//! Each function corresponds to a leaf command in the CLI tree. Larger
//! commands live in submodules: [`report`] (report generation) and
//! [`config_cmd`] (`waka config …`, including `doctor`). Self-update and the
//! background update check live in [`crate::update`].

mod config_cmd;
mod report;

use std::collections::HashMap;
use std::io::IsTerminal as _;
use std::time::Duration;

use anyhow::{bail, Context as _, Result};
use chrono::{Local, NaiveDate};
use indicatif::ProgressBar;
use waka_api::{StatsRange, SummaryEntry, SummaryParams, WakaClient};
use waka_cache::CacheStore;
use waka_config::{Config, CredentialStore, ProfileConfig};
use waka_render::utils::{delimited_field, html_escape};
use waka_render::{
    detect_output_format, should_use_color, BreakdownRenderer, GoalRenderer, LeaderboardRenderer,
    OutputFormat as RenderFormat, ProjectRenderer, RenderOptions, SummaryRenderer,
};

use crate::auth;
use crate::cli::{
    AuthCommands, CacheCommands, Commands, CompletionShell, ConfigCommands, DashboardArgs,
    EditorsCommands, GlobalOpts, GoalsCommands, LanguagesCommands, LeaderboardCommands,
    OutputFormat as CliFormat, ProjectsCommands, PromptArgs, PromptStyle, ReportCommands,
    ReportFormat, StatsCommands, StatsFilterOpts, SummaryPeriod,
};
use crate::spinner::make_spinner;

/// Dispatch a parsed [`Commands`] variant to the appropriate handler.
///
/// `global` carries flags shared by every command (profile, format, color,
/// verbosity). It is passed down to handlers that need it.
pub async fn dispatch(cmd: Commands, global: GlobalOpts) -> Result<()> {
    // Prompt and Completions produce machine-readable output or are
    // latency-sensitive (shell prompt) — skip the update check entirely.
    let skip_update =
        global.quiet || matches!(&cmd, Commands::Prompt(_) | Commands::Completions { .. });

    // Spawn the update check concurrently with the command.
    // On a cache hit (most runs) it completes in microseconds.
    // On a cache miss (first run of the day) it fetches GitHub with up to
    // 5 s network timeout — we wait at most 3 s for it here.
    // Not spawned at all when skipped: it opens the sled cache, whose
    // exclusive lock would otherwise race with the command's own cache access.
    let update_handle = (!skip_update)
        .then(|| tokio::spawn(crate::update::update_check_background(global.clone())));

    let result = match cmd {
        Commands::Auth { cmd } => auth_cmd(cmd, global).await,
        Commands::Stats { cmd } => stats(cmd, &global).await,
        Commands::Projects { cmd } => projects(cmd, &global).await,
        Commands::Languages { cmd } => languages(cmd, &global).await,
        Commands::Editors { cmd } => editors(cmd, &global).await,
        Commands::Goals { cmd } => goals(cmd, &global).await,
        Commands::Leaderboard { cmd } => leaderboard(cmd, &global).await,
        Commands::Report { cmd } => report::report(cmd, &global).await,
        Commands::Dashboard(args) => dashboard(args, &global).await,
        Commands::Prompt(args) => {
            prompt(args, &global);
            Ok(())
        }
        Commands::Completions { shell } => {
            completions(shell);
            Ok(())
        }
        Commands::Config { cmd } => config_cmd::config(cmd, &global).await,
        Commands::Cache { cmd } => cache(cmd, &global),
        Commands::Update => crate::update::update_self(&global).await,
        Commands::Changelog => show_changelog(&global).await,
    };

    // After command output: wait briefly for the update notification.
    if let Some(handle) = update_handle {
        let _ = tokio::time::timeout(Duration::from_secs(3), handle).await;
    }

    result
}

// ─── auth ─────────────────────────────────────────────────────────────────────

async fn auth_cmd(cmd: AuthCommands, global: GlobalOpts) -> Result<()> {
    match cmd {
        AuthCommands::Login(args) => auth::login(args, &global).await,
        AuthCommands::Logout { profile } => auth::logout(profile, &global).await,
        AuthCommands::Status => auth::status(&global).await,
        AuthCommands::ShowKey => auth::show_key(&global).await,
        AuthCommands::Switch { profile } => auth::switch(&profile, &global).await,
    }
}

// ─── stats ────────────────────────────────────────────────────────────────────

/// Implements `waka stats today/yesterday/week/month/year/range`.
///
/// Loads the config, retrieves credentials, optionally hits the local cache,
/// fetches data from the `WakaTime` API, then renders the result.
async fn stats(cmd: StatsCommands, global: &GlobalOpts) -> Result<()> {
    // ── 1. Config, profile, client ────────────────────────────────────────────
    let config = load_config()?;
    let profile = resolve_profile(global, &config);
    let client = build_api_client(&profile, &config)?;

    // ── 3. Build params ───────────────────────────────────────────────────────
    let (params, label) = stats_build_params(cmd)?;
    let cache_key = params.cache_key();
    let ttl = Duration::from_secs(config.cache.ttl_seconds);

    // ── 4. Cache lookup ───────────────────────────────────────────────────────
    let cache_enabled = config.cache.enabled && !global.no_cache;
    let cache = if cache_enabled {
        CacheStore::open(&profile).ok()
    } else {
        None
    };

    let color = !global.no_color && should_use_color();

    // `(resp, footer_line)` — footer_line is shown below the table when set.
    let (resp, footer) = if let Some(ref c) = cache {
        match c.get::<waka_api::SummaryResponse>(&cache_key) {
            Ok(Some(entry)) if !entry.is_expired() => {
                // ── FRESH HIT ──────────────────────────────────────────────
                let age = entry.age_human();
                let indicator = if color {
                    console::style(format!("(cached {age})")).dim().to_string()
                } else {
                    format!("(cached {age})")
                };
                (entry.value, Some(indicator))
            }
            Ok(stale) => {
                // ── EXPIRED HIT or MISS — try network ─────────────────────
                let pb = stats_spinner(&format!("Fetching {label} stats …"));
                let result = client.summaries(params).await;
                pb.finish_and_clear();

                match result {
                    Ok(fresh) => {
                        // Store fresh data in cache.
                        let _ = c.set(&cache_key, &fresh, ttl);
                        (fresh, None)
                    }
                    Err(e) => {
                        if let Some(stale_entry) = stale {
                            // Network failed but we have stale data — show it.
                            let badge = if color {
                                console::style("⚠ offline (showing stale data)")
                                    .yellow()
                                    .to_string()
                            } else {
                                "⚠ offline (showing stale data)".to_owned()
                            };
                            (stale_entry.value, Some(badge))
                        } else {
                            // No stale data — propagate error.
                            return Err(e).with_context(|| {
                                format!("failed to fetch {label} stats from WakaTime")
                            });
                        }
                    }
                }
            }
            Err(_) => {
                // Cache read error — fall through to network.
                let pb = stats_spinner(&format!("Fetching {label} stats …"));
                let result = client.summaries(params).await;
                pb.finish_and_clear();
                let r = result
                    .with_context(|| format!("failed to fetch {label} stats from WakaTime"))?;
                let _ = c.set(&cache_key, &r, ttl);
                (r, None)
            }
        }
    } else {
        // ── CACHE DISABLED OR --no-cache ───────────────────────────────────
        let pb = stats_spinner(&format!("Fetching {label} stats …"));
        let result = client.summaries(params).await;
        pb.finish_and_clear();
        let r = result.with_context(|| format!("failed to fetch {label} stats from WakaTime"))?;
        (r, None)
    };

    // ── 5. Render ─────────────────────────────────────────────────────────────
    let format = stats_resolve_format(global, &config);
    // Convert possessive spinner label ("today's") into display form ("Today").
    let display_label = match label {
        "today's" => "Today",
        "yesterday's" => "Yesterday",
        "last 7 days'" => "Last 7 Days",
        "last 30 days'" => "Last 30 Days",
        "last 365 days'" => "Last 365 Days",
        other => other,
    };
    let opts = RenderOptions {
        color,
        format,
        csv_bom: global.csv_bom,
        period_label: Some(display_label.to_owned()),
        ..RenderOptions::default()
    };

    let output = SummaryRenderer::render(&resp, &opts);
    print!("{output}");

    if let Some(f) = footer {
        println!("{f}");
    }

    Ok(())
}

/// Converts a [`StatsCommands`] variant into a [`SummaryParams`] and a
/// human-readable label for spinner / error messages.
///
/// # Errors
///
/// Returns an error if the `range` subcommand dates cannot be parsed.
fn stats_build_params(cmd: StatsCommands) -> Result<(SummaryParams, &'static str)> {
    let today = Local::now().date_naive();

    match cmd {
        StatsCommands::Today(filters) => {
            let p = SummaryParams::today();
            Ok((stats_apply_filters(p, &filters), "today's"))
        }
        StatsCommands::Yesterday(filters) => {
            let yesterday = today
                .pred_opt()
                .context("cannot compute yesterday from the current date")?;
            let p = SummaryParams::for_range(yesterday, yesterday);
            Ok((stats_apply_filters(p, &filters), "yesterday's"))
        }
        StatsCommands::Week(filters) => {
            // Last 7 days (today inclusive).
            let start = today
                .checked_sub_days(chrono::Days::new(6))
                .context("cannot compute 7-day range")?;
            let p = SummaryParams::for_range(start, today);
            Ok((stats_apply_filters(p, &filters), "last 7 days'"))
        }
        StatsCommands::Month(filters) => {
            // Last 30 days (today inclusive).
            let start = today
                .checked_sub_days(chrono::Days::new(29))
                .context("cannot compute 30-day range")?;
            let p = SummaryParams::for_range(start, today);
            Ok((stats_apply_filters(p, &filters), "last 30 days'"))
        }
        StatsCommands::Year(filters) => {
            // Last 365 days (today inclusive).
            let start = today
                .checked_sub_days(chrono::Days::new(364))
                .context("cannot compute 365-day range")?;
            let p = SummaryParams::for_range(start, today);
            Ok((stats_apply_filters(p, &filters), "last 365 days'"))
        }
        StatsCommands::Range { from, to, filter } => {
            let start = NaiveDate::parse_from_str(&from, "%Y-%m-%d")
                .with_context(|| format!("--from must be YYYY-MM-DD, got '{from}'"))?;
            let end = NaiveDate::parse_from_str(&to, "%Y-%m-%d")
                .with_context(|| format!("--to must be YYYY-MM-DD, got '{to}'"))?;
            if end < start {
                bail!("--to ({to}) must be on or after --from ({from})");
            }
            let p = SummaryParams::for_range(start, end);
            Ok((stats_apply_filters(p, &filter), "custom range"))
        }
    }
}

/// Applies optional API-level filters to `params`.
///
/// The `--language` filter is not supported by the summaries endpoint at API
/// level; it is ignored with a warning on stderr.
// TODO(spec): the WakaTime summaries endpoint does not expose client-side
// language filtering. --language is reserved for post-filtering once SPEC.md
// §5.1 clarifies the intended behaviour.
fn stats_apply_filters(params: SummaryParams, filters: &StatsFilterOpts) -> SummaryParams {
    if filters.language.is_some() {
        eprintln!("warning: --language is not supported yet and was ignored");
    }
    if let Some(project) = &filters.project {
        params.project(project)
    } else {
        params
    }
}

/// Returns the effective [`RenderFormat`] for this invocation.
///
/// Priority: `--format` CLI flag > config `output.format` > `Table` default.
/// When stdout is not a TTY the format is coerced to `Plain` regardless.
fn stats_resolve_format(global: &GlobalOpts, config: &Config) -> RenderFormat {
    // If stdout is piped / redirected, degrade to plain text.
    detect_output_format(configured_format(global.format, &config.output.format))
}

/// Picks the requested format before TTY detection: an explicit `--format`
/// always wins (including `--format table`), otherwise `output.format`.
fn configured_format(cli: Option<CliFormat>, config: &waka_config::OutputFormat) -> RenderFormat {
    use waka_config::OutputFormat as CfgFmt;

    match cli {
        Some(CliFormat::Json) => RenderFormat::Json,
        Some(CliFormat::Csv) => RenderFormat::Csv,
        Some(CliFormat::Plain) => RenderFormat::Plain,
        Some(CliFormat::Table) => RenderFormat::Table,
        None => match config {
            CfgFmt::Json => RenderFormat::Json,
            CfgFmt::Csv => RenderFormat::Csv,
            CfgFmt::Plain => RenderFormat::Plain,
            CfgFmt::Tsv => RenderFormat::Tsv,
            CfgFmt::Table => RenderFormat::Table,
        },
    }
}

/// Creates an indeterminate progress spinner for network operations.
///
/// Hidden automatically when stderr is not a TTY.
fn stats_spinner(msg: &str) -> ProgressBar {
    make_spinner(msg)
}

// ─── shared API-client helpers ────────────────────────────────────────────────

/// Builds a [`WakaClient`] from the active profile's config and credentials.
///
/// Extracts the API URL from `config.profiles[profile]` (falling back to the
/// default) and retrieves the API key from the credential store.
///
/// # Errors
///
/// Returns an error if no API key is found or if the base URL is invalid.
fn build_api_client(profile: &str, config: &Config) -> Result<WakaClient> {
    let api_url = profile_api_url(config, profile);

    let store = CredentialStore::new(profile);
    let api_key = store.get_api_key().with_context(|| {
        format!(
            "No API key found for profile '{profile}'.\n\
             Run `waka auth login` to authenticate."
        )
    })?;

    WakaClient::with_base_url(api_key.expose(), &api_url)
        .with_context(|| format!("invalid api_url in profile '{profile}': {api_url}"))
}

/// Loads `config.toml`.
///
/// A malformed file is reported as an error rather than silently replaced by
/// defaults, which could otherwise be written back over the user's file.
///
/// # Errors
///
/// Returns an error if the config directory cannot be resolved or the file
/// cannot be read or parsed.
pub(crate) fn load_config() -> Result<Config> {
    let path =
        Config::path().map_or_else(|_| "config.toml".to_owned(), |p| p.display().to_string());
    Config::load().with_context(|| format!("could not load config file {path}"))
}

/// Resolves the active profile: `--profile` flag > `core.default_profile` >
/// `"default"` (the default value of `core.default_profile`).
pub(crate) fn resolve_profile(global: &GlobalOpts, config: &Config) -> String {
    global
        .profile
        .clone()
        .unwrap_or_else(|| config.core.default_profile.clone())
}

/// Returns the API base URL configured for `profile`, with a trailing slash so
/// relative endpoint paths join correctly.
pub(crate) fn profile_api_url(config: &Config, profile: &str) -> String {
    let url = config
        .profiles
        .get(profile)
        .map_or_else(|| ProfileConfig::default().api_url, |p| p.api_url.clone());
    if url.ends_with('/') {
        url
    } else {
        format!("{url}/")
    }
}

/// Converts a [`crate::cli::Period`] (CLI value) to its [`StatsRange`] equivalent.
#[must_use]
fn period_to_stats_range(period: crate::cli::Period) -> StatsRange {
    match period {
        crate::cli::Period::SevenDays => StatsRange::Last7Days,
        crate::cli::Period::ThirtyDays => StatsRange::Last30Days,
        crate::cli::Period::OneYear => StatsRange::LastYear,
    }
}

/// Converts [`SummaryEntry`] slices (already aggregated) to `(name, total_seconds)` pairs.
fn entries_from_stats(entries: &[SummaryEntry]) -> Vec<(String, f64)> {
    let mut result: Vec<(String, f64)> = entries
        .iter()
        .map(|e| (e.name.clone(), e.total_seconds))
        .collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    result
}

// ─── projects ─────────────────────────────────────────────────────────────────

/// Handles `waka projects {list,top,show}`.
async fn projects(cmd: ProjectsCommands, global: &GlobalOpts) -> Result<()> {
    let config = load_config()?;
    let profile = resolve_profile(global, &config);
    let client = build_api_client(&profile, &config)?;
    let format = stats_resolve_format(global, &config);
    let color = !global.no_color && should_use_color();
    let opts = RenderOptions {
        color,
        format,
        csv_bom: global.csv_bom,
        ..RenderOptions::default()
    };

    match cmd {
        ProjectsCommands::List { sort_by, limit } => {
            let pb = stats_spinner("Fetching projects …");
            let resp = client.projects().await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| "failed to fetch project list from WakaTime")?;

            let sort_by_name = matches!(sort_by, crate::cli::ProjectSortBy::Name);
            let output = ProjectRenderer::render_list(&resp, limit, sort_by_name, &opts);
            print!("{output}");
        }
        ProjectsCommands::Top { period } => {
            let range = period_to_stats_range(period);
            let pb = stats_spinner("Fetching top projects …");
            let resp = client.stats(range).await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| "failed to fetch top projects from WakaTime")?;

            let entries = entries_from_stats(&resp.data.projects);
            let output = BreakdownRenderer::render(&entries, "Project", Some(10), &opts);
            print!("{output}");
        }
        ProjectsCommands::Show {
            project_name,
            from,
            to,
        } => {
            // Resolve project name: use provided argument or interactive fuzzy
            // select (only when stdout is a TTY).
            let name: String = match project_name {
                Some(n) => n,
                None => {
                    if std::io::stdout().is_terminal() {
                        let pb = stats_spinner("Loading your projects…");
                        let projects_resp = client.projects().await;
                        pb.finish_and_clear();
                        let projects_resp = projects_resp
                            .with_context(|| "failed to fetch projects from WakaTime")?;
                        if projects_resp.data.is_empty() {
                            bail!("No projects found.");
                        }
                        let names: Vec<String> =
                            projects_resp.data.iter().map(|p| p.name.clone()).collect();
                        inquire::Select::new("Select a project:", names)
                            .prompt()
                            .with_context(|| "project selection cancelled")?
                    } else {
                        bail!("Specify a project name, e.g.: waka projects show my-project");
                    }
                }
            };

            let today = chrono::Local::now().date_naive();
            let start = from.as_deref().map_or(Ok(today), |s| {
                chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .with_context(|| format!("--from must be YYYY-MM-DD, got '{s}'"))
            })?;
            let end = to.as_deref().map_or(Ok(today), |s| {
                chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .with_context(|| format!("--to must be YYYY-MM-DD, got '{s}'"))
            })?;
            let params = SummaryParams::for_range(start, end).project(&name);
            let pb = stats_spinner(&format!("Fetching stats for '{name}' …"));
            let resp = client.summaries(params).await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| format!("failed to fetch stats for '{name}'"))?;
            print!("{}", SummaryRenderer::render(&resp, &opts));
        }
    }

    Ok(())
}

// ─── languages ────────────────────────────────────────────────────────────────

/// Handles `waka languages {list,top}`.
async fn languages(cmd: LanguagesCommands, global: &GlobalOpts) -> Result<()> {
    let config = load_config()?;
    let profile = resolve_profile(global, &config);
    let client = build_api_client(&profile, &config)?;
    let format = stats_resolve_format(global, &config);
    let color = !global.no_color && should_use_color();
    let opts = RenderOptions {
        color,
        format,
        csv_bom: global.csv_bom,
        ..RenderOptions::default()
    };

    match cmd {
        LanguagesCommands::List { period } => {
            let range = period_to_stats_range(period);
            let pb = stats_spinner("Fetching languages …");
            let resp = client.stats(range).await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| "failed to fetch language stats from WakaTime")?;

            let entries = entries_from_stats(&resp.data.languages);
            let output = BreakdownRenderer::render(&entries, "Language", None, &opts);
            print!("{output}");
        }
        LanguagesCommands::Top { limit } => {
            let pb = stats_spinner("Fetching top languages …");
            let resp = client.stats(StatsRange::Last7Days).await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| "failed to fetch language stats from WakaTime")?;

            let entries = entries_from_stats(&resp.data.languages);
            let output = BreakdownRenderer::render(&entries, "Language", limit.or(Some(10)), &opts);
            print!("{output}");
        }
    }

    Ok(())
}

// ─── editors ──────────────────────────────────────────────────────────────────

/// Handles `waka editors {list,top}`.
async fn editors(cmd: EditorsCommands, global: &GlobalOpts) -> Result<()> {
    let config = load_config()?;
    let profile = resolve_profile(global, &config);
    let client = build_api_client(&profile, &config)?;
    let format = stats_resolve_format(global, &config);
    let color = !global.no_color && should_use_color();
    let opts = RenderOptions {
        color,
        format,
        csv_bom: global.csv_bom,
        ..RenderOptions::default()
    };

    match cmd {
        EditorsCommands::List { period } => {
            let range = period_to_stats_range(period);
            let pb = stats_spinner("Fetching editors …");
            let resp = client.stats(range).await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| "failed to fetch editor stats from WakaTime")?;

            let entries = entries_from_stats(&resp.data.editors);
            let output = BreakdownRenderer::render(&entries, "Editor", None, &opts);
            print!("{output}");
        }
        EditorsCommands::Top { limit } => {
            let pb = stats_spinner("Fetching top editors …");
            let resp = client.stats(StatsRange::Last7Days).await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| "failed to fetch editor stats from WakaTime")?;

            let entries = entries_from_stats(&resp.data.editors);
            let output = BreakdownRenderer::render(&entries, "Editor", limit.or(Some(10)), &opts);
            print!("{output}");
        }
    }

    Ok(())
}

// ─── goals ────────────────────────────────────────────────────────────────────

async fn goals(cmd: GoalsCommands, global: &GlobalOpts) -> Result<()> {
    let config = load_config()?;
    let profile = resolve_profile(global, &config);
    let client = build_api_client(&profile, &config)?;
    let format = stats_resolve_format(global, &config);
    let color = !global.no_color && should_use_color();
    let opts = RenderOptions {
        color,
        format,
        csv_bom: global.csv_bom,
        ..RenderOptions::default()
    };

    match cmd {
        GoalsCommands::List => {
            let pb = stats_spinner("Fetching goals …");
            let resp = client.goals().await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| "failed to fetch goals from WakaTime")?;
            let output = GoalRenderer::render_list(&resp, &opts);
            print!("{output}");
        }
        GoalsCommands::Show { goal_id } => {
            let pb = stats_spinner("Fetching goals …");
            let resp = client.goals().await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| "failed to fetch goals from WakaTime")?;

            let goal = resp
                .data
                .iter()
                .find(|g| g.id == goal_id)
                .with_context(|| format!("goal '{goal_id}' not found"))?;

            let output = GoalRenderer::render_detail(goal, &opts);
            print!("{output}");
        }
        GoalsCommands::Watch { notify, interval } => {
            goals_watch(&client, global, &opts, notify, interval).await?;
        }
    }

    Ok(())
}

// ─── goals_watch helper ───────────────────────────────────────────────────────

/// Polls `client.goals()` every `interval` seconds, printing a refreshed table
/// to stdout on each tick.  Exits cleanly on Ctrl+C.
///
/// When `notify` is `true` and a goal transitions from a non-`"success"` status
/// to `"success"`, a desktop notification is sent via the system `notify-send`
/// binary (Linux/freedesktop).  The call fails silently when `notify-send` is
/// unavailable.
async fn goals_watch(
    client: &WakaClient,
    global: &GlobalOpts,
    opts: &RenderOptions,
    notify: bool,
    interval: u64,
) -> Result<()> {
    use std::io::Write as _;
    let is_tty = std::io::stdout().is_terminal();
    let interval_dur = Duration::from_secs(interval);

    // Map: goal_id → last known range_status.  Populated on first fetch.
    let mut prev_statuses: HashMap<String, String> = HashMap::new();

    let interval_display = if interval >= 60 {
        format!("{}m", interval / 60)
    } else {
        format!("{interval}s")
    };

    eprintln!("Watching goals… (refreshing every {interval_display}, Ctrl+C to stop)");

    loop {
        // ── fetch ─────────────────────────────────────────────────────────
        let resp = match client.goals().await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("waka: failed to fetch goals: {e}");
                // Wait before retrying rather than tight-looping on network error.
                tokio::select! {
                    () = tokio::time::sleep(interval_dur) => continue,
                    _ = tokio::signal::ctrl_c() => break,
                }
            }
        };

        // ── notifications ─────────────────────────────────────────────────
        if notify {
            for goal in &resp.data {
                let was = prev_statuses.get(&goal.id).map(String::as_str);
                if goal.range_status.as_deref() == Some("success")
                    && !matches!(was, Some("success"))
                {
                    goals_notify_success(&goal.title);
                }
            }
        }

        // Update tracked statuses.
        for goal in &resp.data {
            prev_statuses.insert(
                goal.id.clone(),
                goal.range_status.clone().unwrap_or_default(),
            );
        }

        // ── render ────────────────────────────────────────────────────────
        if is_tty && !global.quiet {
            // Clear screen + move cursor to top-left.
            print!("\x1b[2J\x1b[H");
        }

        let timestamp = Local::now().format("%H:%M");
        let header = if global.quiet {
            String::new()
        } else {
            format!(
                "[{timestamp}] Goals — refreshing every {interval_display} (Ctrl+C to stop)\n\n"
            )
        };

        let body = GoalRenderer::render_list(&resp, opts);
        print!("{header}{body}");

        // Flush stdout so the output appears immediately, even when piped.
        let _ = std::io::stdout().flush();

        // use std::io::Write as _; — moved to top of function

        // ── wait ──────────────────────────────────────────────────────────
        tokio::select! {
            () = tokio::time::sleep(interval_dur) => {},
            _ = tokio::signal::ctrl_c() => break,
        }
    }

    if !global.quiet {
        eprintln!("\nStopped watching goals.");
    }
    Ok(())
}

/// Sends a desktop notification via `notify-send` (silently ignored when
/// `notify-send` is not installed).
fn goals_notify_success(title: &str) {
    let _ = std::process::Command::new("notify-send")
        .arg("waka: Goal Reached! ✓")
        .arg(format!("{title} — target met"))
        .arg("--app-name=waka")
        .arg("--urgency=normal")
        .status();
}

// ─── leaderboard ──────────────────────────────────────────────────────────────

async fn leaderboard(cmd: LeaderboardCommands, global: &GlobalOpts) -> Result<()> {
    let config = load_config()?;
    let profile = resolve_profile(global, &config);
    let client = build_api_client(&profile, &config)?;
    let format = stats_resolve_format(global, &config);
    let color = !global.no_color && should_use_color();
    let opts = RenderOptions {
        color,
        format,
        csv_bom: global.csv_bom,
        ..RenderOptions::default()
    };

    match cmd {
        LeaderboardCommands::Show { page } => {
            let pb = stats_spinner("Fetching leaderboard …");
            let resp = client.leaderboard(page).await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| "failed to fetch leaderboard from WakaTime")?;
            let output = LeaderboardRenderer::render(&resp, &opts);
            print!("{output}");
        }
    }

    Ok(())
}

// ─── dashboard ────────────────────────────────────────────────────────────────

async fn dashboard(args: DashboardArgs, global: &GlobalOpts) -> Result<()> {
    let config = load_config()?;
    let profile = resolve_profile(global, &config);
    let client = build_api_client(&profile, &config)?;
    let refresh_interval = std::time::Duration::from_secs(args.refresh);

    waka_tui::run(client, refresh_interval)
        .await
        .with_context(|| "failed to run TUI dashboard")?;

    Ok(())
}

// ─── prompt ───────────────────────────────────────────────────────────────────

/// Implements `waka prompt`.
///
/// Reads today's total coding time from the local cache and prints a compact
/// string suitable for embedding in a shell prompt or tmux status bar.
///
/// **Never returns an error** — any failure (cache miss, corrupted entry)
/// results in empty output so that the caller's prompt is never
/// broken. The operation is cache-only: no network request is ever made.
///
/// # Output formats
///
/// | `--format` | Example output |
/// |---|---|
/// | `simple`   | `⏱ 6h 42m` |
/// | `detailed` | `⏱ 6h 42m \| my-saas` |
// `needless_pass_by_value`: PromptArgs is a simple options struct — no heap allocation.
#[allow(clippy::needless_pass_by_value)]
fn prompt(args: PromptArgs, global: &GlobalOpts) {
    // All errors are swallowed — a broken prompt is never acceptable.
    if let Some(output) = prompt_inner(&args, global) {
        println!("{output}");
    }
}

/// Formats a prompt output string from pre-computed values.
///
/// Extracted as a pure function for testability.
fn format_prompt_output(total_secs: u64, style: PromptStyle, top_project: Option<&str>) -> String {
    let hours = total_secs / 3_600;
    let mins = (total_secs % 3_600) / 60;

    let time_str = if hours > 0 {
        format!("\u{23F1} {hours}h {mins}m")
    } else {
        format!("\u{23F1} {mins}m")
    };

    match style {
        PromptStyle::Simple => time_str,
        PromptStyle::Detailed => match top_project {
            Some(proj) => format!("{time_str} | {proj}"),
            None => time_str,
        },
    }
}

/// Core logic for [`prompt`]. Returns `None` on any failure (cache miss,
/// I/O error).  The 100ms budget is inherently satisfied because this
/// function only reads one small cache file (no network I/O).
fn prompt_inner(args: &PromptArgs, global: &GlobalOpts) -> Option<String> {
    // The prompt must never print errors, so a broken config falls back to
    // defaults here (read-only: nothing is written back).
    let config = Config::load().unwrap_or_default();
    let profile = resolve_profile(global, &config);

    // Open the cache — silently skip on failure.
    let store = CacheStore::open(&profile).ok()?;

    // Build the same cache key that `waka stats today` writes.
    let cache_key = SummaryParams::today().cache_key();

    // Retrieve the entry. A miss → silent empty output.
    //
    // The TTL is deliberately ignored: it governs when `stats` refetches, but
    // the key is scoped to today's date, so an older entry is still today's
    // total as of its last fetch. Honouring the 5-minute TTL left the prompt
    // empty almost all the time.
    let entry = store
        .get::<waka_api::SummaryResponse>(&cache_key)
        .ok()
        .flatten()?;

    let response = &entry.value;

    // Sum grand totals across all days using integer fields to avoid f64→u64 casts.
    // (Today is always a single day, but be defensive for multi-day ranges.)
    let total_secs: u64 = response
        .data
        .iter()
        .map(|d| {
            u64::from(d.grand_total.hours) * 3_600
                + u64::from(d.grand_total.minutes) * 60
                + u64::from(d.grand_total.seconds)
        })
        .sum();

    if total_secs == 0 {
        return None;
    }

    // Find the top project by total_seconds across all days.
    let top_project: Option<String> = {
        let mut acc = std::collections::HashMap::<&str, f64>::new();
        for day in &response.data {
            for p in &day.projects {
                *acc.entry(p.name.as_str()).or_insert(0.0) += p.total_seconds;
            }
        }
        acc.into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(name, _)| name.to_owned())
    };

    Some(format_prompt_output(
        total_secs,
        args.style,
        top_project.as_deref(),
    ))
}

// ─── completions ──────────────────────────────────────────────────────────────

/// Implements `waka completions <shell>`.
///
/// Generates tab-completion scripts for the given shell and prints them to
/// stdout. The user pipes the output into the appropriate installation path.
// `needless_pass_by_value`: shell is a Copy type; kept for API consistency.
#[allow(clippy::needless_pass_by_value)]
fn completions(shell: CompletionShell) {
    use clap::CommandFactory as _;
    use clap_complete::{generate, shells};

    let mut cmd = crate::cli::Cli::command();
    let name = cmd.get_name().to_owned();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    match shell {
        CompletionShell::Bash => generate(shells::Bash, &mut cmd, &name, &mut out),
        CompletionShell::Zsh => generate(shells::Zsh, &mut cmd, &name, &mut out),
        CompletionShell::Fish => generate(shells::Fish, &mut cmd, &name, &mut out),
        CompletionShell::PowerShell => generate(shells::PowerShell, &mut cmd, &name, &mut out),
        CompletionShell::Elvish => generate(shells::Elvish, &mut cmd, &name, &mut out),
    }
}

// ─── update / changelog ───────────────────────────────────────────────────────

/// Implements `waka changelog`.
///
/// Fetches the latest CHANGELOG.md from the GitHub repository and displays
/// the entries from the installed version onwards.
async fn show_changelog(global: &GlobalOpts) -> Result<()> {
    let pb = stats_spinner("Fetching changelog...");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent(concat!("waka/", env!("CARGO_PKG_VERSION")))
        .build()?;

    // Fetch CHANGELOG.md from GitHub (raw content).
    let url = "https://raw.githubusercontent.com/mouwaficbdr/waka/main/CHANGELOG.md";

    let resp = client.get(url).send().await?;
    pb.finish_and_clear();

    if !resp.status().is_success() {
        bail!(
            "Could not fetch changelog (HTTP {})\n\
             View it online: https://github.com/mouwaficbdr/waka/blob/main/CHANGELOG.md",
            resp.status()
        );
    }

    let content = resp.text().await?;
    let current = env!("CARGO_PKG_VERSION");

    // Find the section that corresponds to the current version and print from
    // the latest entry down to (but not including) the installed version.
    let mut found_newer = false;
    let mut found_current = false;
    let mut output = Vec::new();

    for line in content.lines() {
        // Markdown headings like `## [0.4.0] - 2025-...`
        if let Some(rest) = line.strip_prefix("## [") {
            let ver = rest.split(']').next().unwrap_or("").trim();
            if ver == current {
                found_current = true;
                break; // stop before including the installed version section
            }
            found_newer = true;
        }
        if found_newer {
            output.push(line);
        }
    }

    if !found_newer {
        if found_current {
            println!("  ✓  You are on the latest released version (v{current}).");
        } else {
            // Fallback: print the full changelog if we can't detect version.
            if !global.quiet {
                println!("  ℹ  Could not detect version in changelog. Displaying full content:\n");
            }
            print!("{content}");
        }
        return Ok(());
    }

    // Respect PAGER env var (like `less`).
    let pager = std::env::var("PAGER").ok();
    if let Some(pager_cmd) = pager.as_deref().filter(|p| !p.is_empty()) {
        // Write to pager via stdin.
        let mut child = std::process::Command::new(pager_cmd)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .context("failed to launch pager")?;

        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write as _;
            for line in &output {
                let _ = writeln!(stdin, "{line}");
            }
        }
        let _ = child.wait();
    } else {
        for line in &output {
            println!("{line}");
        }
    }

    Ok(())
}

// ─── cache ────────────────────────────────────────────────────────────────────

/// Parses a human-friendly duration string (e.g. `"1h"`, `"24h"`, `"7d"`) into
/// a [`Duration`].
///
/// Supported suffixes: `s` (seconds), `m` (minutes), `h` (hours), `d` (days).
///
/// # Errors
///
/// Returns an error if the string is malformed or the numeric part overflows.
fn parse_duration(s: &str) -> Result<Duration> {
    let s = s.trim();
    if s.is_empty() {
        bail!("duration string must not be empty");
    }

    let (num_str, unit) = if let Some(n) = s.strip_suffix('s') {
        (n, "s")
    } else if let Some(n) = s.strip_suffix('m') {
        (n, "m")
    } else if let Some(n) = s.strip_suffix('h') {
        (n, "h")
    } else if let Some(n) = s.strip_suffix('d') {
        (n, "d")
    } else {
        bail!(
            "unrecognised duration '{s}': expected a number followed by s/m/h/d (e.g. 1h, 24h, 7d)"
        );
    };

    let n: u64 = num_str
        .trim()
        .parse()
        .with_context(|| format!("invalid number in duration '{s}'"))?;

    let secs = match unit {
        "s" => n,
        "m" => n.saturating_mul(60),
        "h" => n.saturating_mul(3_600),
        "d" => n.saturating_mul(86_400),
        _ => unreachable!(),
    };
    Ok(Duration::from_secs(secs))
}

/// Implements `waka cache clear / info / path`.
// `needless_pass_by_value`: cmd is consumed by the match; GlobalOpts is needed for quiet/profile.
#[allow(clippy::needless_pass_by_value)]
fn cache(cmd: CacheCommands, global: &GlobalOpts) -> Result<()> {
    let config = load_config()?;
    let profile = resolve_profile(global, &config);
    let profile = profile.as_str();

    match cmd {
        CacheCommands::Clear { older } => {
            let store = CacheStore::open(profile)
                .with_context(|| format!("failed to open cache for profile '{profile}'"))?;

            let removed = if let Some(ref dur_str) = older {
                let dur = parse_duration(dur_str)
                    .with_context(|| format!("invalid --older value '{dur_str}'"))?;
                store
                    .clear_older_than(dur)
                    .context("failed to clear old cache entries")?
            } else {
                store.clear().context("failed to clear cache")?
            };

            if !global.quiet {
                if removed == 1 {
                    println!("Removed 1 cache entry.");
                } else {
                    println!("Removed {removed} cache entries.");
                }
            }
            Ok(())
        }

        CacheCommands::Info => {
            let store = CacheStore::open(profile)
                .with_context(|| format!("failed to open cache for profile '{profile}'"))?;
            let info = store.info();

            let size_human = format_bytes(info.size_on_disk);
            let last_write = info.last_write.map_or_else(
                || "never".to_owned(),
                |dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            );

            println!("Profile:     {profile}");
            println!("Entries:     {}", info.entry_count);
            println!("Disk usage:  {size_human}");
            println!("Last write:  {last_write}");
            println!(
                "Path:        {}",
                CacheStore::db_path(profile)
                    .map_or_else(|_| "<unavailable>".to_owned(), |p| p.display().to_string())
            );
            Ok(())
        }

        CacheCommands::Path => {
            let path = CacheStore::db_path(profile)
                .with_context(|| "could not determine cache directory")?;
            println!("{}", path.display());
            Ok(())
        }
    }
}

/// Formats a byte count into a human-readable string (e.g. `1.2 MB`).
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1_024;
    const MB: u64 = KB * 1_024;
    const GB: u64 = MB * 1_024;

    #[allow(clippy::cast_precision_loss)]
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

// ─── unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::StatsFilterOpts;

    fn no_filters() -> StatsFilterOpts {
        StatsFilterOpts {
            project: None,
            language: None,
        }
    }

    // ── stats_build_params ────────────────────────────────────────────────────

    #[test]
    fn build_params_today_label() {
        let (_, label) = stats_build_params(StatsCommands::Today(no_filters()))
            .expect("today params must be buildable");
        assert_eq!(label, "today's");
    }

    #[test]
    fn build_params_yesterday_label() {
        let (_, label) = stats_build_params(StatsCommands::Yesterday(no_filters()))
            .expect("yesterday params must be buildable");
        assert_eq!(label, "yesterday's");
    }

    #[test]
    fn build_params_week_label() {
        let (_, label) = stats_build_params(StatsCommands::Week(no_filters()))
            .expect("week params must be buildable");
        assert_eq!(label, "last 7 days'");
    }

    #[test]
    fn build_params_month_label() {
        let (_, label) = stats_build_params(StatsCommands::Month(no_filters()))
            .expect("month params must be buildable");
        assert_eq!(label, "last 30 days'");
    }

    #[test]
    fn build_params_year_label() {
        let (_, label) = stats_build_params(StatsCommands::Year(no_filters()))
            .expect("year params must be buildable");
        assert_eq!(label, "last 365 days'");
    }

    #[test]
    fn build_params_range_valid() {
        let (_, label) = stats_build_params(StatsCommands::Range {
            from: "2024-01-01".to_owned(),
            to: "2024-01-07".to_owned(),
            filter: no_filters(),
        })
        .expect("valid range must succeed");
        assert_eq!(label, "custom range");
    }

    #[test]
    fn build_params_range_end_before_start_errors() {
        let err = stats_build_params(StatsCommands::Range {
            from: "2024-01-07".to_owned(),
            to: "2024-01-01".to_owned(),
            filter: no_filters(),
        })
        .expect_err("end before start must fail");
        assert!(err.to_string().contains("--to"), "error must mention --to");
    }

    #[test]
    fn build_params_range_invalid_date_format_errors() {
        let err = stats_build_params(StatsCommands::Range {
            from: "not-a-date".to_owned(),
            to: "2024-01-07".to_owned(),
            filter: no_filters(),
        })
        .expect_err("invalid date must fail");
        assert!(err.to_string().contains("YYYY-MM-DD"));
    }

    // ── configured_format ─────────────────────────────────────────────────────

    #[test]
    fn explicit_table_flag_overrides_config_format() {
        assert_eq!(
            configured_format(Some(CliFormat::Table), &waka_config::OutputFormat::Json),
            RenderFormat::Table
        );
    }

    #[test]
    fn config_format_used_without_flag() {
        assert_eq!(
            configured_format(None, &waka_config::OutputFormat::Tsv),
            RenderFormat::Tsv
        );
    }

    // ── resolve_profile ───────────────────────────────────────────────────────

    #[test]
    fn profile_defaults_to_config_default_profile() {
        let global = GlobalOpts {
            profile: None,
            ..GlobalOpts::default()
        };
        assert_eq!(resolve_profile(&global, &Config::default()), "default");

        let mut config = Config::default();
        config.core.default_profile = "work".to_owned();
        assert_eq!(resolve_profile(&global, &config), "work");
    }

    #[test]
    fn profile_flag_overrides_config_default_profile() {
        let global = GlobalOpts {
            profile: Some("personal".to_owned()),
            ..GlobalOpts::default()
        };
        let mut config = Config::default();
        config.core.default_profile = "work".to_owned();
        assert_eq!(resolve_profile(&global, &config), "personal");
    }

    #[test]
    fn profile_api_url_adds_trailing_slash() {
        let mut config = Config::default();
        config
            .profiles
            .entry("self".to_owned())
            .or_default()
            .api_url = "https://wakapi.example.com/api/compat/wakatime/v1".to_owned();
        assert_eq!(
            profile_api_url(&config, "self"),
            "https://wakapi.example.com/api/compat/wakatime/v1/"
        );
        assert!(profile_api_url(&config, "missing").ends_with('/'));
    }

    // ── format_prompt_output ──────────────────────────────────────────────────

    #[test]
    fn prompt_simple_hours_and_minutes() {
        let out = format_prompt_output(6 * 3_600 + 42 * 60, PromptStyle::Simple, None);
        assert_eq!(out, "⏱ 6h 42m");
    }

    #[test]
    fn prompt_simple_minutes_only() {
        let out = format_prompt_output(42 * 60, PromptStyle::Simple, None);
        assert_eq!(out, "⏱ 42m");
    }

    #[test]
    fn prompt_simple_zero_minutes() {
        let out = format_prompt_output(5, PromptStyle::Simple, None);
        assert_eq!(out, "⏱ 0m");
    }

    #[test]
    fn prompt_detailed_with_project() {
        let out = format_prompt_output(6 * 3_600 + 42 * 60, PromptStyle::Detailed, Some("my-saas"));
        assert_eq!(out, "⏱ 6h 42m | my-saas");
    }

    #[test]
    fn prompt_detailed_without_project_falls_back_to_simple() {
        let out = format_prompt_output(6 * 3_600 + 42 * 60, PromptStyle::Detailed, None);
        assert_eq!(out, "⏱ 6h 42m");
    }

    #[test]
    fn prompt_exactly_one_hour() {
        let out = format_prompt_output(3_600, PromptStyle::Simple, None);
        assert_eq!(out, "⏱ 1h 0m");
    }

    // ── parse_duration ────────────────────────────────────────────────────────

    #[test]
    fn parse_duration_seconds() {
        assert_eq!(parse_duration("30s").unwrap(), Duration::from_secs(30));
    }

    #[test]
    fn parse_duration_minutes() {
        assert_eq!(parse_duration("5m").unwrap(), Duration::from_secs(300));
    }

    #[test]
    fn parse_duration_hours() {
        assert_eq!(parse_duration("2h").unwrap(), Duration::from_secs(7_200));
    }

    #[test]
    fn parse_duration_days() {
        assert_eq!(parse_duration("7d").unwrap(), Duration::from_secs(604_800));
    }

    #[test]
    fn parse_duration_rejects_empty() {
        assert!(parse_duration("").is_err());
    }

    #[test]
    fn parse_duration_rejects_unknown_suffix() {
        assert!(parse_duration("10x").is_err());
    }

    #[test]
    fn parse_duration_rejects_non_numeric() {
        assert!(parse_duration("abch").is_err());
    }

    // ── format_bytes ──────────────────────────────────────────────────────────

    #[test]
    fn format_bytes_zero() {
        assert_eq!(format_bytes(0), "0 B");
    }

    #[test]
    fn format_bytes_bytes() {
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn format_bytes_kilobytes() {
        assert_eq!(format_bytes(2_048), "2.0 KB");
    }

    #[test]
    fn format_bytes_megabytes() {
        assert_eq!(format_bytes(5 * 1_024 * 1_024), "5.0 MB");
    }

    #[test]
    fn format_bytes_gigabytes() {
        assert_eq!(format_bytes(2 * 1_024 * 1_024 * 1_024), "2.0 GB");
    }
}
