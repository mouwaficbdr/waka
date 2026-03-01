# waka leaderboard

Browse the public WakaTime leaderboard.

## Usage

```sh
waka leaderboard [OPTIONS]
```

## Options

| Flag             | Description                             |
| ---------------- | --------------------------------------- |
| `--page <n>`     | Page number (1-based, default: 1)       |
| `--format <fmt>` | Output format: `text` (default), `json` |
| `--color <when>` | Color mode: `auto`, `always`, `never`   |

## Examples

```sh
waka leaderboard
waka leaderboard --page 2
waka leaderboard --format json
```

## Sample output

```
╭──────┬──────────────────────┬──────────────────────╮
│ Rank │ User                 │ Weekly Total         │
├──────┼──────────────────────┼──────────────────────┤
│   1  │ alice                │ 68 hrs 12 mins       │
│   2  │ bob_codes            │ 62 hrs 07 mins       │
│   3  │ rustacean42          │ 59 hrs 45 mins       │
│ ...  │                      │                      │
│  47  │ you (alice)          │ 32 hrs 29 mins  ◀    │
╰──────┴──────────────────────┴──────────────────────╯
Page 1/100 — run `waka leaderboard --page 2` for the next page
```
