//! Command handlers for `waka`.
//!
//! Each function corresponds to a leaf command in the CLI tree. Auth handlers
//! and the stats handler are fully implemented; all others remain stubs to be
//! filled in during later phases.

use std::io::IsTerminal as _;
use std::time::Duration;

use anyhow::{bail, Context as _, Result};
use chrono::{Local, NaiveDate};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use waka_api::{StatsRange, SummaryEntry, SummaryParams, WakaClient};
use waka_cache::CacheStore;
use waka_config::{Config, CredentialStore, ProfileConfig};
use waka_render::{
    detect_output_format, should_use_color, BreakdownRenderer, OutputFormat as RenderFormat,
    RenderOptions, SummaryRenderer,
};

use crate::auth;
use crate::cli::{
    AuthCommands, CacheCommands, Commands, CompletionShell, ConfigCommands, DashboardArgs,
    EditorsCommands, GlobalOpts, GoalsCommands, LanguagesCommands, LeaderboardCommands,
    OutputFormat as CliFormat, ProjectsCommands, PromptArgs, ReportCommands, StatsCommands,
    StatsFilterOpts,
};

/// Dispatch a parsed [`Commands`] variant to the appropriate handler.
///
/// `global` carries flags shared by every command (profile, format, color,
/// verbosity). It is passed down to handlers that need it.
pub async fn dispatch(cmd: Commands, global: GlobalOpts) -> Result<()> {
    match cmd {
        Commands::Auth { cmd } => auth_cmd(cmd, global).await,
        Commands::Stats { cmd } => stats(cmd, &global).await,
        Commands::Projects { cmd } => projects(cmd, &global).await,
        Commands::Languages { cmd } => languages(cmd, &global).await,
        Commands::Editors { cmd } => editors(cmd, &global).await,
        Commands::Goals { cmd } => goals(cmd),
        Commands::Leaderboard { cmd } => leaderboard(cmd),
        Commands::Report { cmd } => report(cmd),
        Commands::Dashboard(args) => dashboard(args),
        Commands::Prompt(args) => prompt(args),
        Commands::Completions { shell } => {
            completions(shell);
            Ok(())
        }
        Commands::Config { cmd } => config(cmd, &global).await,
        Commands::Cache { cmd } => cache(cmd, &global),
    }
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
    let opts = RenderOptions {
        color,
        format,
        csv_bom: global.csv_bom,
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
    let target = if std::io::stderr().is_terminal() {
        ProgressDrawTarget::stderr()
    } else {
        ProgressDrawTarget::hidden()
    };

    let pb = ProgressBar::with_draw_target(None, target);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}")
            .expect("spinner template is valid"),
    );
    pb.set_message(msg.to_owned());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
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

/// Converts a [`Period`] (CLI value) to its [`StatsRange`] equivalent.
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
            // Projects list: use AllTime stats for time data per project.
            let pb = stats_spinner("Fetching projects …");
            let resp = client.stats(StatsRange::AllTime).await;
            pb.finish_and_clear();
            let resp = resp.with_context(|| "failed to fetch project list from WakaTime")?;

            let mut entries = entries_from_stats(&resp.data.projects);

            if matches!(sort_by, crate::cli::ProjectSortBy::Name) {
                entries.sort_by(|a, b| a.0.cmp(&b.0));
            }

            let output = BreakdownRenderer::render(&entries, "Project", limit, &opts);
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
            let today = chrono::Local::now().date_naive();
            let start = from.as_deref().map_or(Ok(today), |s| {
                chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .with_context(|| format!("--from must be YYYY-MM-DD, got '{s}'"))
            })?;
            let end = to.as_deref().map_or(Ok(today), |s| {
                chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .with_context(|| format!("--to must be YYYY-MM-DD, got '{s}'"))
            })?;
            let params = SummaryParams::for_range(start, end).project(&project_name);
            let pb = stats_spinner(&format!("Fetching stats for '{project_name}' …"));
            let resp = client.summaries(params).await;
            pb.finish_and_clear();
            let resp =
                resp.with_context(|| format!("failed to fetch stats for '{project_name}'"))?;
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
// `needless_pass_by_value`: cmd consumed by match; data is ignored since all arms bail.
#[allow(clippy::needless_pass_by_value)]
fn goals(cmd: GoalsCommands) -> Result<()> {
    match cmd {
        GoalsCommands::List => bail!("not yet implemented: goals list"),
        GoalsCommands::Show { .. } => bail!("not yet implemented: goals show"),
        GoalsCommands::Watch { .. } => bail!("not yet implemented: goals watch"),
    }
}

// ─── leaderboard ──────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd is consumed by the match for exhaustive checking.
#[allow(clippy::needless_pass_by_value)]
fn leaderboard(cmd: LeaderboardCommands) -> Result<()> {
    match cmd {
        LeaderboardCommands::Show { .. } => bail!("not yet implemented: leaderboard show"),
    }
}

// ─── report ───────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd is consumed by the match for exhaustive checking.
#[allow(clippy::needless_pass_by_value)]
fn report(cmd: ReportCommands) -> Result<()> {
    match cmd {
        ReportCommands::Generate { .. } => bail!("not yet implemented: report generate"),
        ReportCommands::Summary { .. } => bail!("not yet implemented: report summary"),
    }
}

// ─── dashboard ────────────────────────────────────────────────────────────────

fn dashboard(_args: DashboardArgs) -> Result<()> {
    bail!("not yet implemented: dashboard")
}

// ─── prompt ───────────────────────────────────────────────────────────────────

fn prompt(_args: PromptArgs) -> Result<()> {
    bail!("not yet implemented: prompt")
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
async fn check_latest_version() -> anyhow::Result<Option<String>> {
    // GitHub requires a User-Agent header.
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .user_agent(concat!("waka/", env!("CARGO_PKG_VERSION")))
        .build()?;

    let resp = client
        .get("https://api.github.com/repos/mouwaficbdr/waka/releases/latest")
        .send()
        .await?;

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
