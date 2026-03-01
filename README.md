# waka

The WakaTime CLI you always deserved. Fast, beautiful, composable.

[![CI](https://github.com/mouwaficbdr/waka/actions/workflows/ci.yml/badge.svg)](https://github.com/mouwaficbdr/waka/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/waka-cli.svg)](https://crates.io/crates/waka-cli)
[![Crates.io Downloads](https://img.shields.io/crates/d/waka-cli.svg)](https://crates.io/crates/waka-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

![waka — WakaTime CLI](demo.gif)

---

## Installation

**Cargo:**

```bash
cargo install waka-cli
```

**Homebrew (macOS / Linux):**

```bash
brew tap mouwaficbdr/waka
brew install waka
```

**Universal installer (cargo-dist):**

```bash
curl -sSfL https://github.com/mouwaficbdr/waka/releases/latest/download/waka-installer.sh | sh
```

---

## Quick Start

```bash
waka auth login          # authenticate with your WakaTime API key
waka stats today         # view today's coding activity
waka stats week          # view the last 7 days
waka dashboard           # launch the interactive TUI
```

---

## Commands

### `waka auth` — Authentication

| Command                 | Description                                                    |
| ----------------------- | -------------------------------------------------------------- |
| `auth login`            | Log in with your WakaTime API key (interactive or `--api-key`) |
| `auth logout`           | Remove the stored API key                                      |
| `auth status`           | Show whether you are currently logged in                       |
| `auth show-key`         | Display the stored API key (masked by default)                 |
| `auth switch <PROFILE>` | Switch to a different profile                                  |

API keys are stored in the OS keychain (macOS Keychain, GNOME Keyring, Windows Credential Manager) with a `0600` plain-text fallback. Multi-profile support: use `-p work` or `-p personal` on any command.

### `waka stats` — Coding Statistics

| Command                             | Description                    |
| ----------------------------------- | ------------------------------ |
| `stats today`                       | Today's coding activity        |
| `stats yesterday`                   | Yesterday's coding activity    |
| `stats week`                        | Last 7 days                    |
| `stats month`                       | Last 30 days                   |
| `stats year`                        | Last 365 days                  |
| `stats range --from DATE --to DATE` | Custom date range (YYYY-MM-DD) |

All stats subcommands accept `--project <NAME>` and `--language <LANG>` filters.

### `waka projects` — Projects

| Command                   | Description                        |
| ------------------------- | ---------------------------------- |
| `projects list`           | List all projects with coding time |
| `projects top`            | Show the most active projects      |
| `projects show <PROJECT>` | Detailed stats for one project     |

`projects list` accepts `--sort-by time|name` and `--limit N`.  
`projects top` accepts `--period 7d|30d|1y`.  
`projects show` accepts `--from DATE` and `--to DATE`.

### `waka languages` — Languages

| Command          | Description                         |
| ---------------- | ----------------------------------- |
| `languages list` | List all languages with coding time |
| `languages top`  | Show top languages                  |

`languages list` accepts `--period 7d|30d|1y`. `languages top` accepts `--limit N`.

### `waka editors` — Editors

| Command        | Description                       |
| -------------- | --------------------------------- |
| `editors list` | List all editors with coding time |
| `editors top`  | Show top editors                  |

`editors list` accepts `--period 7d|30d|1y`. `editors top` accepts `--limit N`.

### `waka goals` — Goals

| Command                | Description                                    |
| ---------------------- | ---------------------------------------------- |
| `goals list`           | List all active goals                          |
| `goals show <GOAL_ID>` | Show details for a specific goal               |
| `goals watch`          | Refresh goals periodically (`--interval SECS`) |

`goals watch --notify` sends a desktop notification when a goal is reached (requires `notify-send` on Linux).

### `waka leaderboard` — Leaderboard

| Command            | Description                 |
| ------------------ | --------------------------- |
| `leaderboard show` | Show the public leaderboard |

Accepts `--page N` for pagination.

### `waka report` — Reports

| Command                                 | Description                    |
| --------------------------------------- | ------------------------------ |
| `report generate --from DATE --to DATE` | Generate a productivity report |
| `report summary`                        | Brief productivity summary     |

`report generate` accepts `-F/--output-format md|html|json|csv` and `-o FILE`.  
`report summary` accepts `--period week|month`.

### `waka dashboard` — Interactive TUI

```bash
waka dashboard [--refresh SECONDS]
```

Live TUI dashboard powered by [ratatui](https://ratatui.rs/):

- 5 views: Main (overview), Projects, Languages, Goals, Activity (30-day heatmap)
- Auto-refresh every 60 seconds (configurable with `--refresh`)
- Keyboard navigation: `Tab` / `1–5` to switch views, `↑↓` to scroll
- `r` to refresh, `e` to export current view to JSON, `?` for help, `q` / Esc to quit

### `waka prompt` — Shell Prompt Integration

```bash
waka prompt [--format simple|detailed]
```

Reads today's total from the local cache only — no network call, always fast.

```
⏱ 6h 42m                   # simple (default)
⏱ 6h 42m | my-saas          # detailed
```

### `waka completions` — Shell Completions

```bash
waka completions bash        # Bash
waka completions zsh         # Zsh
waka completions fish        # Fish
waka completions powershell  # PowerShell
waka completions elvish      # Elvish
```

### `waka config` — Configuration

| Command                    | Description                       |
| -------------------------- | --------------------------------- |
| `config get <KEY>`         | Get the value of a config key     |
| `config set <KEY> <VALUE>` | Set the value of a config key     |
| `config edit`              | Open the config file in `$EDITOR` |
| `config path`              | Print the path to the config file |
| `config reset`             | Reset config to defaults          |
| `config doctor`            | Run a full diagnostic check       |

### `waka cache` — Cache Management

| Command       | Description                         |
| ------------- | ----------------------------------- |
| `cache info`  | Entry count, disk usage, last write |
| `cache path`  | Print the cache directory path      |
| `cache clear` | Remove all cached entries           |

`cache clear --older <DURATION>` removes only entries older than a given duration (e.g. `24h`, `7d`).

Cache location by platform:

| Platform | Path                               |
| -------- | ---------------------------------- |
| Linux    | `~/.cache/waka/<profile>/`         |
| macOS    | `~/Library/Caches/waka/<profile>/` |
| Windows  | `%LOCALAPPDATA%\waka\<profile>\`   |

### `waka update` — Self-Update

```bash
waka update
```

Updates waka to the latest release.

### `waka changelog` — Changelog

```bash
waka changelog
```

Shows the changelog from the installed version to the latest.

---

## Output Formats

Every tabular command supports `--format`:

| Format | Flag             | Notes                                                      |
| ------ | ---------------- | ---------------------------------------------------------- |
| Table  | `--format table` | Default when stdout is a TTY                               |
| Plain  | `--format plain` | Default when piped                                         |
| JSON   | `--format json`  | Machine-readable                                           |
| CSV    | `--format csv`   | Spreadsheet-friendly; add `--csv-bom` for Excel on Windows |

TSV is also supported and can be set as the default via `output.format = "tsv"` in the config file.

Color output respects `NO_COLOR`, `TERM=dumb`, and `--no-color`.

---

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

---

## Shell Integration

**Starship** ([starship.rs](https://starship.rs/)):

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

---

## `waka-api` — Rust Library

The HTTP client is available as a standalone crate for Rust developers who want to build their own WakaTime integrations:

```toml
# Cargo.toml
[dependencies]
waka-api = "1"
```

```rust
use waka_api::{WakaClient, SummaryParams};

let client = WakaClient::new("your-api-key");
let summary = client.summaries(SummaryParams::today()).await?;
```

Full documentation at [docs.rs/waka-api](https://docs.rs/waka-api).

---

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request — it covers the development setup, commit format, and coding standards used in this project.

## License

MIT — see [LICENSE](LICENSE).
