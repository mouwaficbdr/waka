# DEVELOPMENT_PLAN.md — `waka` Phased Task List

> This is your **working task list**. Mark each task `[x]` as you complete it.
> Do not start a phase until the previous one is fully complete and green.
> Each task maps to one or more commits. Each commit must be green.

---

## Phase 0 — Foundations (v0.1.0)

_Goal: A working `waka auth login` + `waka stats today` with real API data._

### 0.1 — Repository Bootstrap

- [x] Create `Cargo.toml` workspace with all 6 crate members declared
- [x] Create each crate with `cargo new --lib` / `cargo new --bin`
- [x] Add `rustfmt.toml` with project formatting config
- [x] Add `.clippy.toml` with pedantic config
- [x] Add `.cargo/config.toml` with strip/LTO release profile
- [x] Add `deny.toml` for cargo-deny (licenses + advisories)
- [x] Add `cliff.toml` for git-cliff CHANGELOG generation
- [x] Add `LICENSE` (MIT)
- [x] Add `.gitignore` (standard Rust + OS files)
- [x] Add `.editorconfig`
- [x] Verify: `cargo build` succeeds on empty workspace
- [x] **Commit:** `chore: bootstrap workspace with all crates and tooling config`

### 0.2 — CI Pipeline

- [x] Create `.github/workflows/ci.yml`:
    - Triggers: push to any branch, PR to main
    - Jobs: `fmt`, `clippy`, `test`, `build`
    - Matrix: ubuntu-latest, macos-latest, windows-latest
    - Uses `cargo nextest`
- [x] Create `.github/workflows/audit.yml`:
    - Triggers: push to main, weekly schedule
    - Jobs: `cargo audit`, `cargo deny check`
- [x] Add CI status badge to `README.md`
- [x] Verify: CI passes on empty workspace
- [x] **Commit:** `ci: add GitHub Actions CI and security audit workflows`

### 0.3 — `waka-api` — Types & Client Skeleton

- [x] Define all API response types in `waka-api/src/types.rs`:
    - `User`, `SummaryResponse`, `SummaryData`, `Project`, `Language`, `Editor`
    - `GrandTotal`, `Category`, `Goal`, `Leaderboard`
    - All fields from the real WakaTime API (use fixtures as reference)
    - All types: `#[derive(Debug, Clone, Serialize, Deserialize)]`
- [x] Add test fixtures in `tests/fixtures/`:
    - `user_current.json`
    - `summaries_today.json`
    - `summaries_week.json`
    - `stats_last_7_days.json`
    - `errors/401_unauthorized.json`
    - `errors/429_rate_limit.json`
- [x] Define `ApiError` enum with `thiserror`:
    - `Unauthorized`, `RateLimit { retry_after: Option<u64> }`, `NotFound`, `ServerError { status: u16 }`, `NetworkError`, `ParseError`
- [x] Write unit tests for all error variants
- [x] **Commit:** `feat(api): define response types and error enum`

### 0.4 — `waka-api` — HTTP Client

- [x] Implement `WakaClient` struct with `api_key: String`, `base_url: Url`, `http: reqwest::Client`
- [x] Implement `WakaClient::new(api_key: &str) -> Self`
- [x] Implement `WakaClient::with_base_url(api_key: &str, base_url: &str) -> Result<Self>`
- [x] Implement private `get<T>(&self, path: &str, query: &[(&str, &str)]) -> Result<T, ApiError>`
    - Sets `Authorization: Basic <base64>` header
    - Handles 401 → `ApiError::Unauthorized`
    - Handles 429 with `Retry-After` header → `ApiError::RateLimit`
    - Handles 5xx → `ApiError::ServerError`
    - Implements retry with exponential backoff (max 3 attempts)
    - Timeout: 10 seconds
- [x] Write integration tests using `wiremock`:
    - Successful request returns deserialized type
    - 401 returns `ApiError::Unauthorized`
    - 429 returns `ApiError::RateLimit` with correct `retry_after`
    - Network timeout returns `ApiError::NetworkError`
- [x] **Commit:** `feat(api): implement HTTP client with auth, retry, and error handling`

### 0.5 — `waka-api` — Endpoints (Phase 0 subset)

