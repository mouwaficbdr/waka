# Man Pages

`waka` ships ROFF-format man pages for every subcommand.

## Installed man pages

| Page                  | Description               |
| --------------------- | ------------------------- |
| `waka(1)`             | Main binary overview      |
| `waka-today(1)`       | Today's coding summary    |
| `waka-week(1)`        | Weekly coding summary     |
| `waka-stats(1)`       | Aggregated stats          |
| `waka-projects(1)`    | Project list              |
| `waka-goals(1)`       | Goals list                |
| `waka-leaderboard(1)` | Public leaderboard        |
| `waka-report(1)`      | Report generation         |
| `waka-dashboard(1)`   | TUI dashboard             |
| `waka-auth(1)`        | Authentication management |
| `waka-config(1)`      | Configuration management  |
| `waka-update(1)`      | Self-update               |
| `waka-changelog(1)`   | Release changelog         |

## Viewing man pages

```sh
man waka
man waka-today
man waka-report
```

## Installation location

### Homebrew

Man pages are installed automatically to the Homebrew prefix:

```
$(brew --prefix)/share/man/man1/
```

### Cargo

Copy the man pages from the release archive to a directory on your `MANPATH`:

```sh
cp man/*.1 ~/.local/share/man/man1/
mandb   # update the man page database (Linux)
```

### Generating locally

Man pages are generated at build time via `clap_mangen`:

```sh
cargo build -p waka       # generated to man/
man -l man/waka.1
```

The source man pages are committed to the repository in `man/`.
