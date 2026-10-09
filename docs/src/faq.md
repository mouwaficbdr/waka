# FAQ

## Why Rust?

Rust gives `waka` a native binary with < 200 ms cold start, no runtime to install, and excellent cross-platform support. It also makes it easy to ship static musl binaries on Linux.

## Does waka send data to any server besides WakaTime?

`waka` sends no analytics or telemetry. Besides the WakaTime API (or the `api_url` configured for your profile), it contacts GitHub:

- once a day, to check whether a newer release exists (`api.github.com`). Disable it with `waka config set core.update_check false` or `WAKA_NO_UPDATE_CHECK=1`;
- when you run `waka update` or `waka changelog`.

## Where is my API key stored?

In your **system keychain** by default:

- macOS: macOS Keychain
- Linux: the kernel keyring (keyutils), which is in memory and cleared on reboot (set `WAKATIME_API_KEY` to avoid logging in again)
- If no keychain is available: a separate `credentials` file with `0600` permissions
- Windows: Windows Credential Manager

The key is **never** logged or echoed.

## How do I use waka in a Docker container or CI?

Set the `WAKA_API_KEY` environment variable:

```sh
docker run --rm -e WAKA_API_KEY=waka_xxx my-image waka stats today
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