- [x] Implement `WakaClient::me() -> Result<User, ApiError>`
- [x] Implement `SummaryParams` builder:
    - Required: `start: NaiveDate`, `end: NaiveDate`
    - Optional: `project: Option<String>`, `branches: Option<String>`
    - Constructor: `SummaryParams::today()`, `SummaryParams::for_range(start, end)`
- [x] Implement `WakaClient::summaries(params: SummaryParams) -> Result<SummaryResponse, ApiError>`
- [x] Write unit tests for `SummaryParams` date formatting
- [x] Write integration tests for `me()` and `summaries()` with mock server
- [x] **Commit:** `feat(api): implement me() and summaries() endpoints`

### 0.6 — `waka-config` — Core

- [x] Define `Config` struct mirroring `config.toml` schema from SPEC.md
- [x] Implement config file path resolution using `directories::ProjectDirs`
    - Linux: `$XDG_CONFIG_HOME/waka/config.toml` or `~/.config/waka/config.toml`
    - macOS: `~/Library/Application Support/waka/config.toml`
    - Windows: `%APPDATA%\waka\config.toml`
- [x] Implement `Config::load() -> Result<Config>` — reads from file, falls back to defaults
- [x] Implement `Config::save(&self) -> Result<()>`
- [x] Implement `Config::default()` with all spec-defined defaults
- [x] Unknown fields in TOML are silently ignored (serde `#[serde(deny_unknown_fields)]` is NOT used)
- [x] Write unit tests for load/save/defaults
- [x] **Commit:** `feat(config): implement config file load/save with XDG paths`

### 0.7 — `waka-config` — Credentials

- [x] Implement `CredentialStore` with the priority chain from SPEC.md §5.2:
    1. `--api-key` flag (passed in at runtime)
    2. `WAKATIME_API_KEY` env var
    3. `WAKA_API_KEY` env var
    4. System keychain via `keyring` crate
    5. `~/.config/waka/credentials` file (base64, permissions 0600)
    6. `~/.wakatime.cfg` (read-only, for compatibility)
- [x] Implement `CredentialStore::get_api_key() -> Result<String>`
- [x] Implement `CredentialStore::set_api_key(key: &str) -> Result<()>` (stores in keychain)
- [x] Implement `CredentialStore::delete_api_key() -> Result<()>`
- [x] Ensure API key is NEVER logged (use a `Sensitive(String)` newtype with `Debug` impl that redacts)
- [x] Write unit tests (mock the keyring, test env var priority)
- [x] **Commit:** `feat(config): implement credential store with keychain and env var priority chain`

### 0.8 — `waka` binary — CLI skeleton

- [x] Set up `clap` with derive macros in `waka/src/main.rs`
- [x] Define the full command tree from SPEC.md §6.1 (stubs only — no implementation yet)
- [x] Define global options: `--profile`, `--format`, `--no-cache`, `--no-color`, `--quiet`, `--verbose`
- [x] Implement `--version` (use `env!("CARGO_PKG_VERSION")`)
- [x] Implement exit code handling — all handlers return `Result<(), AppError>`; `main()` maps to exit codes
- [x] Verify: `waka --help` shows the full command tree correctly
- [x] Verify: `waka stats --help`, `waka auth --help`, etc. all work
- [x] **Commit:** `feat(cli): scaffold full command tree with clap derive`

### 0.9 — `waka auth` — Implementation

- [x] Implement `waka auth login` (interactive):
    - Check if already logged in, warn user
    - Prompt for API key (hidden input via `rpassword` or `console`)
    - Show spinner while testing connection
    - Call `WakaClient::me()` to validate
    - On success: save to keychain, print user info
    - On failure: clear error message with suggestions
- [x] Implement `waka auth login --api-key <KEY>` (non-interactive)
- [x] Implement `waka auth logout`:
    - Delete from keychain
    - Confirm deletion
- [x] Implement `waka auth status`:
    - Show if logged in (yes/no)
    - Show username if logged in
    - Never show the actual key
- [x] Write integration tests for all auth flows
- [x] **Commit:** `feat(auth): implement auth login, logout, and status commands`

### 0.10 — `waka-render` — Core Renderers

