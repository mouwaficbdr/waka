# waka auth

Manage authentication with the WakaTime API.

## Subcommands

| Subcommand         | Description                                 |
| ------------------ | ------------------------------------------- |
| `waka auth login`  | Store your API key in the system keychain   |
| `waka auth logout` | Remove the API key from the system keychain |
| `waka auth status` | Show the current authentication status      |

## waka auth login

```sh
waka auth login [--api-key <key>]
```

If `--api-key` is not supplied, an interactive prompt appears.

## waka auth logout

```sh
waka auth logout
```

Removes the stored API key. You will need to run `waka auth login` again before making API calls.

## waka auth status

```sh
waka auth status
```

Example output:

```
Authenticated as: alice (alice@example.com)
API key: waka_****...****  (stored in system keychain)
```

If not authenticated:

```
Not authenticated. Run `waka auth login` to get started.
```

## Key storage priority

`waka` looks for an API key in this order:

1. `--api-key` CLI flag
2. `WAKA_API_KEY` environment variable
3. System keychain (preferred)
4. Plain-text fallback in `~/.config/waka/config.toml` (last resort, 0600 permissions)
