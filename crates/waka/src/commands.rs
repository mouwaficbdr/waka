//! Command handlers for `waka`.
//!
//! Each function corresponds to a leaf command in the CLI tree. Auth handlers
//! and the stats handler are fully implemented; all others remain stubs to be
//! filled in during later phases.

use std::collections::HashMap;
use std::io::IsTerminal as _;
use std::time::Duration;

use anyhow::{bail, Context as _, Result};
use chrono::{Local, NaiveDate};
use indicatif::ProgressBar;
use waka_api::{StatsRange, SummaryEntry, SummaryParams, WakaClient};
use waka_cache::CacheStore;
use waka_config::{Config, CredentialStore, ProfileConfig};
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
    let update_handle = tokio::spawn(update_check_background(global.clone()));

    let result = match cmd {
        Commands::Auth { cmd } => auth_cmd(cmd, global).await,
        Commands::Stats { cmd } => stats(cmd, &global).await,
        Commands::Projects { cmd } => projects(cmd, &global).await,
        Commands::Languages { cmd } => languages(cmd, &global).await,
        Commands::Editors { cmd } => editors(cmd, &global).await,
        Commands::Goals { cmd } => goals(cmd, &global).await,
        Commands::Leaderboard { cmd } => leaderboard(cmd, &global).await,
        Commands::Report { cmd } => report(cmd, &global).await,
        Commands::Dashboard(args) => dashboard(args, &global).await,
        Commands::Prompt(args) => {
            prompt(args, &global);
            Ok(())
        }
        Commands::Completions { shell } => {
            completions(shell);
            Ok(())
        }
        Commands::Config { cmd } => config(cmd, &global).await,
        Commands::Cache { cmd } => cache(cmd, &global),
        Commands::Update => update_self(&global).await,
        Commands::Changelog => show_changelog(&global).await,
    };

    // After command output: wait briefly for the update notification.
    if !skip_update {
        let _ = tokio::time::timeout(Duration::from_secs(3), update_handle).await;
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
    // ── 1. Config + credentials ───────────────────────────────────────────────
    let config = Config::load().unwrap_or_default();
    let profile = stats_profile_name(global);
    let api_url = config
        .profiles
        .get(&profile)
        .map_or_else(|| ProfileConfig::default().api_url, |p| p.api_url.clone());

    let store = CredentialStore::new(&profile);
    let api_key = store.get_api_key().with_context(|| {
        format!(
            "No API key found for profile '{profile}'.\n\
             Run `waka auth login` to authenticate."
        )
    })?;

    // ── 2. Build client ───────────────────────────────────────────────────────
    let api_url_normalized = if api_url.ends_with('/') {
        api_url.clone()
    } else {
        format!("{api_url}/")
    };
    let client = WakaClient::with_base_url(api_key.expose(), &api_url_normalized)
        .with_context(|| format!("invalid api_url in profile '{profile}': {api_url}"))?;

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

/// Extracts the active profile name from [`GlobalOpts`] or returns `"default"`.
fn stats_profile_name(global: &GlobalOpts) -> String {
    global.profile.as_deref().unwrap_or("default").to_owned()
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
/// level and is silently ignored.
// TODO(spec): the WakaTime summaries endpoint does not expose client-side
// language filtering. --language is reserved for post-filtering once SPEC.md
// §5.1 clarifies the intended behaviour.
fn stats_apply_filters(params: SummaryParams, filters: &StatsFilterOpts) -> SummaryParams {
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
    use waka_config::OutputFormat as CfgFmt;

    // CLI flag takes precedence over config.
    let configured = match global.format {
        Some(CliFormat::Json) => RenderFormat::Json,
        Some(CliFormat::Csv) => RenderFormat::Csv,
        Some(CliFormat::Plain) => RenderFormat::Plain,
        Some(CliFormat::Table) | None => match config.output.format {
            CfgFmt::Json => RenderFormat::Json,
            CfgFmt::Csv => RenderFormat::Csv,
            CfgFmt::Plain => RenderFormat::Plain,
            CfgFmt::Tsv => RenderFormat::Tsv,
            CfgFmt::Table => RenderFormat::Table,
        },
    };

    // If stdout is piped / redirected, degrade to plain text.
    detect_output_format(configured)
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
    let api_url = config
        .profiles
        .get(profile)
        .map_or_else(|| ProfileConfig::default().api_url, |p| p.api_url.clone());

    let store = CredentialStore::new(profile);
    let api_key = store.get_api_key().with_context(|| {
        format!(
            "No API key found for profile '{profile}'.\n\
             Run `waka auth login` to authenticate."
        )
    })?;

    let normalized = if api_url.ends_with('/') {
        api_url.clone()
    } else {
        format!("{api_url}/")
    };
    WakaClient::with_base_url(api_key.expose(), &normalized)
        .with_context(|| format!("invalid api_url in profile '{profile}': {api_url}"))
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
    let config = Config::load().unwrap_or_default();
    let profile = stats_profile_name(global);
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
    let config = Config::load().unwrap_or_default();
    let profile = stats_profile_name(global);
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
    let config = Config::load().unwrap_or_default();
    let profile = stats_profile_name(global);
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
    let config = Config::load().unwrap_or_default();
    let profile = stats_profile_name(global);
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
    let config = Config::load().unwrap_or_default();
    let profile = stats_profile_name(global);
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

// ─── report ───────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd is consumed by the match for exhaustive checking.
#[allow(clippy::needless_pass_by_value)]
async fn report(cmd: ReportCommands, global: &GlobalOpts) -> Result<()> {
    match cmd {
        ReportCommands::Generate {
            from,
            to,
            output,
            output_format,
        } => report_generate(from, to, output, output_format, global).await,
        ReportCommands::Summary { period } => report_summary(period, global).await,
    }
}

/// Generates a productivity report for a date range.
async fn report_generate(
    from: String,
    to: String,
    output: Option<std::path::PathBuf>,
    format: ReportFormat,
    global: &GlobalOpts,
) -> Result<()> {
    use chrono::NaiveDate;

    // Parse dates
    let start_date = NaiveDate::parse_from_str(&from, "%Y-%m-%d")
        .with_context(|| format!("invalid start date '{from}' (expected YYYY-MM-DD)"))?;
    let end_date = NaiveDate::parse_from_str(&to, "%Y-%m-%d")
        .with_context(|| format!("invalid end date '{to}' (expected YYYY-MM-DD)"))?;

    if end_date < start_date {
        bail!("end date must be after start date");
    }

    // Build client
    let config = Config::load().unwrap_or_default();
    let profile = stats_profile_name(global);
    let client = build_api_client(&profile, &config)?;

    // Fetch data for the period
    let pb = stats_spinner("Fetching data for report...");
    let params = waka_api::SummaryParams::for_range(start_date, end_date);
    let summary = client.summaries(params).await?;

    // Fetch goals (may fail if user has no goals - that's ok)
    let goals = client.goals().await.ok();

    pb.finish_and_clear();

    // Generate report content
    let content = match format {
        ReportFormat::Md => {
            generate_report_markdown(&summary, goals.as_ref(), start_date, end_date)
        }
        ReportFormat::Html => generate_report_html(&summary, goals.as_ref(), start_date, end_date),
        ReportFormat::Json => generate_report_json(&summary, goals.as_ref(), start_date, end_date)?,
        ReportFormat::Csv => generate_report_csv(&summary, start_date, end_date),
    };

    // Write output
    if let Some(path) = output {
        std::fs::write(&path, content)
            .with_context(|| format!("failed to write report to {}", path.display()))?;
        if !global.quiet {
            eprintln!("Report written to {}", path.display());
        }
    } else {
        print!("{content}");
    }

    Ok(())
}

/// Shows a brief productivity summary.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
async fn report_summary(period: SummaryPeriod, global: &GlobalOpts) -> Result<()> {
    let config = Config::load().unwrap_or_default();
    let profile = stats_profile_name(global);
    let client = build_api_client(&profile, &config)?;

    let today = chrono::Local::now().date_naive();
    let params = match period {
        SummaryPeriod::Week => {
            let start = today - chrono::Duration::days(6);
            waka_api::SummaryParams::for_range(start, today)
        }
        SummaryPeriod::Month => {
            let start = today - chrono::Duration::days(29);
            waka_api::SummaryParams::for_range(start, today)
        }
    };

    let pb = stats_spinner("Fetching summary...");
    let summary = client.summaries(params).await?;
    pb.finish_and_clear();

    // Calculate totals
    let total_seconds: f64 = summary
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .sum();
    let hours = (total_seconds / 3600.0).floor() as u64;
    let minutes = ((total_seconds % 3600.0) / 60.0).floor() as u64;

    let days = summary.data.len();
    let avg_seconds = if days > 0 {
        total_seconds / days as f64
    } else {
        0.0
    };
    let avg_hours = (avg_seconds / 3600.0).floor() as u64;
    let avg_minutes = ((avg_seconds % 3600.0) / 60.0).floor() as u64;

    let period_name = match period {
        SummaryPeriod::Week => "Last 7 days",
        SummaryPeriod::Month => "Last 30 days",
    };

    println!("\n{period_name} Summary\n");
    println!("  Total:   {hours}h {minutes}m");
    println!("  Average: {avg_hours}h {avg_minutes}m per day");
    println!("  Days:    {days}\n");

    Ok(())
}

// ─── dashboard ────────────────────────────────────────────────────────────────

async fn dashboard(args: DashboardArgs, global: &GlobalOpts) -> Result<()> {
    let config = Config::load().unwrap_or_default();
    let profile = stats_profile_name(global);
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
/// **Never returns an error** — any failure (cache miss, expired entry,
/// corrupted DB) results in empty output so that the caller's prompt is never
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
/// expired entry, I/O error).  The 100ms budget is inherently satisfied
/// because this function only reads from sled (no network I/O).
fn prompt_inner(args: &PromptArgs, global: &GlobalOpts) -> Option<String> {
    let profile = global.profile.as_deref().unwrap_or("default");

    // Open the cache — silently skip on failure.
    let store = CacheStore::open(profile).ok()?;

    // Build the same cache key that `waka stats today` writes.
    let cache_key = SummaryParams::today().cache_key();

    // Retrieve the entry. Miss or expired → silent empty output.
    let entry = store
        .get::<waka_api::SummaryResponse>(&cache_key)
        .ok()
        .flatten()?;

    if entry.is_expired() {
        return None;
    }

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
        args.format,
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

// ─── config ───────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd is consumed by the match for exhaustive checking.
#[allow(clippy::needless_pass_by_value)]
async fn config(cmd: ConfigCommands, global: &GlobalOpts) -> Result<()> {
    match cmd {
        ConfigCommands::Get { .. } => bail!("not yet implemented: config get"),
        ConfigCommands::Set { .. } => bail!("not yet implemented: config set"),
        ConfigCommands::Edit => bail!("not yet implemented: config edit"),
        ConfigCommands::Path => bail!("not yet implemented: config path"),
        ConfigCommands::Reset { .. } => bail!("not yet implemented: config reset"),
        ConfigCommands::Doctor => config_doctor(global).await,
    }
}

/// Runs a full diagnostic check and prints a human-readable report.
///
/// Checks performed (in order):
/// 1. Config file found at the platform-specific path
/// 2. API key present in the credential priority chain
/// 3. API key valid (calls `/users/current`)
/// 4. API reachable (measures round-trip time)
/// 5. Cache directory writable
/// 6. Shell completions installed for the active shell
/// 7. Update check (compares version with GitHub Releases)
// The function has many sequential checks that are intentionally laid out in order.
#[allow(clippy::too_many_lines)]
async fn config_doctor(global: &GlobalOpts) -> Result<()> {
    let use_color = !global.no_color && should_use_color();
    let profile = global.profile.as_deref().unwrap_or("default");
    let mut issues: u32 = 0;
    let mut warnings: u32 = 0;

    // ── colour helpers ───────────────────────────────────────────────────────
    let ok_mark = if use_color {
        console::style("✓").green().to_string()
    } else {
        "✓".to_owned()
    };
    let warn_mark = if use_color {
        console::style("⚠").yellow().to_string()
    } else {
        "⚠".to_owned()
    };
    let fail_mark = if use_color {
        console::style("✗").red().to_string()
    } else {
        "✗".to_owned()
    };

    // ── 1. Config file ───────────────────────────────────────────────────────
    match waka_config::Config::path() {
        Ok(path) => {
            if path.exists() {
                println!("  {ok_mark}  Config file found at {}", path.display());
            } else {
                println!(
                    "  {warn_mark}  Config file not found at {} (using defaults)",
                    path.display()
                );
                warnings += 1;
            }
        }
        Err(e) => {
            println!("  {fail_mark}  Could not determine config path: {e}");
            issues += 1;
        }
    }

    // ── 2. API key ───────────────────────────────────────────────────────────
    let store = waka_config::CredentialStore::new(profile);
    let api_key_result = store.get_api_key();
    let api_key = if let Ok(key) = &api_key_result {
        println!("  {ok_mark}  API key found in credential store");
        Some(key.expose().to_owned())
    } else {
        println!("  {fail_mark}  No API key found — run `waka auth login` to authenticate");
        issues += 1;
        None
    };

    // ── 3 & 4. API key valid + reachability ──────────────────────────────────
    if let Some(key) = api_key {
        // Resolve API URL from config.
        let config = waka_config::Config::load().unwrap_or_default();
        let api_url = config.profiles.get(profile).map_or_else(
            || waka_config::ProfileConfig::default().api_url,
            |p| p.api_url.clone(),
        );
        let api_url_normalized = if api_url.ends_with('/') {
            api_url.clone()
        } else {
            format!("{api_url}/")
        };

        match waka_api::WakaClient::with_base_url(&key, &api_url_normalized) {
            Err(e) => {
                println!("  {fail_mark}  Could not build API client: {e}");
                issues += 1;
            }
            Ok(client) => {
                let t0 = std::time::Instant::now();
                match client.me().await {
                    Ok(user_resp) => {
                        let elapsed_ms = t0.elapsed().as_millis();
                        let identity = user_resp.email.as_deref().unwrap_or(&user_resp.username);
                        println!("  {ok_mark}  API key is valid (authenticated as {identity})");
                        println!("  {ok_mark}  API reachable (ping: {elapsed_ms}ms)");
                    }
                    Err(waka_api::ApiError::Unauthorized) => {
                        let elapsed_ms = t0.elapsed().as_millis();
                        println!(
                            "  {fail_mark}  API key is invalid or expired — run `waka auth login`"
                        );
                        // API IS reachable even if unauthorized.
                        println!("  {ok_mark}  API reachable (ping: {elapsed_ms}ms)");
                        issues += 1;
                    }
                    Err(e) => {
                        println!("  {fail_mark}  API unreachable: {e}");
                        println!("  {fail_mark}  Could not validate API key (network error)");
                        issues += 1;
                    }
                }
            }
        }
    } else {
        // Skip reachability if we have no key.
        println!("  {warn_mark}  API reachability check skipped (no API key)");
        warnings += 1;
    }

    // ── 5. Cache directory ───────────────────────────────────────────────────
    match waka_config::Config::cache_dir() {
        Ok(cache_path) => {
            // Try to create the directory if it does not exist.
            if let Err(e) = std::fs::create_dir_all(&cache_path) {
                println!(
                    "  {fail_mark}  Cache directory not writable at {}: {e}",
                    cache_path.display()
                );
                issues += 1;
            } else {
                // Quick writability probe: create then remove a temp file.
                let probe = cache_path.join(".waka_write_probe");
                match std::fs::write(&probe, b"") {
                    Ok(()) => {
                        let _ = std::fs::remove_file(&probe);
                        println!(
                            "  {ok_mark}  Cache directory writable at {}",
                            cache_path.display()
                        );
                    }
                    Err(e) => {
                        println!(
                            "  {fail_mark}  Cache directory not writable at {}: {e}",
                            cache_path.display()
                        );
                        issues += 1;
                    }
                }
            }
        }
        Err(e) => {
            println!("  {fail_mark}  Could not determine cache directory: {e}");
            issues += 1;
        }
    }

    // ── 6. Shell completions ─────────────────────────────────────────────────
    let detected_shell = std::env::var("SHELL").ok().and_then(|s| {
        std::path::Path::new(&s)
            .file_name()
            .and_then(|n| n.to_str())
            .map(std::string::ToString::to_string)
    });

    match detected_shell.as_deref() {
        Some("zsh") => {
            let zfunc = dirs_check_zsh_completions();
            if zfunc {
                println!("  {ok_mark}  Shell completions installed (zsh)");
            } else {
                println!(
                    "  {warn_mark}  Shell completions not found for zsh — run `waka completions zsh`"
                );
                warnings += 1;
            }
        }
        Some("bash") => {
            let found = dirs_check_bash_completions();
            if found {
                println!("  {ok_mark}  Shell completions installed (bash)");
            } else {
                println!(
                    "  {warn_mark}  Shell completions not found for bash — run `waka completions bash`"
                );
                warnings += 1;
            }
        }
        Some("fish") => {
            let fish_path = directories::BaseDirs::new()
                .map(|d| d.home_dir().join(".config/fish/completions/waka.fish"));
            if fish_path.as_ref().is_some_and(|p| p.exists()) {
                println!("  {ok_mark}  Shell completions installed (fish)");
            } else {
                println!(
                    "  {warn_mark}  Shell completions not found for fish — run `waka completions fish`"
                );
                warnings += 1;
            }
        }
        Some(shell) => {
            println!("  {warn_mark}  Shell completions check skipped (unsupported shell: {shell})");
            warnings += 1;
        }
        None => {
            println!("  {warn_mark}  Shell completions check skipped ($SHELL not set)");
            warnings += 1;
        }
    }

    // ── 7. Version / update check ────────────────────────────────────────────
    let current = env!("CARGO_PKG_VERSION");
    match check_latest_version().await {
        Ok(Some(latest)) if latest != current && version_is_newer(&latest, current) => {
            println!(
                "  {warn_mark}  waka v{current} installed — v{latest} available (run: waka update)"
            );
            warnings += 1;
        }
        Ok(_) => {
            println!("  {ok_mark}  waka v{current} is up to date");
        }
        Err(_) => {
            // Update check failure is non-critical — silently skip.
            println!("  {warn_mark}  Could not check for updates (network unavailable?)");
            warnings += 1;
        }
    }

    // ── Summary ──────────────────────────────────────────────────────────────
    if issues == 0 && warnings == 0 {
        println!("  {ok_mark}  No known issues");
    } else if issues > 0 {
        let label = if issues == 1 { "issue" } else { "issues" };
        println!("  {fail_mark}  {issues} {label} found — check the output above");
    }

    Ok(())
}

/// Returns `true` if the zsh completion file exists in a standard `$fpath`
/// location.
fn dirs_check_zsh_completions() -> bool {
    // Common user-level locations
    let Some(base) = directories::BaseDirs::new() else {
        return false;
    };
    let home = base.home_dir();
    let candidates = [
        home.join(".zfunc/_waka"),
        home.join(".zfunc/waka.zsh"),
        home.join(".local/share/zsh/site-functions/_waka"),
        std::path::PathBuf::from("/usr/local/share/zsh/site-functions/_waka"),
        std::path::PathBuf::from("/usr/share/zsh/site-functions/_waka"),
    ];
    candidates.iter().any(|p| p.exists())
}

/// Returns `true` if the bash completion file can be found.
fn dirs_check_bash_completions() -> bool {
    let Some(base) = directories::BaseDirs::new() else {
        return false;
    };
    let home = base.home_dir();
    let candidates = [
        home.join(".local/share/bash-completion/completions/waka"),
        home.join(".bash_completion.d/waka"),
        std::path::PathBuf::from("/usr/local/share/bash-completion/completions/waka"),
        std::path::PathBuf::from("/usr/share/bash-completion/completions/waka"),
    ];
    candidates.iter().any(|p| p.exists())
}

/// Fetches the latest release tag from the GitHub API.
///
/// Returns `None` if no releases exist yet, or an error on network failure.
const GITHUB_API: &str = "https://api.github.com/repos/mouwaficbdr/waka/releases/latest";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

async fn check_latest_version() -> anyhow::Result<Option<String>> {
    // GitHub requires a User-Agent header.
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .user_agent(concat!("waka/", env!("CARGO_PKG_VERSION")))
        .build()?;

    let resp = client.get(GITHUB_API).send().await?;

    // 404 means no releases yet.
    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let json: serde_json::Value = resp.error_for_status()?.json().await?;
    let tag = json["tag_name"]
        .as_str()
        .map(|t| t.trim_start_matches('v').to_owned());

    Ok(tag)
}

/// Naive semver comparison: returns `true` when `candidate` is strictly newer
/// than `current`. Both strings are expected in `MAJOR.MINOR.PATCH` format.
///
/// Returns `false` if parsing fails (safe default).
fn version_is_newer(candidate: &str, current: &str) -> bool {
    fn parse(s: &str) -> Option<(u64, u64, u64)> {
        let mut it = s.splitn(3, '.').map(|p| p.parse::<u64>().ok());
        let maj = it.next()??;
        let min = it.next()??;
        let pat = it.next()??;
        Some((maj, min, pat))
    }
    match (parse(candidate), parse(current)) {
        (Some(c), Some(cur)) => c > cur,
        _ => false,
    }
}

/// Background update check task.
///
/// Runs once per day (as determined by the cache TTL). Prints a one-line
/// notice to **stderr** when a newer `waka` version is available.
///
/// All errors are swallowed — an update check must never break a command.
async fn update_check_background(global: GlobalOpts) {
    // Cache key stores the last-fetched latest version string.
    // TTL = 24 h — so we hit GitHub at most once per day.
    const UPDATE_CACHE_KEY: &str = "update_check:latest";
    const CHECK_INTERVAL: Duration = Duration::from_secs(86_400);

    // 1. Honour WAKA_NO_UPDATE_CHECK env var.
    if std::env::var_os("WAKA_NO_UPDATE_CHECK").is_some() {
        return;
    }

    // 2. Honour config flag.
    let config = Config::load().unwrap_or_default();
    if !config.core.update_check {
        return;
    }

    let profile = global.profile.as_deref().unwrap_or("default");
    let Ok(store) = CacheStore::open(profile) else {
        return;
    };

    // 3. Determine if we need to fetch (cache miss or expired).
    let cached = store.get::<String>(UPDATE_CACHE_KEY).ok().flatten();

    let latest = if cached.as_ref().is_some_and(|e| !e.is_expired()) {
        // Cache hit — use stored value immediately (no network).
        cached.map(|e| e.value)
    } else {
        // Cache miss or expired — fetch from GitHub.
        match check_latest_version().await {
            Ok(Some(v)) => {
                let _ = store.set(UPDATE_CACHE_KEY, &v, CHECK_INTERVAL);
                Some(v)
            }
            // No releases yet or network error — skip silently.
            _ => return,
        }
    };

    let Some(latest) = latest else { return };
    let current = env!("CARGO_PKG_VERSION");

    if version_is_newer(&latest, current) {
        eprintln!("\n  ⚠  waka v{current} installed — v{latest} available (run: waka update)");
    }
}

// ─── update / changelog ───────────────────────────────────────────────────────

/// Whether the binary lives under a Homebrew-managed path.
fn is_homebrew_install() -> bool {
    std::env::current_exe().is_ok_and(|exe| {
        let p = exe.to_string_lossy();
        p.contains("/Cellar/") || p.contains("/homebrew/")
    })
}

/// Returns the release asset target triple and archive extension for the
/// current platform, e.g. `("x86_64-unknown-linux-gnu", "tar.gz")`.
fn platform_target() -> Result<(&'static str, &'static str)> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Ok(("x86_64-unknown-linux-gnu", "tar.gz")),
        ("linux", "aarch64") => Ok(("aarch64-unknown-linux-gnu", "tar.gz")),
        ("macos", "x86_64") => Ok(("x86_64-apple-darwin", "tar.gz")),
        ("macos", "aarch64") => Ok(("aarch64-apple-darwin", "tar.gz")),
        ("windows", "x86_64") => Ok(("x86_64-pc-windows-msvc", "zip")),
        (os, arch) => bail!(
            "Unsupported platform {os}/{arch}. Update manually:\n\
             https://github.com/mouwaficbdr/waka/releases"
        ),
    }
}

/// Extract the `waka` binary from a `.tar.gz` archive and atomically replace
/// the running executable. Non-Windows only.
#[cfg(not(target_os = "windows"))]
fn extract_tar_gz_and_replace(archive_bytes: &[u8], current_exe: &std::path::Path) -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let gz = GzDecoder::new(archive_bytes);
    let mut archive = Archive::new(gz);

    for entry in archive.entries().context("failed to read tar entries")? {
        let mut entry = entry.context("corrupt tar entry")?;
        let path = entry.path().context("invalid tar entry path")?;
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();

        if file_name == "waka" {
            let temp_path = current_exe.with_extension("waka.tmp");
            {
                let mut dest =
                    std::fs::File::create(&temp_path).context("cannot create temp file")?;
                std::io::copy(&mut entry, &mut dest).context("failed to write new binary")?;

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt as _;
                    let mut perms = dest
                        .metadata()
                        .context("cannot read temp file metadata")?
                        .permissions();
                    perms.set_mode(0o755);
                    std::fs::set_permissions(&temp_path, perms)
                        .context("cannot set executable permission")?;
                }
            }
            std::fs::rename(&temp_path, current_exe)
                .context("failed to replace binary — try with elevated privileges")?;
            return Ok(());
        }
    }
    bail!("Could not find 'waka' binary in the release archive")
}

/// Extract `waka.exe` from a `.zip` archive and replace the running binary.
/// Windows only.
#[cfg(target_os = "windows")]
fn extract_zip_and_replace(archive_bytes: &[u8], current_exe: &std::path::Path) -> Result<()> {
    use std::io::Cursor;
    use zip::ZipArchive;

    let cursor = Cursor::new(archive_bytes);
    let mut archive = ZipArchive::new(cursor).context("failed to open zip archive")?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).context("corrupt zip entry")?;
        let name = file.name().to_owned();
        if name == "waka.exe" || name.ends_with("/waka.exe") {
            let temp_path = current_exe.with_extension("tmp.exe");
            let mut dest = std::fs::File::create(&temp_path).context("cannot create temp file")?;
            std::io::copy(&mut file, &mut dest).context("failed to write new binary")?;
            drop(dest);
            std::fs::rename(&temp_path, current_exe)
                .context("failed to replace binary — try running as Administrator")?;
            return Ok(());
        }
    }
    bail!("Could not find 'waka.exe' in the release archive")
}