- [x] Define `RenderOptions` struct: `{ color: bool, width: u16, format: OutputFormat }`
- [x] Define `OutputFormat` enum: `Table, Json, Csv, Plain, Tsv`
- [x] Implement `detect_output_format(config: &Config) -> OutputFormat` — TTY detection
- [x] Implement `format_duration(seconds: u64) -> String` — "6h 42m" format
- [x] Implement `format_bar(ratio: f64, width: u8) -> String` — ASCII progress bar
- [x] Implement `SummaryRenderer::render_table(data: &SummaryResponse, opts: &RenderOptions) -> String`
- [x] Implement `SummaryRenderer::render_json(data: &SummaryResponse) -> String`
- [x] Implement `SummaryRenderer::render_plain(data: &SummaryResponse) -> String`
- [x] Add snapshot tests for each renderer with `summaries_today.json` fixture
- [x] **Commit:** `feat(render): implement summary renderers (table, json, plain) with snapshot tests`

### 0.11 — `waka stats today/week/month`

- [x] Implement `waka stats today` command handler:
    - Load config + credentials
    - Show spinner
    - Call `summaries(SummaryParams::today())`
    - Pass to renderer
    - Print to stdout
- [x] Implement `waka stats week` (last 7 days + sparkline)
- [x] Implement `waka stats month` (last 30 days)
- [x] Implement `--project` filter on all stats commands
- [x] Implement `--format` override on all stats commands
- [x] Manual end-to-end test: `waka stats today` should work with a real API key
- [x] **Commit:** `feat(stats): implement today/week/month commands`

### 0.12 — `waka config doctor`

- [x] Implement `waka config doctor`:
    - Check config file exists and is valid TOML
    - Check API key present
    - Check API key valid (call `me()`)
    - Check network reachability
    - Check cache directory writable
    - Check for available updates (compare git tag with GitHub Releases API)
    - Format output as per SPEC.md §6.3
- [x] **Commit:** `feat(config): implement doctor diagnostic command`

### 0.13 — Phase 0 Polish

- [x] Write `README.md` (Phase 0 features only — no promised features)
- [x] Write `CONTRIBUTING.md`
- [x] Write `CODE_OF_CONDUCT.md` (Contributor Covenant v2.1)
- [x] Write `SECURITY.md`
- [x] Verify all Phase 0 completion criteria from `CLAUDE.md §13`
- [x] `cargo build --release` produces a working binary for host platform
- [x] Tag: `v0.1.0`
- [x] **Commit:** `docs: add README, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY`

---

## Phase 1 — Completeness (v0.2.0)

_Goal: All stats commands, projects, languages, editors, all output formats, multi-profile, shell completions._

### 1.1 — `waka-cache` — Core

- [x] Implement `CacheStore` wrapping `sled::Db`
- [x] Implement `CacheStore::open(profile: &str) -> Result<Self>` — path from `ProjectDirs`
- [x] Implement `CacheStore::get<T: DeserializeOwned>(key: &str) -> Result<Option<CacheEntry<T>>>`
- [x] Implement `CacheStore::set<T: Serialize>(key: &str, value: &T, ttl: Duration) -> Result<()>`
- [x] Implement `CacheEntry<T>` with `value: T`, `inserted_at: DateTime<Utc>`, `ttl: Duration`
- [x] Implement `CacheEntry::is_expired() -> bool`
- [x] Implement `CacheEntry::age_human() -> String` — "3m ago", "2h ago"
- [x] Implement `CacheStore::clear() -> Result<usize>` — returns count of cleared entries
- [x] Implement `CacheStore::clear_older_than(duration: Duration) -> Result<usize>`
- [x] Implement `CacheStore::info() -> CacheInfo` — size on disk, entry count, last write
- [x] Handle corrupted sled DB gracefully: log warning, return empty cache (never panic)
- [x] Write unit tests for all operations
- [x] **Commit:** `feat(cache): implement local cache with sled, TTL, and graceful corruption handling`

### 1.2 — Cache Integration into Stats Commands

