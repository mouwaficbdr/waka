# Integrations

## Shell prompt integration

Display your daily coding total in your shell prompt.

### Zsh (Powerlevel10k / custom)

Add to your `.zshrc`:

```sh
waka_prompt() {
    local total
    total=$(waka today --format json 2>/dev/null \
        | python3 -c "import sys,json; d=json.load(sys.stdin); \
          print(d['data'][0]['grand_total']['text'])" 2>/dev/null)
    [[ -n "$total" ]] && echo " %F{cyan}⌚ $total%f"
}
# Append $(waka_prompt) to your PROMPT or RPROMPT
```

### starship

Add to `~/.config/starship.toml`:

```toml
[custom.waka]
command = "waka today --format json | python3 -c \"import sys,json; d=json.load(sys.stdin); print(d['data'][0]['grand_total']['text'])\" 2>/dev/null"
when = "true"
format = "⌚ [$output]($style) "
style = "cyan"
```

## tmux status bar

Add to `~/.tmux.conf`:

```sh
set -g status-right "#(waka today --format json 2>/dev/null | python3 -c \"import sys,json; d=json.load(sys.stdin); print(d['data'][0]['grand_total']['text'])\" 2>/dev/null) | %H:%M"
set -g status-interval 300   # refresh every 5 min
```

## CI / GitHub Actions

Use `WAKA_API_KEY` to authenticate in CI:

```yaml
- name: Export coding report
  env:
      WAKA_API_KEY: ${{ secrets.WAKA_API_KEY }}
  run: waka report --range last_7_days --format json --output coding-report.json
```

## Piping output

`waka` detects when stdout is piped and automatically switches to plain-text mode with no colors:

```sh
waka today --format json | jq '.data[0].grand_total.text'
waka projects --format json | jq '.[].name'
```
