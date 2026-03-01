# waka report

Generate a detailed coding report and save it to a file.

## Usage

```sh
waka report [OPTIONS] --output <FILE>
```

## Options

| Flag                  | Description                                                                                   |
| --------------------- | --------------------------------------------------------------------------------------------- |
| `--output <FILE>`     | Output file path (required)                                                                   |
| `--format <fmt>`      | Report format: `markdown` (default), `html`, `json`, `csv`                                    |
| `--range <range>`     | Time range: `last_7_days` (default), `last_30_days`, `last_6_months`, `last_year`, `all_time` |
| `--from <YYYY-MM-DD>` | Custom start date                                                                             |
| `--to <YYYY-MM-DD>`   | Custom end date                                                                               |
| `--project <name>`    | Filter to a specific project                                                                  |

## Examples

```sh
# Markdown report for the last 7 days
waka report --output report.md

# HTML report for last month
waka report --format html --range last_30_days --output report.html

# JSON for programmatic processing
waka report --format json --output report.json

# CSV for spreadsheet import
waka report --format csv --output report.csv

# Custom date range
waka report --from 2025-02-01 --to 2025-02-28 --output feb-report.md
```

## Report contents

Every report includes:

- **Summary** — total time, daily average, best day
- **Languages** — time per language with percentages
- **Projects** — time per project with percentages
- **Editors** — time per editor
- **Operating Systems** — time per OS
- **Machines** — time per machine / hostname
- **Daily breakdown** — per-day totals for the requested range
