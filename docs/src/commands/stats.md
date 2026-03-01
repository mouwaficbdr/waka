# waka stats

Show aggregated coding statistics for a predefined time range.

## Usage

```sh
waka stats [OPTIONS] [RANGE]
```

## Arguments

| Argument | Values                                                                  | Default       |
| -------- | ----------------------------------------------------------------------- | ------------- |
| `RANGE`  | `last_7_days`, `last_30_days`, `last_6_months`, `last_year`, `all_time` | `last_7_days` |

## Options

| Flag             | Description                             |
| ---------------- | --------------------------------------- |
| `--format <fmt>` | Output format: `text` (default), `json` |
| `--color <when>` | Color mode: `auto`, `always`, `never`   |

## Examples

```sh
waka stats
waka stats last_30_days
waka stats all_time --format json
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
