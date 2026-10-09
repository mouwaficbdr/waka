# Authentication

`waka` uses your [WakaTime API key](https://wakatime.com/api-key) to authenticate.

## Interactive login

The simplest way is the interactive login flow:

```sh
waka auth login
```

You will be prompted for your API key. It is validated against the profile's API, then stored in
your **system keychain**: macOS Keychain, Windows Credential Manager, or the kernel keyring
(keyutils) on Linux. When the keychain is unavailable, the key is saved to a credentials file
readable only by you (`0600`).

> **Linux note:** the kernel keyring is kept in memory and cleared on reboot, so you may need to
> run `waka auth login` again after restarting. Setting `WAKATIME_API_KEY` avoids this.

## Non-interactive / CI

Pass the key via the `WAKATIME_API_KEY` (or `WAKA_API_KEY`) environment variable:

```sh
export WAKATIME_API_KEY=waka_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
waka stats today
```

Or store it without the interactive prompt:

```sh
waka auth login --api-key waka_xxxx
```

Avoid `--api-key` on shared machines: it ends up in your shell history.

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

## Profiles

Each profile has its own keychain entry (the `0600` fallback file is currently shared by all
profiles). `waka auth switch work` makes `work` the default profile for every command;
`--profile` overrides it for a single command.

## Security notes

- The API key is **never** written to `config.toml`. Without a keychain it goes to a separate
  `credentials` file created with `0600` permissions.
- It is **never** logged, echoed, or included in error messages.
- All HTTP requests use TLS (rustls — no OpenSSL dependency).
