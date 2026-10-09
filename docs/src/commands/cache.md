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

- The default cache TTL is 5 minutes (`cache.ttl_seconds = 300` in the config file).
- Use `--no-cache` on any command to bypass the cache for a single request without clearing it.
- `waka cache clear --older 24h` removes only entries older than the given duration.
- Each entry is a small JSON file, written atomically (temporary file + rename), so several `waka`
  processes (a shell prompt, a tmux status bar, a dashboard) can use the cache at the same time.
- The cache lives in the platform cache directory, one sub-directory per profile:
    - Linux: `~/.cache/waka/<profile>/`
    - macOS: `~/Library/Caches/waka/<profile>/`
    - Windows: `%LOCALAPPDATA%\waka\cache\<profile>\`
