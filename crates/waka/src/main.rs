//! `waka` — `WakaTime` CLI.
//!
//! Entry point for the `waka` binary. Parses CLI arguments and dispatches
//! to command handlers. All user-facing errors are printed to stderr and the
//! process exits with the code defined in SPEC.md Annexe B.

mod auth;
mod cli;
mod commands;
mod error;
mod spinner;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // `--verbose` surfaces the crates' tracing output (HTTP requests, retries,
    // cache warnings) on stderr. `RUST_LOG` takes precedence when set.
    if cli.global.verbose || std::env::var_os("RUST_LOG").is_some() {
        let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            tracing_subscriber::EnvFilter::new(
                "waka=debug,waka_api=debug,waka_cache=debug,waka_config=debug,waka_tui=debug",
            )
        });
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .without_time()
            .init();
    }

    if let Err(err) = commands::dispatch(cli.command, cli.global).await {
        eprintln!("{}", error::format_error(&err));
        std::process::exit(error::exit_code(&err));
    }
}
