//! Build script for `waka`.
//!
//! Emits `rerun-if-changed` directives so Cargo re-runs this script when the
//! CLI definitions change. Shell completions are generated at **runtime** via
//! `waka completions <shell>` which is the standard practice for clap-based
//! tools whose CLI is defined inside the binary.
//!
//! To regenerate static completion files for distribution, run:
//! ```sh
//! cargo run -- completions bash        > completions/waka.bash
//! cargo run -- completions zsh         > completions/_waka
//! cargo run -- completions fish        > completions/waka.fish
//! cargo run -- completions powershell  > completions/_waka.ps1
//! cargo run -- completions elvish      > completions/waka.elv
//! ```

fn main() {
    // Re-run this build script if the CLI definitions change.
    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=build.rs");
}
