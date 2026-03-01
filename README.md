# waka

[![CI](https://github.com/mouwaficbdr/waka/actions/workflows/ci.yml/badge.svg)](https://github.com/mouwaficbdr/waka/actions/workflows/ci.yml)
[![Security Audit](https://github.com/mouwaficbdr/waka/actions/workflows/audit.yml/badge.svg)](https://github.com/mouwaficbdr/waka/actions/workflows/audit.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/waka-cli.svg)](https://crates.io/crates/waka-cli)

**The WakaTime CLI you always deserved.** Fast, beautiful, composable.

> ⚠️ This project is under active development. See [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md) for the current status.

---

## Installation

```bash
# Cargo
cargo install waka-cli

# macOS / Linux (Homebrew)
brew tap mouwaficbdr/waka
brew install waka

# Universal installer
curl -sSfL https://github.com/mouwaficbdr/waka/releases/latest/download/waka-installer.sh | sh
```

## Quick Start

```bash
# Set up your API key (from https://wakatime.com/settings/api-key)
waka auth login

# Today's coding stats
waka stats today

# This week at a glance
waka stats week

# Interactive TUI dashboard
waka dashboard
```

## Features

_Features listed here reflect the currently implemented state._

- `waka auth` — Secure API key management (OS keychain)
- `waka stats today/week/month` — Coding stats with beautiful tables
- `waka config doctor` — Diagnose configuration issues
- Output formats: `table`, `json`, `csv`, `plain` — composable with Unix pipes

## Roadmap

See [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md) for the full phased roadmap.

Coming next: projects, languages, editors, TUI dashboard, goals, reports.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

## License

MIT — see [LICENSE](LICENSE).