/// Implements `waka update`.
///
/// Downloads the latest release from GitHub Releases and atomically replaces
/// the current binary.
async fn update_self(global: &GlobalOpts) -> Result<()> {
    // 1. Fetch latest version.
    let pb = stats_spinner("Checking for updates…");
    let latest = match check_latest_version().await {
        Ok(Some(v)) => v,
        Ok(None) => {
            pb.finish_and_clear();
            if !global.quiet {
                println!("  ✓  No releases found — you are on the latest build.");
            }
            return Ok(());
        }
        Err(e) => {
            pb.finish_and_clear();
            bail!("Failed to check for updates: {e}");
        }
    };
    pb.finish_and_clear();

    // 2. Already up-to-date?
    if !version_is_newer(&latest, CURRENT_VERSION) {
        if !global.quiet {
            println!("  ✓  Already on the latest version (v{latest})");
        }
        return Ok(());
    }

    // 3. Homebrew — defer to brew(1).
    if is_homebrew_install() {
        println!("  ℹ  Detected Homebrew installation. Run:\n\n       brew upgrade waka\n");
        return Ok(());
    }

    if !global.quiet {
        println!("  ⬆  Updating waka v{CURRENT_VERSION} → v{latest}");
    }

    // 4. Resolve platform asset.
    let (target, ext) = platform_target()?;
    let archive_name = format!("waka-v{latest}-{target}.{ext}");
    let url =
        format!("https://github.com/mouwaficbdr/waka/releases/download/v{latest}/{archive_name}");

    // 5. Download.
    let pb = stats_spinner(&format!("Downloading {archive_name}…"));
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .user_agent(concat!("waka/", env!("CARGO_PKG_VERSION")))
        .build()
        .context("failed to build HTTP client")?;

    let resp = client
        .get(&url)
        .send()
        .await
        .context("download request failed")?;

    if !resp.status().is_success() {
        pb.finish_and_clear();
        bail!(
            "Download failed (HTTP {}): {}\n\
             Check release assets at: https://github.com/mouwaficbdr/waka/releases",
            resp.status(),
            url
        );
    }

    let bytes = resp
        .bytes()
        .await
        .context("failed to read download response")?;
    pb.finish_and_clear();

    // 6. Extract + atomically replace.
    let pb = stats_spinner("Installing new binary…");
    let current_exe =
        std::env::current_exe().context("cannot determine current executable path")?;

    #[cfg(target_os = "windows")]
    extract_zip_and_replace(&bytes, &current_exe)?;

    #[cfg(not(target_os = "windows"))]
    extract_tar_gz_and_replace(&bytes, &current_exe)?;

    pb.finish_and_clear();

    if !global.quiet {
        println!("  ✓  waka updated to v{latest} successfully!");
    }

    Ok(())
}

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
    let profile = global.profile.as_deref().unwrap_or("default");

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

