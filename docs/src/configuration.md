# Configuration

`waka` stores its configuration in a TOML file. The file is optional: when it
does not exist, defaults are used. Run `waka config path` to see where it lives
on your machine.

## Viewing and editing settings

```sh
waka config get                       # print the whole config
waka config get cache.ttl_seconds     # print a single key
waka config set cache.ttl_seconds 600 # set a value (validated before saving)
waka config edit                      # open in $VISUAL / $EDITOR
waka config path                      # print the path to config.toml
waka config reset                     # restore defaults (asks for confirmation)
```

Keys use dotted paths. `waka config set` parses the value according to the
key's type and rejects unknown keys or invalid values without touching the file.

If the file contains invalid TOML, commands stop with an error pointing at the
problem instead of silently falling back to defaults. `waka config doctor`
reports it too.

## Configuration reference

This is the default configuration, as printed by `waka config get`:

```toml
[core]
# Profile used when --profile is not given (changed by `waka auth switch`)
default_profile = "default"
# Check GitHub once a day for a newer waka release
update_check = true
telemetry = false

[output]
color = "auto"
# Default output format: "table", "plain", "json", "csv" or "tsv"
format = "table"
date_format = "%Y-%m-%d"
time_format_24h = true

[cache]
# Cache API responses locally
enabled = true
# Time-to-live for cached responses, in seconds
ttl_seconds = 300

[display]
show_progress_bar = true
show_sparklines = true
week_start = "monday"

[profiles.default]
# WakaTime-compatible API base URL (change it for self-hosted instances such as Wakapi)
api_url = "https://wakatime.com/api/v1"
```

| Key                       | Default                       | Effect                                                     |
| ------------------------- | ----------------------------- | ---------------------------------------------------------- |
| `core.default_profile`    | `default`                     | Profile used when `--profile` is not passed                |
| `core.update_check`       | `true`                        | Daily check for a newer release (also `WAKA_NO_UPDATE_CHECK`) |
| `output.format`           | `table`                       | Default output format; `--format` overrides it             |
| `cache.enabled`           | `true`                        | Enable the local response cache                            |
| `cache.ttl_seconds`       | `300`                         | Cache time-to-live in seconds                              |
| `profiles.<name>.api_url` | `https://wakatime.com/api/v1` | API base URL for that profile                              |

The remaining keys (`core.telemetry`, `output.color`, `output.date_format`,
`output.time_format_24h`, `cache.path` and the `display.*` section) are accepted
and validated but not used yet. Color output is controlled with `--no-color` and
`NO_COLOR`.

## Profiles

Each profile has its own API key, cache and `api_url`:

```sh
waka config set profiles.work.api_url https://waka.example.com/api/v1
waka auth login --profile work
waka auth switch work        # make it the default
waka -p default stats today  # use another profile for one command
```

## Environment variables

| Variable               | Effect                                                         |
| ---------------------- | -------------------------------------------------------------- |
| `WAKATIME_API_KEY`     | API key, used before the keychain (takes precedence)           |
| `WAKA_API_KEY`         | API key, used when `WAKATIME_API_KEY` is not set               |
| `WAKA_NO_UPDATE_CHECK` | Disable the daily update check                                 |
| `NO_COLOR`             | Standard: disables all colors                                  |
| `RUST_LOG`             | Fine-grained debug logging (`--verbose` enables a sane default) |
| `VISUAL` / `EDITOR`    | Editor used by `waka config edit`                              |

## Config file location

| OS      | Default path                                                         |
| ------- | -------------------------------------------------------------------- |
| Linux   | `$XDG_CONFIG_HOME/waka/config.toml` (usually `~/.config/waka/config.toml`) |
| macOS   | `~/Library/Application Support/waka/config.toml`                     |
| Windows | `%APPDATA%\waka\config.toml`                                         |
