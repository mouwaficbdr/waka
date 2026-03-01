# waka stats week

Display the last 7 days of coding activity.

## Usage

```sh
waka stats week [OPTIONS]
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
waka stats week
waka stats week --project my-saas --format json
waka stats week --no-cache
```

## Sample output

```
╭─────────────────────────────────────────────────────────────╮
│  This week — Mon 24 Feb → Sat 1 Mar 2025                    │
├───────────┬─────────┬───────────────────────────────────────┤
│  Day      │  Total  │  Bar                                  │
├───────────┼─────────┤───────────────────────────────────────┤
│  Mon      │  6h 12m │  ████████████████                     │
│  Tue      │  4h 47m │  ████████████                         │
│  Wed      │  7h 03m │  ██████████████████                   │
│  Thu      │  5h 30m │  ██████████████                       │
│  Fri      │  3h 15m │  ████████                             │
│  Sat      │  5h 42m │  ██████████████                       │
├───────────┼─────────┤───────────────────────────────────────┤
│  Total    │ 32h 29m │                                       │
╰───────────┴─────────┴───────────────────────────────────────╯
```
