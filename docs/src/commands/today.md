# waka stats today

Display today's coding activity summary.

## Usage

```sh
waka stats today [OPTIONS]
```

## Options

| Flag                    | Description                                              |
| ----------------------- | -------------------------------------------------------- |
| `--project <name>`      | Filter to a specific project                             |
| `--language <lang>`     | Filter to a specific language                            |
| `-f, --format <FORMAT>` | Output format: `table` (default), `json`, `csv`, `plain` |
| `--no-cache`            | Force a fresh API request                                |

## Examples

```sh
# Show today's summary
waka stats today

# Filter to a single project
waka stats today --project my-saas

# Machine-readable JSON output
waka stats today --format json

# Bypass the cache
waka stats today --no-cache
```

## Sample output

```
╭─────────────────────────────────────────────────╮
│  Today — Saturday, 1 March 2025                 │
├──────────────────────┬──────────────────────────┤
│  Total               │  5 hrs 42 mins           │
├──────────────────────┼──────────────────────────┤
│  Languages           │                          │
│    Rust              │  4 hrs 11 mins  (73 %)   │
│    TOML              │  0 hrs 44 mins  (13 %)   │
│    Markdown          │  0 hrs 47 mins  (14 %)   │
├──────────────────────┼──────────────────────────┤
│  Editors             │                          │
│    VS Code           │  5 hrs 42 mins (100 %)   │
╰──────────────────────┴──────────────────────────╯
```
