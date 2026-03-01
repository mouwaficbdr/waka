# waka — WakaTime CLI

**waka** is an open source, blazing-fast [WakaTime](https://wakatime.com) CLI written in Rust.
It lets you query your coding stats, browse projects, track goals, and generate reports — entirely from your terminal.

```
waka today
```

```
╭─────────────────────────────────────────────────╮
│  Today — Saturday, 1 March 2025                 │
├──────────────────────┬──────────────────────────┤
│  Total               │  5 hrs 42 mins           │
├──────────────────────┼──────────────────────────┤
│  Rust                │  4 hrs 11 mins  (73 %)   │
│  TOML                │  0 hrs 44 mins  (13 %)   │
│  Markdown            │  0 hrs 47 mins  (14 %)   │
╰──────────────────────┴──────────────────────────╯
```

## Features

- **Fast** — native Rust binary, < 200 ms cold start
- **Secure** — API key stored in system keychain (macOS Keychain, GNOME Keyring, Windows Credential Manager)
- **Flexible output** — tables, plain text, JSON — auto-detected pipe mode
- **Reports** — export to Markdown, HTML, JSON, or CSV
- **Interactive TUI** — full-screen Ratatui dashboard (`waka dashboard`)
- **Shell completions** — Bash, Zsh, Fish, PowerShell
- **Man pages** — installed alongside the binary

## Quick start

1. [Install waka](./installation.md)
2. [Authenticate](./authentication.md) with `waka auth login`
3. Run `waka today` to view your coding activity

## License

MIT — see [LICENSE](https://github.com/mouwaficbdr/waka/blob/main/LICENSE).
