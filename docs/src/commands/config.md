# waka config

View and modify `waka`'s configuration. See [Configuration](../configuration.md)
for every key and its effect.

## Subcommands

| Subcommand                      | Description                                        |
| ------------------------------- | -------------------------------------------------- |
| `waka config get [KEY]`         | Print one value, or the whole config without a key |
| `waka config set <KEY> <VALUE>` | Set a value (validated before saving)              |
| `waka config edit`              | Open the config file in `$VISUAL` / `$EDITOR`      |
| `waka config path`              | Print the path to the config file                  |
| `waka config reset [--confirm]` | Reset all settings to defaults                     |
| `waka config doctor`            | Run a full diagnostic check                        |

## Examples

```sh
waka config get
waka config get cache.ttl_seconds
waka config set cache.ttl_seconds 600
waka config set output.format json
waka config set profiles.work.api_url https://waka.example.com/api/v1
waka config edit
waka config path
waka config reset --confirm
```