- [x] Integrate `CacheStore` into all stats command handlers
- [x] Cache key format: `summaries:{date}`, `summaries:{date}:{project}`, `stats:{range}`
- [x] Cache hit (valid TTL): return immediately, add `(cached Xm ago)` indicator in table footer
- [x] Cache hit (expired): try refresh; on success show fresh; on failure show stale (sync, not background)
- [x] Cache miss: fetch, display, store
- [x] `--no-cache` flag: bypass read + write
- [x] Network error with cache: show stale data with `⚠ offline` badge
- [x] Network error without cache: clear error message
- [x] **Commit:** `feat(cache): integrate CacheStore into stats command with TTL, stale-if-error`

### 1.3 — `waka stats yesterday` and `range`

- [x] Implement `waka stats yesterday`
- [x] Implement `waka stats year`
- [x] Implement `waka stats range --from <DATE> --to <DATE>`
- [x] Date parsing: `YYYY-MM-DD` accepted (flexible parsing not in SPEC.md — see SPEC_GAPS.md §3)
- [x] **Commit:** (included in earlier stats implementation commits)

### 1.4 — `waka-api` — Additional Endpoints

- [x] Implement `WakaClient::projects() -> Result<ProjectsResponse, ApiError>`
- [x] Implement `WakaClient::stats(range: StatsRange) -> Result<StatsResponse, ApiError>`
    - `StatsRange` enum: `Last7Days`, `Last30Days`, `Last6Months`, `LastYear`, `AllTime`
- [x] Implement `WakaClient::goals() -> Result<GoalsResponse, ApiError>`
- [x] Implement `WakaClient::leaderboard(page: u32) -> Result<LeaderboardResponse, ApiError>`
- [x] Add fixtures and tests for all new endpoints
- [x] **Commit:** `feat(api): add projects, stats, goals, and leaderboard endpoints`

### 1.5 — Projects, Languages, Editors Commands

- [x] Implement `waka projects list`
- [x] Implement `waka projects top --period`
- [x] Implement `waka projects show <name>`
- [x] Implement `waka languages list` and `top`
- [x] Implement `waka editors list` and `top`
- [x] Add renderers in `waka-render` for each (table + json + plain)
- [x] Add snapshot tests for all new renderers
- [x] **Commit:** `feat(projects): implement projects, languages, and editors commands with BreakdownRenderer`

### 1.6 — Output Formats: CSV & TSV

- [x] Implement `SummaryRenderer::render_csv(data: &SummaryResponse) -> String`
- [x] Implement CSV for projects, languages, editors
- [x] Implement TSV variants
- [x] Add `--csv-bom` flag for Windows Excel compatibility
- [x] Fix `detect_output_format` to preserve CSV/TSV/JSON when piped
- [x] Test pipe output: `waka stats today --format=csv | head`
- [x] **Commit:** `feat(render): add CSV and TSV output formats with --csv-bom support`

### 1.7 — Multi-Profile Support

- [x] Implement profile switching: `--profile <name>` global flag
- [x] Implement `waka auth switch <profile>`
- [x] Implement profile-scoped credential storage
- [x] Implement profile-scoped cache (separate sled DB per profile)
- [x] **Commit:** `feat(config): implement multi-profile support with waka auth switch`

### 1.8 — TTY Detection & NO_COLOR

- [x] Implement full TTY detection in `waka-render` via `should_use_color()`
- [x] Auto-switch to `plain` format when stdout is not a TTY
- [x] Respect `NO_COLOR` env var (handled by `should_use_color()`)
- [x] Respect `TERM=dumb`
- [x] Test: `waka stats today | cat` produces clean plain text without escape codes
- [x] **Commit:** `feat(render): implement TTY detection, NO_COLOR, and TERM=dumb support`

### 1.9 — Shell Completions

- [x] Generate completions via `waka completions <shell>` using `clap_complete`:
    - Bash → `completions/waka.bash`
    - Zsh → `completions/_waka`
    - Fish → `completions/waka.fish`
    - PowerShell → `completions/_waka.ps1`
    - Elvish → `completions/waka.elv`
- [x] Implement `waka completions <shell>` command (prints to stdout)
- [x] Add minimal `build.rs` with rerun-if-changed directives
- [x] Document installation in `CONTRIBUTING.md`
- [x] **Commit:** `feat(completions): generate shell completions for bash/zsh/fish/powershell/elvish`

### 1.10 — Cache Management Commands

- [x] Implement `waka cache clear`
- [x] Implement `waka cache clear --older <DURATION>`
- [x] Implement `waka cache info`
- [x] Implement `waka cache path`
- [x] **Commit:** `feat(cache): implement cache management commands`

