//! Command handlers for `waka`.
//!
//! Each function corresponds to a leaf command in the CLI tree. All handlers
//! are stubs that will be filled in during later phases. They return
//! `anyhow::Result<()>` for uniform error propagation.

use anyhow::{bail, Result};

use crate::cli::{
    AuthCommands, Commands, ConfigCommands, EditorsCommands, GoalsCommands, LanguagesCommands,
    LeaderboardCommands, ProjectsCommands, ReportCommands, StatsCommands,
};
use crate::cli::{DashboardArgs, PromptArgs};

/// Dispatch a parsed [`Commands`] variant to the appropriate handler.
///
/// `dispatch` is `async` because concrete handlers will perform network I/O
/// in later phases. Stub handlers are synchronous; when a handler becomes
/// async it simply gains an `async` keyword with no further refactoring needed.
// `unused_async`: correct by design — inner handlers become async in later phases.
#[allow(clippy::unused_async)]
pub async fn dispatch(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Auth { cmd } => auth(cmd),
        Commands::Stats { cmd } => stats(cmd),
        Commands::Projects { cmd } => projects(cmd),
        Commands::Languages { cmd } => languages(cmd),
        Commands::Editors { cmd } => editors(cmd),
        Commands::Goals { cmd } => goals(cmd),
        Commands::Leaderboard { cmd } => leaderboard(cmd),
        Commands::Report { cmd } => report(cmd),
        Commands::Dashboard(args) => dashboard(args),
        Commands::Prompt(args) => prompt(args),
        Commands::Config { cmd } => config(cmd),
    }
}

// ─── auth ─────────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd consumed by match; data ignored since all arms bail.
#[allow(clippy::needless_pass_by_value)]
fn auth(cmd: AuthCommands) -> Result<()> {
    match cmd {
        AuthCommands::Login(_) => bail!("not yet implemented: auth login"),
        AuthCommands::Logout { .. } => bail!("not yet implemented: auth logout"),
        AuthCommands::Status => bail!("not yet implemented: auth status"),
        AuthCommands::ShowKey => bail!("not yet implemented: auth show-key"),
        AuthCommands::Switch { .. } => bail!("not yet implemented: auth switch"),
    }
}

// ─── stats ────────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd consumed by match; data ignored since all arms bail.
#[allow(clippy::needless_pass_by_value)]
fn stats(cmd: StatsCommands) -> Result<()> {
    match cmd {
        StatsCommands::Today(_) => bail!("not yet implemented: stats today"),
        StatsCommands::Yesterday(_) => bail!("not yet implemented: stats yesterday"),
        StatsCommands::Week(_) => bail!("not yet implemented: stats week"),
        StatsCommands::Month(_) => bail!("not yet implemented: stats month"),
        StatsCommands::Year(_) => bail!("not yet implemented: stats year"),
        StatsCommands::Range { .. } => bail!("not yet implemented: stats range"),
    }
}

// ─── projects ─────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd consumed by match; data ignored since all arms bail.
#[allow(clippy::needless_pass_by_value)]
fn projects(cmd: ProjectsCommands) -> Result<()> {
    match cmd {
        ProjectsCommands::List { .. } => bail!("not yet implemented: projects list"),
        ProjectsCommands::Top { .. } => bail!("not yet implemented: projects top"),
        ProjectsCommands::Show { .. } => bail!("not yet implemented: projects show"),
    }
}

// ─── languages ────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd consumed by match; data ignored since all arms bail.
#[allow(clippy::needless_pass_by_value)]
fn languages(cmd: LanguagesCommands) -> Result<()> {
    match cmd {
        LanguagesCommands::List { .. } => bail!("not yet implemented: languages list"),
        LanguagesCommands::Top { .. } => bail!("not yet implemented: languages top"),
    }
}

// ─── editors ──────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd consumed by match; data ignored since all arms bail.
#[allow(clippy::needless_pass_by_value)]
fn editors(cmd: EditorsCommands) -> Result<()> {
    match cmd {
        EditorsCommands::List { .. } => bail!("not yet implemented: editors list"),
        EditorsCommands::Top { .. } => bail!("not yet implemented: editors top"),
    }
}

// ─── goals ────────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd consumed by match; data ignored since all arms bail.
#[allow(clippy::needless_pass_by_value)]
fn goals(cmd: GoalsCommands) -> Result<()> {
    match cmd {
        GoalsCommands::List => bail!("not yet implemented: goals list"),
        GoalsCommands::Show { .. } => bail!("not yet implemented: goals show"),
        GoalsCommands::Watch { .. } => bail!("not yet implemented: goals watch"),
    }
}

// ─── leaderboard ──────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd consumed by match; data ignored since all arms bail.
#[allow(clippy::needless_pass_by_value)]
fn leaderboard(cmd: LeaderboardCommands) -> Result<()> {
    match cmd {
        LeaderboardCommands::Show { .. } => bail!("not yet implemented: leaderboard show"),
    }
}

// ─── report ───────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd consumed by match; data ignored since all arms bail.
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

// ─── config ───────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd consumed by match; data ignored since all arms bail.
#[allow(clippy::needless_pass_by_value)]
fn config(cmd: ConfigCommands) -> Result<()> {
    match cmd {
        ConfigCommands::Get { .. } => bail!("not yet implemented: config get"),
        ConfigCommands::Set { .. } => bail!("not yet implemented: config set"),
        ConfigCommands::Edit => bail!("not yet implemented: config edit"),
        ConfigCommands::Path => bail!("not yet implemented: config path"),
        ConfigCommands::Reset { .. } => bail!("not yet implemented: config reset"),
        ConfigCommands::Doctor => bail!("not yet implemented: config doctor"),
    }
}
