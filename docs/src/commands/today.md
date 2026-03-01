# waka today

Display today's coding activity summary.

## Usage

```sh
waka today [OPTIONS]
```

## Options

| Flag               | Description                             |
| ------------------ | --------------------------------------- |
| `--project <name>` | Filter to a specific project            |
| `--format <fmt>`   | Output format: `text` (default), `json` |
| `--color <when>`   | Color mode: `auto`, `always`, `never`   |

## Examples

```sh
# Show today's summary
waka today

# Filter to a single project
waka today --project my-saas

# Machine-readable JSON output
waka today --format json
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
