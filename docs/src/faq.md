# FAQ

## Why Rust?

Rust gives `waka` a native binary with < 200 ms cold start, no runtime to install, and excellent cross-platform support. It also makes it easy to ship static musl binaries on Linux.

## Does waka send data to any server besides WakaTime?

No. `waka` only makes HTTPS requests to `https://wakatime.com/api/v1/` (or your configured `api.base_url`). It never phones home for analytics, telemetry, or update checks beyond what you explicitly request with `waka update`.

## Where is my API key stored?

In your **system keychain** by default:

- macOS: macOS Keychain
- Linux: GNOME Keyring / libsecret (falls back to plain-text `~/.config/waka/config.toml` with `0600` permissions if no keychain is available)
- Windows: Windows Credential Manager

The key is **never** logged or echoed.

## How do I use waka in a Docker container or CI?

Set the `WAKA_API_KEY` environment variable:

```sh
docker run --rm -e WAKA_API_KEY=waka_xxx my-image waka today
```

## How is caching handled?

`waka` caches API responses locally using `sled` (an embedded key-value store). The default TTL is 5 minutes. Disable with `waka config set cache.enabled false` or clear with `rm -rf ~/.cache/waka/`.

## waka shows a spinner but my terminal looks garbled

The spinner requires ANSI escape code support. Set `--color never` or `WAKA_COLOR=never` if your terminal does not support it.

## The binary is > 10 MB. Why?

On first inspection this might seem large, but the binary includes TLS (via rustls), an async runtime (tokio), a full TUI widget framework (ratatui), and all dependencies statically linked. There is no OpenSSL dependency. The release build with LTO is currently ~8 MB.

## Can I use waka offline?

`waka` needs an internet connection to fetch data. If a cached response exists (TTL not expired), some commands will work offline.

## Where can I report bugs or request features?

Open an issue on [GitHub](https://github.com/mouwaficbdr/waka/issues).
Please read [CONTRIBUTING.md](./contributing.md) first.
