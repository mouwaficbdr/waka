//! `waka` — `WakaTime` CLI.
//!
//! Entry point for the `waka` binary. Parses CLI arguments and dispatches
//! to command handlers. All user-facing errors are printed to stderr and the
//! process exits with the code defined in SPEC.md Annexe B.

mod auth;
mod cli;
mod commands;
mod error;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(err) = commands::dispatch(cli.command, cli.global).await {
        eprintln!("Error: {err:#}");
        std::process::exit(error::exit_code(&err));
    }
}
