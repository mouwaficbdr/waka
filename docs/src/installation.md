# Installation

## Homebrew (macOS / Linux)

```sh
brew install mouwaficbdr/tap/waka
```

## Cargo (all platforms)

Requires Rust 1.82.0 or later.

```sh
cargo install waka
```

## Pre-built binaries

Download the latest binary for your platform from the
[GitHub Releases](https://github.com/mouwaficbdr/waka/releases) page.

| Platform       | File                              |
| -------------- | --------------------------------- |
| Linux x86-64   | `waka-x86_64-unknown-linux-musl`  |
| Linux ARM64    | `waka-aarch64-unknown-linux-musl` |
| macOS x86-64   | `waka-x86_64-apple-darwin`        |
| macOS ARM64    | `waka-aarch64-apple-darwin`       |
| Windows x86-64 | `waka-x86_64-pc-windows-msvc.exe` |

Extract and place the binary on your `$PATH`.

## Verify the installation

```sh
waka --version
```

## Next step

[Authenticate with your WakaTime API key →](./authentication.md)
