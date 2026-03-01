# Commands Overview

`waka` is organized into the following subcommands:

| Command            | Description                                    |
| ------------------ | ---------------------------------------------- |
| `waka today`       | Today's coding summary                         |
| `waka week`        | This week's coding summary                     |
| `waka stats`       | Aggregated stats for a predefined range        |
| `waka projects`    | List all projects                              |
| `waka goals`       | List your coding goals                         |
| `waka leaderboard` | Browse the public leaderboard                  |
| `waka report`      | Generate a coding report (md, html, json, csv) |
| `waka dashboard`   | Open the interactive TUI dashboard             |
| `waka auth`        | Manage authentication                          |
| `waka config`      | View and edit configuration                    |
| `waka update`      | Update waka to the latest version              |
| `waka changelog`   | Show recent changes                            |

## Global flags

These flags are accepted by every subcommand:

| Flag              | Default | Description                             |
| ----------------- | ------- | --------------------------------------- |
| `--color <when>`  | `auto`  | Color output: `auto`, `always`, `never` |
| `--format <fmt>`  | `text`  | Output format: `text`, `json`           |
| `--api-key <key>` | —       | Override the configured API key         |
| `-v, --verbose`   | off     | Enable verbose/debug logging            |
| `-h, --help`      | —       | Print help                              |
| `-V, --version`   | —       | Print version                           |

## Man pages

Every subcommand has a corresponding man page installed alongside the binary:

```sh
man waka
man waka-today
man waka-report
```