### 1.11 — Phase 1 Polish

- [x] Update `README.md` with Phase 1 features
- [x] Verify all Phase 1 completion criteria
- [x] Tag: `v0.2.0`

---

## Phase 2 — UX Premium (v0.3.0)

_Goal: TUI dashboard, goals with notifications, shell prompt integration, update checker._

### 2.1 — `waka prompt`

- [x] Implement `waka prompt` — reads from cache only, no network call
- [x] Output formats: `simple` ("⏱ 6h 42m"), `detailed` ("⏱ 6h 42m | my-saas")
- [x] Silent on error (empty string — never break a shell prompt)
- [x] Timeout: 100ms max (cache only — if cache is cold, return empty)
- [x] Document Starship and tmux integration
- [x] **Commit:** `feat(prompt): implement shell prompt integration command`

### 2.2 — Update Checker

- [x] Implement background update check (once per day, async, non-blocking)
- [x] Fetch latest version from GitHub Releases API
- [x] Store last-check timestamp in cache
- [x] If new version available: print subtle message below output
- [x] `WAKA_NO_UPDATE_CHECK=1` disables the check
- [x] `update_check = false` in config disables the check
- [x] **Commit:** `feat(cli): implement non-blocking update checker`

### 2.3 — `waka goals`

- [x] Implement `waka goals list` with progress bars
- [x] Implement `waka goals show <id>` with full detail
- [x] Add `GoalRenderer` with table + json + plain
- [x] Add snapshot tests
- [x] **Commit:** `feat(goals): implement goals list and show commands`

### 2.4 — `waka goals watch`

- [x] Implement `waka goals watch` with configurable polling interval
- [x] Show goals table, refresh in-place (clear/reprint — no full TUI)
- [x] When goal is reached: send system notification via `notify-send` (best-effort)
- [x] Graceful `Ctrl+C` handling (restore terminal state)
- [x] **Commit:** `feat(goals): implement goals watch with system notifications`

### 2.5 — `waka leaderboard`

- [x] Implement `waka leaderboard show`
- [x] Highlight current user's rank
- [x] `--page <N>` flag
- [x] **Commit:** `feat(leaderboard): implement leaderboard command`

### 2.6 — `waka-tui` — Architecture

- [x] Set up ratatui + crossterm in `waka-tui`
- [x] Define `App` state struct with all data fields
- [x] Define `Event` enum: `Tick`, `Key`, `ApiUpdate(Box<SummaryResponse>)`, `Error`
- [x] Implement event loop: tokio + crossterm event stream + ticker
- [x] Implement background data fetching task (separate tokio task, sends events to main loop)
- [x] Implement graceful shutdown on `q` / `Esc` / `Ctrl+C` (cursor restored, terminal cleaned up)
- [x] **Commit:** `feat(tui): implement ratatui app skeleton with event loop and state management`

### 2.7 — `waka-tui` — Main View

- [x] Implement layout from SPEC.md §7.2 (Today, This Week, Top Projects, Languages, Goals, Activity)
- [x] Implement sparkline widget for weekly view
- [x] Implement progress bar widget for goals
- [x] Implement activity chart (30-day sparkline)
- [x] Implement status bar: "Last updated: HH:MM:SS · Auto-refresh in Xm Ys · Tab: switch view"
- [x] **Commit:** `feat(tui): implement main dashboard layout with all widgets`

### 2.8 — `waka-tui` — Additional Views

- [x] Implement projects detail view (Tab → 2)
- [x] Implement languages view (Tab → 3)
- [x] Implement goals view (Tab → 4)
- [x] Implement activity calendar view (Tab → 5) — GitHub-style heatmap using block characters
- [x] Implement keyboard navigation in lists (`↑↓`, `Enter`)
- [x] Implement help overlay (`?` key)
- [x] **Commit:** `feat(tui): implement all dashboard views and keyboard navigation`

### 2.9 — `waka-tui` — Polish

- [x] Implement offline indicator badge `⚠ offline`
- [x] Implement loading indicator during background refresh
- [x] Implement `r` key for manual refresh
- [x] Implement `e` key for export current view
- [x] Handle terminal resize gracefully
- [x] Test on small terminal widths (80 columns minimum)
- [x] **Commit:** `feat(tui): add offline indicator, manual refresh, export, and resize handling`

