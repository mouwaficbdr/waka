# waka config

View and modify `waka`'s configuration.

## Subcommands

| Subcommand                      | Description                           |
| ------------------------------- | ------------------------------------- |
| `waka config get [KEY]`         | Print one or all configuration values |
| `waka config set <KEY> <VALUE>` | Set a configuration value             |
| `waka config edit`              | Open the config file in `$EDITOR`     |
| `waka config path`              | Print the path to the config file     |
| `waka config reset`             | Reset all settings to defaults        |

## Examples

```sh
waka config get
waka config get ui.color
waka config set ui.color always
waka config set ui.table_style minimal
waka config edit
waka config path
```

## All configuration keys

See the full [Configuration reference](../configuration.md) for acceptable values.

| Key              | Default    | Description                   |
| ---------------- | ---------- | ----------------------------- |
| `ui.color`       | `auto`     | Color mode                    |
| `ui.table_style` | `rounded`  | Table display style           |
| `ui.format`      | `text`     | Default output format         |
| `api.base_url`   | (wakatime) | API base URL                  |
| `api.timeout`    | `10`       | Request timeout in seconds    |
| `cache.enabled`  | `true`     | Enable local response caching |
| `cache.ttl`      | `300`      | Cache TTL in seconds          |