// ─── report generation ────────────────────────────────────────────────────────

/// Generates a Markdown report.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::redundant_closure_for_method_calls
)]
fn generate_report_markdown(
    summary: &waka_api::SummaryResponse,
    goals: Option<&waka_api::GoalsResponse>,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
) -> String {
    use chrono::Datelike as _;
    use std::fmt::Write as _;

    let mut output = String::new();

    // Header
    output.push_str("# Productivity Report\n\n");
    let _ = writeln!(output, "**Period:** {start_date} to {end_date}\n");

    // Calculate totals
    let total_seconds: f64 = summary
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .sum();
    let hours = (total_seconds / 3600.0).floor() as u64;
    let minutes = ((total_seconds % 3600.0) / 60.0).floor() as u64;

    let _ = writeln!(output, "**Total Time:** {hours}h {minutes}m\n");

    let days_with_data = summary
        .data
        .iter()
        .filter(|d| d.grand_total.total_seconds > 0.0)
        .count();
    if days_with_data > 0 {
        let avg_seconds = total_seconds / days_with_data as f64;
        let avg_hours = (avg_seconds / 3600.0).floor() as u64;
        let avg_minutes = ((avg_seconds % 3600.0) / 60.0).floor() as u64;
        let _ = writeln!(
            output,
            "**Average per day:** {avg_hours}h {avg_minutes}m (across {days_with_data} active days)\n"
        );
    }

    output.push_str("---\n\n");

    // Projects breakdown
    output.push_str("## Projects\n\n");

    let mut project_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for project in &day.projects {
            *project_totals.entry(project.name.clone()).or_insert(0.0) += project.total_seconds;
        }
    }

    if project_totals.is_empty() {
        output.push_str("*No project data available.*\n\n");
    } else {
        let mut projects: Vec<_> = project_totals.into_iter().collect();
        projects.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        output.push_str("| Project | Time | Percentage |\n");
        output.push_str("|---------|------|------------|\n");

        for (name, secs) in projects.iter().take(10) {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(output, "| {name} | {hrs}h {mins}m | {pct}% |");
        }
        output.push('\n');
    }

    // Languages breakdown
    output.push_str("## Languages\n\n");

    let mut language_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for lang in &day.languages {
            *language_totals.entry(lang.name.clone()).or_insert(0.0) += lang.total_seconds;
        }
    }

    if language_totals.is_empty() {
        output.push_str("*No language data available.*\n\n");
    } else {
        let mut languages: Vec<_> = language_totals.into_iter().collect();
        languages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        output.push_str("| Language | Time | Percentage |\n");
        output.push_str("|----------|------|------------|\n");

        for (name, secs) in languages.iter().take(10) {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(output, "| {name} | {hrs}h {mins}m | {pct}% |");
        }
        output.push('\n');
    }

    // Editors breakdown
    output.push_str("## Editors\n\n");

    let mut editor_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for editor in &day.editors {
            *editor_totals.entry(editor.name.clone()).or_insert(0.0) += editor.total_seconds;
        }
    }

    if editor_totals.is_empty() {
        output.push_str("*No editor data available.*\n\n");
    } else {
        let mut editors: Vec<_> = editor_totals.into_iter().collect();
        editors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        output.push_str("| Editor | Time | Percentage |\n");
        output.push_str("|--------|------|------------|\n");

        for (name, secs) in &editors {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(output, "| {name} | {hrs}h {mins}m | {pct}% |");
        }
        output.push('\n');
    }

    // Daily activity
    output.push_str("## Daily Activity\n\n");
    output.push_str("| Date | Day | Time |\n");
    output.push_str("|------|-----|------|\n");

    for day_data in &summary.data {
        let date_str = day_data.range.date.as_deref().unwrap_or("N/A");
        let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok();
        let day_name = match date.as_ref().map(|d| d.weekday()) {
            Some(chrono::Weekday::Mon) => "Mon",
            Some(chrono::Weekday::Tue) => "Tue",
            Some(chrono::Weekday::Wed) => "Wed",
            Some(chrono::Weekday::Thu) => "Thu",
            Some(chrono::Weekday::Fri) => "Fri",
            Some(chrono::Weekday::Sat) => "Sat",
            Some(chrono::Weekday::Sun) => "Sun",
            None => "?",
        };
        let secs = day_data.grand_total.total_seconds;
        let hrs = (secs / 3600.0).floor() as u64;
        let mins = ((secs % 3600.0) / 60.0).floor() as u64;
        let _ = writeln!(output, "| {date_str} | {day_name} | {hrs}h {mins}m |");
    }
    output.push('\n');

    // Goals achieved
    if let Some(goals_resp) = goals {
        if !goals_resp.data.is_empty() {
            output.push_str("## Goals\n\n");

            for goal in &goals_resp.data {
                let status = if goal.status == "success" {
                    "✓ Achieved"
                } else if goal.status == "pending" {
                    "⋯ In Progress"
                } else {
                    "✗ Not Achieved"
                };
                let title = &goal.title;
                let _ = writeln!(output, "- **{title}**: {status}");
            }
            output.push('\n');
        }
    }

    output.push_str("---\n\n");
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M");
    let _ = writeln!(output, "*Generated on {now} by waka*");

    output
}

