# Installation

## Homebrew (macOS / Linux)

```sh
brew tap mouwaficbdr/waka
brew install waka
```

## Pre-built binaries

Download the archive for your platform from the
[latest GitHub release](https://github.com/mouwaficbdr/waka/releases/latest).
Asset names include the version, e.g. `waka-v2.0.2-x86_64-unknown-linux-gnu.tar.gz`.

| Platform            | Archive                                   |
| ------------------- | ----------------------------------------- |
| Linux x86-64        | `waka-<version>-x86_64-unknown-linux-gnu.tar.gz`  |
| Linux ARM64         | `waka-<version>-aarch64-unknown-linux-gnu.tar.gz` |
| macOS Intel         | `waka-<version>-x86_64-apple-darwin.tar.gz`       |
| macOS Apple Silicon | `waka-<version>-aarch64-apple-darwin.tar.gz`      |
| Windows x86-64      | `waka-<version>-x86_64-pc-windows-msvc.zip`       |

Extract the archive and place the `waka` binary on your `$PATH`.

> The `waka` binary is not published on crates.io, so `cargo install waka` does
> not work. The `waka-api` library crate is published separately.

## Verify the installation

```sh
waka --version
```

## Next step

[Authenticate with your WakaTime API key →](./authentication.md)
