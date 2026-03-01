# waka changelog

Display the release changelog from your installed version to the latest.

## Usage

```sh
waka changelog [OPTIONS]
```

## Options

| Flag             | Description                             |
| ---------------- | --------------------------------------- |
| `--all`          | Show the full changelog (all versions)  |
| `--format <fmt>` | Output format: `text` (default), `json` |

## Examples

```sh
# Show what changed since your installed version
waka changelog

# Full history
waka changelog --all
```

## Sample output

```
## v0.5.0 — 2025-04-01

### Features
- feat(tui): add language pie chart to dashboard
- feat(report): add PDF export via wkhtmltopdf

### Fixes
- fix(cache): handle corrupted sled database gracefully

## v0.4.0 — 2025-03-15

### Features
- feat(report): implement report generation in md/html/json/csv
- feat(cli): implement update and changelog commands
...
```

The changelog is fetched from
[github.com/mouwaficbdr/waka/CHANGELOG.md](https://github.com/mouwaficbdr/waka/blob/main/CHANGELOG.md).