### 2.10 — Error Messages Polish

- [x] Audit all user-facing error messages against SPEC.md §10.2 format
- [x] Every error must have: title, reason, suggested actions, support link
- [x] **Commit:** `fix(cli): standardize all error messages per spec format`

### 2.11 — Phase 2 Polish

- [x] Update `README.md` with TUI screenshot (use `VHS` or `asciinema`)
- [x] Verify all Phase 2 completion criteria
- [x] Tag: `v0.3.0`

---

## Phase 3 — Ecosystem (v0.4.0)

_Goal: Reports, update command, man pages, full distribution, waka-api on crates.io._

### 3.1 — `waka report generate`

- [x] Implement `waka report generate --from <DATE> --to <DATE>`
- [x] Implement Markdown output
- [x] Implement HTML output (CSS inline, responsive)
- [x] Implement JSON output
- [x] Implement CSV output
- [x] Content: summary, projects breakdown, languages, editors, daily activity, goals achieved
- [x] `--output <FILE>` flag for saving to file
- [x] **Commit:** `feat(report): implement report generation in md/html/json/csv`

### 3.2 — `waka update` and `waka changelog`

- [x] Implement `waka update` (detect install method, invoke appropriate updater)
- [x] Implement `waka changelog` (shows CHANGELOG.md from installed version to latest)
- [x] **Commit:** `feat(cli): implement update and changelog commands`

### 3.3 — Man Pages

- [x] Generate man pages using `clap_mangen` in `build.rs`
- [x] Output to `man/`
- [x] Document installation in `CONTRIBUTING.md`
- [x] **Commit:** `feat(docs): generate man pages with clap_mangen`

### 3.4 — cargo-dist Setup

- [x] Configure `dist-workspace.toml` for cargo-dist
- [x] Configure Homebrew tap formula generation
- [x] Create `.github/workflows/release.yml` triggered on tags `v*.*.*`
- [x] Verify release builds for all Tier 1 targets
- [x] **Commit:** `ci: configure cargo-dist for multi-platform releases`

### 3.5 — `waka-api` crates.io Publication

- [x] Finalize `waka-api` public API (ensure stability)
- [x] Write comprehensive `examples/` for `waka-api`
- [x] Verify `cargo doc` renders correctly on docs.rs
- [x] Publish to crates.io
- [x] **Commit:** `chore(api): prepare waka-api for crates.io publication`

### 3.6 — Documentation Site

- [x] Set up mdBook in `docs/`
- [x] Write: Installation, Configuration, All Commands, Integrations, FAQ
- [x] Configure GitHub Pages deployment
- [x] **Commit:** `docs: set up mdBook documentation site`

### 3.7 — Phase 3 Polish

- [x] Verify all Phase 3 completion criteria
- [x] Tag: `v0.4.0`

## Phase 4 — Stability v1.0

_Goal: Stable interfaces, security audit, 80%+ test coverage, Windows validation._

### 4.1 — Interface Stabilization

- [x] Audit all CLI flags and subcommands — no planned breaking changes
- [x] Audit `waka-api` public types — no planned breaking changes
- [x] Audit config file schema — no planned breaking changes
- [x] Document stable interfaces clearly

### 4.2 — Test Coverage

- [x] Run `cargo llvm-cov` and measure coverage
- [x] Identify uncovered paths
- [x] Reach 80%+ coverage on `waka-api`, `waka-cache`, `waka-config`, `waka-render`

### 4.3 — Windows Validation

- [x] Run full test suite on Windows in CI
- [x] Validate keychain (Windows Credential Manager) integration
- [x] Validate path handling on Windows
- [x] Fix any Windows-specific issues

### 4.4 — v1.0.0

- [x] Final review of all documentation
- [x] All completion criteria met for all phases
- [x] Tag: `v1.0.0` 🎉

---

## Notes

- Task IDs follow `<phase>.<task>` format (e.g., `0.3`)
- Each task should ideally produce 1-3 commits
- When a task reveals unexpected complexity, break it into sub-tasks and document here
- When discovering spec gaps, document in `SPEC_GAPS.md`
