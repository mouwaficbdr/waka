# Configuration

`waka` stores its configuration in `~/.config/waka/config.toml` (XDG-compliant).

## Viewing and editing settings

```sh
waka config get            # list all settings
waka config get ui.color   # get a single key
waka config set ui.color always
waka config edit           # open in $EDITOR
waka config path           # print the path to config.toml
```

## Configuration reference

```toml
[ui]
# Color output: "auto" (default), "always", or "never"
color = "auto"

# Table style: "rounded" (default), "sharp", "minimal", "markdown"
table_style = "rounded"

# Output format for non-table commands: "text" (default) or "json"
format = "text"

[api]
# WakaTime API base URL (only change for self-hosted instances)
base_url = "https://wakatime.com/api/v1/"

# Request timeout in seconds (default: 10)
timeout = 10

[cache]
# Enable or disable local response caching
enabled = true

# Time-to-live for cached responses in seconds (default: 300 = 5 min)
ttl = 300
```

## Environment variables

All settings can be overridden with environment variables:

| Variable        | Equivalent setting | Notes                         |
| --------------- | ------------------ | ----------------------------- |
| `WAKA_API_KEY`  | —                  | API key (bypasses keychain)   |
| `WAKA_BASE_URL` | `api.base_url`     |                               |
| `WAKA_COLOR`    | `ui.color`         | `auto`, `always`, `never`     |
| `WAKA_FORMAT`   | `ui.format`        | `text`, `json`                |
| `NO_COLOR`      | —                  | Standard — disables all color |

## Config file location

| OS      | Default path                 |
| ------- | ---------------------------- |
| Linux   | `~/.config/waka/config.toml` |
| macOS   | `~/.config/waka/config.toml` |
| Windows | `%APPDATA%\waka\config.toml` |

Override with `WAKA_CONFIG_DIR`.
