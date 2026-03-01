//! Build script for `waka`.
//!
//! Generates man pages using `clap_mangen` and writes them to:
//! - `$OUT_DIR/man/` (for Cargo internal use / CI artifacts)
//! - `man/` (workspace-level, committed for packaging)
//!
//! Shell completions are generated at **runtime** via `waka completions <shell>`
//! which is the standard practice for clap-based tools. To regenerate static
//! completion files for distribution, run:
//! ```sh
//! cargo run -- completions bash        > completions/waka.bash
//! cargo run -- completions zsh         > completions/_waka
//! cargo run -- completions fish        > completions/waka.fish
//! cargo run -- completions powershell  > completions/_waka.ps1
//! cargo run -- completions elvish      > completions/waka.elv
//! ```

use std::io::Error;
use std::path::PathBuf;

/// Returns the `waka` top-level `clap::Command` for man page generation.
///
/// Only name, version, about and populated subcommand names are needed —
/// argument details are shown at runtime via `--help`. This is sufficient for
/// useful section-1 man pages.
fn build_command() -> clap::Command {
    use clap::Command;

    Command::new("waka")
        .version(env!("CARGO_PKG_VERSION"))
        .about("WakaTime CLI — track your coding time from the terminal")
        .subcommand(
            Command::new("auth")
                .about("Manage API key and authentication")
                .subcommand(Command::new("login").about("Log in with your WakaTime API key"))
                .subcommand(Command::new("logout").about("Remove the stored API key"))
                .subcommand(Command::new("status").about("Show current authentication status"))
                .subcommand(Command::new("whoami").about("Display the authenticated user info")),
        )
        .subcommand(
            Command::new("stats")
                .about("Show coding statistics")
                .subcommand(Command::new("today").about("Show today's coding stats"))
                .subcommand(Command::new("yesterday").about("Show yesterday's coding stats"))
                .subcommand(Command::new("week").about("Show stats for the last 7 days"))
                .subcommand(Command::new("month").about("Show stats for the last 30 days"))
                .subcommand(Command::new("year").about("Show stats for the last 365 days"))
                .subcommand(Command::new("all-time").about("Show all-time coding stats"))
                .subcommand(Command::new("range").about("Show stats for a custom date range")),
        )
        .subcommand(
            Command::new("projects")
                .about("Browse and filter projects")
                .subcommand(Command::new("list").about("List all projects"))
                .subcommand(Command::new("top").about("Show top projects by time coded")),
        )
        .subcommand(
            Command::new("languages")
                .about("Browse coding languages")
                .subcommand(Command::new("list").about("List all languages"))
                .subcommand(Command::new("top").about("Show top languages by time coded")),
        )
        .subcommand(
            Command::new("editors")
                .about("Browse editors and IDEs")
                .subcommand(Command::new("list").about("List all editors"))
                .subcommand(Command::new("top").about("Show top editors by time coded")),
        )
        .subcommand(
            Command::new("goals")
                .about("View and watch coding goals")
                .subcommand(Command::new("list").about("List all goals"))
                .subcommand(Command::new("watch").about("Watch goals with live updates")),
        )
        .subcommand(
            Command::new("leaderboard")
                .about("View the WakaTime leaderboard")
                .subcommand(Command::new("show").about("Show the public leaderboard")),
        )
        .subcommand(
            Command::new("report")
                .about("Generate productivity reports")
                .subcommand(Command::new("generate").about("Generate a report for a date range"))
                .subcommand(Command::new("summary").about("Show a brief productivity summary")),
        )
        .subcommand(Command::new("dashboard").about("Launch the interactive TUI dashboard"))
        .subcommand(
            Command::new("prompt")
                .about("Shell prompt integration (reads from cache only, no network)"),
        )
        .subcommand(Command::new("completions").about("Generate shell completions"))
        .subcommand(
            Command::new("config")
                .about("Manage waka configuration")
                .subcommand(Command::new("show").about("Print the current configuration"))
                .subcommand(Command::new("edit").about("Open the config file in $EDITOR"))
                .subcommand(Command::new("set").about("Set a configuration value"))
                .subcommand(Command::new("reset").about("Reset configuration to defaults")),
        )
        .subcommand(
            Command::new("cache")
                .about("Manage the local response cache")
                .subcommand(Command::new("clear").about("Clear all cached data"))
                .subcommand(Command::new("info").about("Show cache statistics"))
                .subcommand(Command::new("path").about("Print the cache directory path")),
        )
        .subcommand(Command::new("update").about("Update waka to the latest version"))
        .subcommand(
            Command::new("changelog")
                .about("Show the changelog from the installed version to the latest"),
        )
}

fn main() -> Result<(), Error> {
    // Re-run this build script if the CLI or the build script itself changes.
    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);
    let man_out = out_dir.join("man");
    std::fs::create_dir_all(&man_out)?;

    // Also write to workspace `man/` so generated pages can be committed and
    // used for packaging (distros, Homebrew formula, etc.).
    let manifest_dir =
        PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").ok_or(std::io::ErrorKind::NotFound)?);
    let man_dir = manifest_dir.join("../../man");
    std::fs::create_dir_all(&man_dir)?;

    // Generate man pages for the top-level command and all sub-commands.
    clap_mangen::generate_to(build_command(), &man_out)?;
    clap_mangen::generate_to(build_command(), &man_dir)?;

    Ok(())
}