/// Generates an HTML report with inline CSS.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::redundant_closure_for_method_calls
)]
fn generate_report_html(
    summary: &waka_api::SummaryResponse,
    goals: Option<&waka_api::GoalsResponse>,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
) -> String {
    use chrono::Datelike as _;
    use std::fmt::Write as _;

    // Calculate totals
    let total_seconds: f64 = summary
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .sum();
    let hours = (total_seconds / 3600.0).floor() as u64;
    let minutes = ((total_seconds % 3600.0) / 60.0).floor() as u64;

    let days_with_data = summary
        .data
        .iter()
        .filter(|d| d.grand_total.total_seconds > 0.0)
        .count();
    let avg_display = if days_with_data > 0 {
        let avg_seconds = total_seconds / days_with_data as f64;
        let avg_hours = (avg_seconds / 3600.0).floor() as u64;
        let avg_minutes = ((avg_seconds % 3600.0) / 60.0).floor() as u64;
        format!("{avg_hours}h {avg_minutes}m (across {days_with_data} active days)")
    } else {
        "N/A".to_string()
    };

    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("<title>Productivity Report</title>\n");
    html.push_str("<style>\n");
    html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 900px; margin: 40px auto; padding: 20px; background: #f5f5f5; }\n");
    html.push_str("h1 { color: #333; border-bottom: 3px solid #4CAF50; padding-bottom: 10px; }\n");
    html.push_str("h2 { color: #555; margin-top: 40px; border-bottom: 2px solid #ddd; padding-bottom: 8px; }\n");
    html.push_str(".header { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-bottom: 20px; }\n");
    html.push_str(".stat { font-size: 18px; margin: 10px 0; }\n");
    html.push_str(".stat strong { color: #4CAF50; }\n");
    html.push_str("table { width: 100%; border-collapse: collapse; background: white; margin: 20px 0; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
    html.push_str("th { background: #4CAF50; color: white; padding: 12px; text-align: left; }\n");
    html.push_str("td { padding: 12px; border-bottom: 1px solid #ddd; }\n");
    html.push_str("tr:last-child td { border-bottom: none; }\n");
    html.push_str("tr:hover { background: #f9f9f9; }\n");
    html.push_str(
        ".footer { text-align: center; margin-top: 40px; color: #888; font-size: 14px; }\n",
    );
    html.push_str(".goal-list { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
    html.push_str(".goal-item { padding: 10px 0; border-bottom: 1px solid #eee; }\n");
    html.push_str(".goal-item:last-child { border-bottom: none; }\n");
    html.push_str("</style>\n");
    html.push_str("</head>\n<body>\n");

    // Header
    html.push_str("<div class=\"header\">\n");
    html.push_str("<h1>Productivity Report</h1>\n");
    let _ = writeln!(
        html,
        "<div class=\"stat\"><strong>Period:</strong> {start_date} to {end_date}</div>"
    );
    let _ = writeln!(
        html,
        "<div class=\"stat\"><strong>Total Time:</strong> {hours}h {minutes}m</div>"
    );
    let _ = writeln!(
        html,
        "<div class=\"stat\"><strong>Average per day:</strong> {avg_display}</div>"
    );
    html.push_str("</div>\n");

    // Projects
    html.push_str("<h2>Projects</h2>\n");
    let mut project_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for project in &day.projects {
            *project_totals.entry(project.name.clone()).or_insert(0.0) += project.total_seconds;
        }
    }

    if project_totals.is_empty() {
        html.push_str("<p><em>No project data available.</em></p>\n");
    } else {
        let mut projects: Vec<_> = project_totals.into_iter().collect();
        projects.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        html.push_str(
            "<table>\n<thead><tr><th>Project</th><th>Time</th><th>Percentage</th></tr></thead>\n<tbody>\n",
        );
        for (name, secs) in projects.iter().take(10) {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(
                html,
                "<tr><td>{name}</td><td>{hrs}h {mins}m</td><td>{pct}%</td></tr>"
            );
        }
        html.push_str("</tbody>\n</table>\n");
    }

    // Languages
    html.push_str("<h2>Languages</h2>\n");
    let mut language_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for lang in &day.languages {
            *language_totals.entry(lang.name.clone()).or_insert(0.0) += lang.total_seconds;
        }
    }

    if language_totals.is_empty() {
        html.push_str("<p><em>No language data available.</em></p>\n");
    } else {
        let mut languages: Vec<_> = language_totals.into_iter().collect();
        languages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        html.push_str(
            "<table>\n<thead><tr><th>Language</th><th>Time</th><th>Percentage</th></tr></thead>\n<tbody>\n",
        );
        for (name, secs) in languages.iter().take(10) {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(
                html,
                "<tr><td>{name}</td><td>{hrs}h {mins}m</td><td>{pct}%</td></tr>"
            );
        }
        html.push_str("</tbody>\n</table>\n");
    }

    // Editors
    html.push_str("<h2>Editors</h2>\n");
    let mut editor_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for editor in &day.editors {
            *editor_totals.entry(editor.name.clone()).or_insert(0.0) += editor.total_seconds;
        }
    }

    if editor_totals.is_empty() {
        html.push_str("<p><em>No editor data available.</em></p>\n");
    } else {
        let mut editors: Vec<_> = editor_totals.into_iter().collect();
        editors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        html.push_str(
            "<table>\n<thead><tr><th>Editor</th><th>Time</th><th>Percentage</th></tr></thead>\n<tbody>\n",
        );
        for (name, secs) in &editors {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(
                html,
                "<tr><td>{name}</td><td>{hrs}h {mins}m</td><td>{pct}%</td></tr>"
            );
        }
        html.push_str("</tbody>\n</table>\n");
    }

    // Daily activity
    html.push_str("<h2>Daily Activity</h2>\n");
    html.push_str(
        "<table>\n<thead><tr><th>Date</th><th>Day</th><th>Time</th></tr></thead>\n<tbody>\n",
    );
    for day_data in &summary.data {
        let date_str = day_data.range.date.as_deref().unwrap_or("N/A");
        let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok();
        let day_name = match date.as_ref().map(|d| d.weekday()) {
            Some(chrono::Weekday::Mon) => "Mon",
            Some(chrono::Weekday::Tue) => "Tue",
            Some(chrono::Weekday::Wed) => "Wed",
            Some(chrono::Weekday::Thu) => "Thu",
            Some(chrono::Weekday::Fri) => "Fri",
            Some(chrono::Weekday::Sat) => "Sat",
            Some(chrono::Weekday::Sun) => "Sun",
            None => "?",
        };
        let secs = day_data.grand_total.total_seconds;
        let hrs = (secs / 3600.0).floor() as u64;
        let mins = ((secs % 3600.0) / 60.0).floor() as u64;
        let _ = writeln!(
            html,
            "<tr><td>{date_str}</td><td>{day_name}</td><td>{hrs}h {mins}m</td></tr>"
        );
    }
    html.push_str("</tbody>\n</table>\n");

    // Goals
    if let Some(goals_resp) = goals {
        if !goals_resp.data.is_empty() {
            html.push_str("<h2>Goals</h2>\n<div class=\"goal-list\">\n");
            for goal in &goals_resp.data {
                let status = if goal.status == "success" {
                    "✓ Achieved"
                } else if goal.status == "pending" {
                    "⋯ In Progress"
                } else {
                    "✗ Not Achieved"
                };
                let title = &goal.title;
                let _ = writeln!(
                    html,
                    "<div class=\"goal-item\"><strong>{title}:</strong> {status}</div>"
                );
            }
            html.push_str("</div>\n");
        }
    }

    // Footer
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M");
    let _ = writeln!(
        html,
        "<div class=\"footer\">Generated on {now} by waka</div>"
    );
    html.push_str("</body>\n</html>");

    html
}

