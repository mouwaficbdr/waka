# waka cache

Manage the local response cache used to speed up commands and enable offline/prompt use.

## Subcommands

| Subcommand         | Description                                    |
| ------------------ | ---------------------------------------------- |
| `waka cache clear` | Delete all cached responses                    |
| `waka cache info`  | Show cache statistics (size, entry count, TTL) |
| `waka cache path`  | Print the path to the cache directory          |

## Usage

```sh
waka cache <SUBCOMMAND>
```

## Examples

```sh
# Show cache location and stats
waka cache info
waka cache path

# Clear the cache (e.g. after an API key change)
waka cache clear
```

## Notes

- The default cache TTL is 5 minutes for most endpoints.
- Use `--no-cache` on any command to bypass the cache for a single request without clearing it.
- The cache is stored using an embedded key-value database (`sled`) in the platform cache directory:
    - Linux: `~/.cache/waka/`
    - macOS: `~/Library/Caches/waka/`
    - Windows: `%LOCALAPPDATA%\waka\cache\`
