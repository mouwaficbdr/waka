# Integrations

## Shell prompt and status bars

Use [`waka prompt`](./commands/prompt.md): it reads today's total from the local cache only, so it
is safe to run on every prompt render (no network request, no API rate limit).

```sh
waka prompt                    # ⏱ 6h 42m
waka prompt --style detailed   # ⏱ 6h 42m | my-saas
```

The cached value is refreshed whenever `waka stats today` runs. To keep it current without
thinking about it, refresh it periodically, e.g. with cron:

```sh
*/15 * * * * waka stats today > /dev/null 2>&1
```

### Zsh

```sh
# ~/.zshrc
RPROMPT='$(waka prompt 2>/dev/null)'
```

### Starship

```toml
# ~/.config/starship.toml
[custom.waka]
command = "waka prompt 2>/dev/null"
when = "true"
format = "[$output]($style) "
style = "dimmed yellow"
```

### tmux

```sh
# ~/.tmux.conf
set -g status-right "#(waka prompt 2>/dev/null) | %H:%M"
```

## CI / GitHub Actions

Authenticate with the `WAKATIME_API_KEY` (or `WAKA_API_KEY`) environment variable:

```yaml
- name: Export last week's coding report
  env:
    WAKATIME_API_KEY: ${{ secrets.WAKATIME_API_KEY }}
  run: |
    waka report generate \
      --from "$(date -d '7 days ago' +%F)" --to "$(date +%F)" \
      -F json -o coding-report.json
```

## Piping output

When stdout is not a terminal, `waka` switches to plain text without colors. Use
`--format json` (or `csv`) for machine-readable output:

```sh
waka stats today --format json | jq -r '.data[0].grand_total.text'
waka projects list --format json | jq -r '.data[].name'
```
