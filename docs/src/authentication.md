# Authentication

`waka` uses your [WakaTime API key](https://wakatime.com/api-key) to authenticate.

## Interactive login

The simplest way is the interactive login flow:

```sh
waka auth login
```

You will be prompted for your API key, which is stored securely in your **system keychain**
(macOS Keychain, GNOME Keyring / libsecret on Linux, Windows Credential Manager).

## Non-interactive / CI

Pass the key via the `WAKA_API_KEY` environment variable:

```sh
export WAKA_API_KEY=waka_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
waka today
```

Or use the `--api-key` flag for a one-off command:

```sh
waka today --api-key waka_xxxx
```

## Verify authentication

```sh
waka auth status
```

Expected output:

```
Authenticated as: alice (alice@example.com)
API key: waka_****...****  (stored in system keychain)
```

## Logout

```sh
waka auth logout
```

This removes the API key from the system keychain. Your WakaTime data is unaffected.

## Security notes

- The API key is **never** written to `~/.config/waka/config.toml` unless the keychain is unavailable.
- It is **never** logged, echoed, or included in error messages.
- All HTTP requests use TLS (rustls — no OpenSSL dependency).