/// Generates a JSON report.
fn generate_report_json(
    summary: &waka_api::SummaryResponse,
    goals: Option<&waka_api::GoalsResponse>,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
) -> Result<String> {
    let total_seconds: f64 = summary
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .sum();

    let report = serde_json::json!({
        "period": {
            "start": start_date.to_string(),
            "end": end_date.to_string(),
        },
        "summary": {
            "total_seconds": total_seconds,
            "total_hours": (total_seconds / 3600.0).floor(),
            "total_minutes": ((total_seconds % 3600.0) / 60.0).floor(),
        },
        "data": summary.data,
        "goals": goals.map(|g| &g.data),
    });

    serde_json::to_string_pretty(&report).context("failed to serialize report to JSON")
}

/// Generates a CSV report.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::redundant_closure_for_method_calls
)]
fn generate_report_csv(
    summary: &waka_api::SummaryResponse,
    _start_date: chrono::NaiveDate,
    _end_date: chrono::NaiveDate,
) -> String {
    use chrono::Datelike as _;
    use std::fmt::Write as _;

    let mut output = String::new();

    // Header
    output.push_str("Date,Day,Total Time (hours),Projects,Languages,Editors\n");

    // Daily rows
    for day_data in &summary.data {
        let date_str = day_data.range.date.as_deref().unwrap_or("N/A");
        let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok();
        let day_name = match date.as_ref().map(|d| d.weekday()) {
            Some(chrono::Weekday::Mon) => "Mon",
            Some(chrono::Weekday::Tue) => "Tue",
            Some(chrono::Weekday::Wed) => "Wed",
            Some(chrono::Weekday::Thu) => "Thu",
            Some(chrono::Weekday::Fri) => "Fri",
            Some(chrono::Weekday::Sat) => "Sat",
            Some(chrono::Weekday::Sun) => "Sun",
            None => "?",
        };

        let total_hours = day_data.grand_total.total_seconds / 3600.0;

        let projects: Vec<String> = day_data
            .projects
            .iter()
            .map(|p| {
                let h = (p.total_seconds / 3600.0).round() as u64;
                format!("{}({h}h)", p.name)
            })
            .collect();
        let projects_str = projects.join("; ");

        let languages: Vec<String> = day_data
            .languages
            .iter()
            .map(|l| {
                let h = (l.total_seconds / 3600.0).round() as u64;
                format!("{}({h}h)", l.name)
            })
            .collect();
        let languages_str = languages.join("; ");

        let editors: Vec<String> = day_data
            .editors
            .iter()
            .map(|e| {
                let h = (e.total_seconds / 3600.0).round() as u64;
                format!("{}({h}h)", e.name)
            })
            .collect();
        let editors_str = editors.join("; ");

        let _ = writeln!(
            output,
            "{date_str},{day_name},{total_hours:.2},\"{projects_str}\",\"{languages_str}\",\"{editors_str}\""
        );
    }

    output
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

    // ── stats_profile_name ────────────────────────────────────────────────────

    #[test]
    fn profile_name_defaults_to_default() {
        let global = GlobalOpts {
            profile: None,
            ..GlobalOpts::default()
        };
        assert_eq!(stats_profile_name(&global), "default");
    }

    #[test]
    fn profile_name_uses_explicit_value() {
        let global = GlobalOpts {
            profile: Some("work".to_owned()),
            ..GlobalOpts::default()
        };
        assert_eq!(stats_profile_name(&global), "work");
    }

    // ── version_is_newer ──────────────────────────────────────────────────────

    #[test]
    fn version_is_newer_returns_true_for_higher_patch() {
        assert!(version_is_newer("0.1.1", "0.1.0"));
    }

    #[test]
    fn version_is_newer_returns_true_for_higher_minor() {
        assert!(version_is_newer("0.2.0", "0.1.9"));
    }

    #[test]
    fn version_is_newer_returns_true_for_higher_major() {
        assert!(version_is_newer("1.0.0", "0.9.9"));
    }

    #[test]
    fn version_is_newer_returns_false_for_equal() {
        assert!(!version_is_newer("0.1.0", "0.1.0"));
    }

    #[test]
    fn version_is_newer_returns_false_for_older() {
        assert!(!version_is_newer("0.0.9", "0.1.0"));
    }

    #[test]
    fn version_is_newer_returns_false_for_malformed_input() {
        assert!(!version_is_newer("not-a-version", "0.1.0"));
        assert!(!version_is_newer("0.1.0", "not-a-version"));
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
