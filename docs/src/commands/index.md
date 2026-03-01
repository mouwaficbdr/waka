# Commands Overview

`waka` is organized into the following top-level commands:

| Command            | Description                                                |
| ------------------ | ---------------------------------------------------------- |
| `waka stats`       | Coding statistics (`today`, `week`, `month`, `range`, ...) |
| `waka projects`    | Browse and filter projects                                 |
| `waka languages`   | Browse coding languages                                    |
| `waka editors`     | Browse editors and IDEs                                    |
| `waka goals`       | View and watch coding goals                                |
| `waka leaderboard` | View the public leaderboard                                |
| `waka report`      | Generate productivity reports (md, html, json, csv)        |
| `waka dashboard`   | Interactive full-screen TUI dashboard                      |
| `waka prompt`      | Shell prompt integration (cache-only, no network)          |
| `waka completions` | Generate shell completion scripts                          |
| `waka auth`        | Manage API key and authentication                          |
| `waka cache`       | Manage the local response cache                            |
| `waka config`      | View and edit configuration                                |
| `waka update`      | Update waka to the latest version                          |
| `waka changelog`   | Show the changelog from installed version to latest        |

## Global flags

These flags are accepted by **every** subcommand:

| Flag                    | Default   | Description                                      |
| ----------------------- | --------- | ------------------------------------------------ |
| `-p, --profile <NAME>`  | `default` | Use a specific named profile                     |
| `-f, --format <FORMAT>` | `table`   | Output format: `table`, `json`, `csv`, `plain`   |
| `--no-cache`            | off       | Bypass the local cache and force a fresh request |
| `--no-color`            | off       | Disable color output (also: `NO_COLOR` env var)  |
| `--quiet`               | off       | Suppress non-essential output                    |
| `--verbose`             | off       | Enable verbose logging (shows HTTP requests)     |
| `--csv-bom`             | off       | Prepend UTF-8 BOM to CSV output (Windows Excel)  |
| `-h, --help`            | —         | Print help                                       |
| `-V, --version`         | —         | Print version                                    |

## Man pages

Every command has a corresponding man page installed alongside the binary:

```sh
man waka
man waka-stats
man waka-report
man waka-dashboard
```
