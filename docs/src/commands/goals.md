# waka goals

Display your WakaTime coding goals and their current progress.

## Usage

```sh
waka goals [OPTIONS]
```

## Options

| Flag             | Description                             |
| ---------------- | --------------------------------------- |
| `--format <fmt>` | Output format: `text` (default), `json` |
| `--color <when>` | Color mode: `auto`, `always`, `never`   |

## Examples

```sh
waka goals
waka goals --format json
```

## Sample output

```
╭─────────────────────────────────────────────────────────────╮
│  Goals (3)                                                  │
├───────────────────────────────┬─────────┬──────────┬────────┤
│  Goal                         │  Target │  Current │  Status│
├───────────────────────────────┼─────────┼──────────┼────────┤
│  Code at least 4 hrs/day      │  4h/day │  5h 42m  │  ✓     │
│  Rust > 70% of weekly coding  │  70 %   │  77 %    │  ✓     │
│  10 hrs/week on my-saas       │  10h    │  6h 30m  │  ✗     │
╰───────────────────────────────┴─────────┴──────────┴────────╯
```

Goals are created and managed on [wakatime.com/goals](https://wakatime.com/goals).
