# waka stats

Show coding statistics for a predefined or custom time range.

## Subcommands

| Subcommand                | Description                                          |
| ------------------------- | ---------------------------------------------------- |
| `waka stats today`        | Today's coding activity                              |
| `waka stats yesterday`    | Yesterday's coding activity                          |
| `waka stats week`         | Last 7 days                                          |
| `waka stats month`        | Last 30 days                                         |
| `waka stats year`         | Last 12 months                                       |
| `waka stats range`        | Custom date range (requires `--from` and `--to`)     |

## Usage

```sh
waka stats <SUBCOMMAND> [OPTIONS]
```

## Common options

These options apply to all `waka stats` subcommands:

| Flag                    | Description                                              |
| ----------------------- | -------------------------------------------------------- |
| `--project <name>`      | Filter results to a specific project                     |
| `--language <lang>`     | Filter results to a specific programming language        |
| `-f, --format <FORMAT>` | Output format: `table` (default), `json`, `csv`, `plain` |
| `--no-cache`            | Bypass the local cache and force a fresh API request     |

## `waka stats range` options

| Flag            | Required | Description                 |
| --------------- | -------- | --------------------------- |
| `--from <DATE>` | yes      | Start date in `YYYY-MM-DD`  |
| `--to <DATE>`   | yes      | End date in `YYYY-MM-DD`    |

## Examples

```sh
# Built-in ranges
waka stats today
waka stats yesterday
waka stats week
waka stats month
waka stats year

# Custom range
waka stats range --from 2024-01-01 --to 2024-03-31

# Filter to a project and output as JSON
waka stats week --project my-saas --format json

# Bypass cache
waka stats today --no-cache
```

## Sample output

```
╭─────────────────────────────────────────────────╮
│  Stats — Last 7 days                            │
├──────────────────────┬──────────────────────────┤
│  Total               │  32 hrs 29 mins          │
│  Daily average       │  4 hrs 38 mins           │
│  Best day            │  Wed 26 Feb (7 hrs 3 min)│
├──────────────────────┼──────────────────────────┤
│  Top Languages       │                          │
│    Rust              │  25 hrs 14 mins  (77 %)  │
│    TOML              │   3 hrs 11 mins  (10 %)  │
│    Markdown          │   4 hrs 04 mins  (13 %)  │
╰──────────────────────┴──────────────────────────╯
```
