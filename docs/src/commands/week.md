# waka week

Display this week's coding activity summary (Monday → today).

## Usage

```sh
waka week [OPTIONS]
```

## Options

| Flag               | Description                             |
| ------------------ | --------------------------------------- |
| `--project <name>` | Filter to a specific project            |
| `--format <fmt>`   | Output format: `text` (default), `json` |
| `--color <when>`   | Color mode: `auto`, `always`, `never`   |

## Examples

```sh
waka week
waka week --project my-saas --format json
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
