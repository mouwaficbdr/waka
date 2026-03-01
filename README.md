# waka

[![CI](https://github.com/mouwaficbdr/waka/actions/workflows/ci.yml/badge.svg)](https://github.com/mouwaficbdr/waka/actions/workflows/ci.yml)
[![Security Audit](https://github.com/mouwaficbdr/waka/actions/workflows/audit.yml/badge.svg)](https://github.com/mouwaficbdr/waka/actions/workflows/audit.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/waka-cli.svg)](https://crates.io/crates/waka-cli)

**The WakaTime CLI you always deserved.** Fast, beautiful, composable.

---

## Installation

```bash
# Cargo
cargo install waka-cli

# macOS / Linux (Homebrew)
brew tap mouwaficbdr/waka
brew install waka

# Universal installer
curl -sSfL https://github.com/mouwaficbdr/waka/releases/latest/download/waka-installer.sh | sh
```

## Quick Start

```bash
# Set up your API key (from https://wakatime.com/settings/api-key)
waka auth login

# Today's coding stats
waka stats today

# Last 7 days
waka stats week

# Output as JSON and pipe to jq
waka stats today --format json | jq '.languages[0]'

# Export to CSV (with Windows BOM for Excel)
waka stats week --format csv --csv-bom > week.csv
```

## Features

_All features listed here are fully implemented._

### Authentication

```bash
waka auth login [--api-key <KEY>] [--profile <NAME>]
waka auth logout [--profile <NAME>]
waka auth status
waka auth show-key
waka auth switch <PROFILE>
```

Multi-profile support: keep separate `work` and `personal` profiles.
API keys are stored securely in the OS keychain (macOS Keychain, GNOME Keyring, Windows Credential Manager) with a plain-text fallback at `600` permissions.

### Coding Stats

```bash
waka stats today
waka stats yesterday
waka stats week
waka stats month
waka stats year
waka stats range --from 2024-01-01 --to 2024-01-31

# Filter by project or language
waka stats week --project my-app --language Rust
```

### Projects, Languages & Editors

```bash
waka projects list [--sort-by time|name] [--limit N]
waka projects top [--period 7d|30d|1y]
waka projects show <PROJECT> [--from DATE] [--to DATE]

waka languages list [--period 7d]
waka languages top [--limit N]

waka editors list [--period 7d]
waka editors top [--limit N]
```

### Goals

```bash
waka goals list
waka goals show <GOAL_ID>
waka goals watch [--notify] [--interval SECONDS]
```

### Leaderboard

```bash
waka leaderboard show [--page N]
```

### Reports

```bash
waka report generate --from 2024-01-01 --to 2024-01-31 [--format md|html|json|csv] [-o report.md]
waka report summary [--period week|month]
```

### Cache Management

```bash
waka cache info          # entry count, disk usage, last write
waka cache path          # print the cache directory
waka cache clear         # remove all cached entries
waka cache clear --older 24h   # remove entries older than 24h
```

Cache is TTL-aware and stored per-profile under the platform cache directory:

| Platform | Path                               |
| -------- | ---------------------------------- |
| Linux    | `~/.cache/waka/<profile>/`         |
| macOS    | `~/Library/Caches/waka/<profile>/` |
| Windows  | `%LOCALAPPDATA%\waka\<profile>\`   |

### Diagnostics

```bash
waka config doctor   # full diagnostic: API key, network, cache, shell completions, update check
```

### Shell Prompt Integration

`waka prompt` reads today's total from the local cache only — no network call, always fast.

```bash
waka prompt                   # ⏱ 6h 42m
waka prompt --format detailed # ⏱ 6h 42m | my-saas
```

**Starship** ([starship.rs](https://starship.rs/)) module:

```toml
# ~/.config/starship.toml
[custom.waka]
command = "waka prompt --format simple 2>/dev/null"
when = "true"
format = "[$output]($style) "
style = "dimmed yellow"
```

**tmux** status bar:

```bash
# ~/.tmux.conf
set -g status-right "#(waka prompt 2>/dev/null) | %H:%M"
```

### Shell Completions

```bash
# Bash
waka completions bash >> ~/.bashrc

# Zsh
waka completions zsh > ~/.zsh/completions/_waka

# Fish
waka completions fish > ~/.config/fish/completions/waka.fish

# PowerShell
waka completions powershell | Out-String | Invoke-Expression

# Elvish
waka completions elvish > ~/.config/elvish/lib/waka.elv
```

## Output Formats

Every tabular command supports `--format`:

| Format | Flag             | Notes                                                      |
| ------ | ---------------- | ---------------------------------------------------------- |
| Table  | `--format table` | Default when stdout is a TTY                               |
| Plain  | `--format plain` | Default when piped                                         |
| JSON   | `--format json`  | Machine-readable                                           |
| CSV    | `--format csv`   | Spreadsheet-friendly; add `--csv-bom` for Excel on Windows |
| TSV    | `--format tsv`   | Tab-separated                                              |

Color output respects `NO_COLOR`, `TERM=dumb`, and `--no-color`.

## Global Options

| Flag                    | Description                         |
| ----------------------- | ----------------------------------- |
| `-p, --profile <NAME>`  | Use a specific profile              |
| `-f, --format <FORMAT>` | Output format                       |
| `--no-cache`            | Skip cache, force fresh API request |
| `--no-color`            | Disable colors                      |
| `--quiet`               | Suppress non-essential output       |
| `--verbose`             | Show HTTP request details           |
| `--csv-bom`             | Prepend UTF-8 BOM to CSV output     |

## Roadmap

See [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md) for the full phased roadmap.

Coming next: TUI interactive dashboard, goal progress bars with notifications,
update checker.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

## License

MIT — see [LICENSE](LICENSE).
