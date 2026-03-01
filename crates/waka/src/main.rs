//! `waka` — `WakaTime` CLI.
//!
//! Entry point for the `waka` binary. Parses CLI arguments and dispatches
//! to command handlers. All user-facing errors are printed to stderr.

fn main() {
    println!("waka v{}", env!("CARGO_PKG_VERSION"));
}
