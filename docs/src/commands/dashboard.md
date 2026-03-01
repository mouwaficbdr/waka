# waka dashboard

Open the interactive full-screen TUI dashboard.

## Usage

```sh
waka dashboard [OPTIONS]
```

## Options

| Flag             | Description                           |
| ---------------- | ------------------------------------- |
| `--color <when>` | Color mode: `auto`, `always`, `never` |

## Controls

| Key         | Action              |
| ----------- | ------------------- |
| `q` / `Esc` | Quit                |
| `Tab`       | Switch panel        |
| `←` / `→`   | Change date range   |
| `r`         | Refresh data        |
| `h`         | Toggle help overlay |

## Example

```sh
waka dashboard
```

The dashboard displays:

- **Top-left** — weekly coding bar chart
- **Top-right** — language breakdown (pie / bar)
- **Bottom** — project list with time bars
- **Status bar** — current range and last refresh timestamp

> **Note:** The dashboard requires a terminal that supports at least 80×24 characters and 256 colors
> (most modern terminals qualify). The TUI does not fall back gracefully in very small terminals.
