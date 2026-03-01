# Stable Interfaces — `waka`

> This document describes the interfaces considered **stable** as of v0.4.0.
> Breaking changes to anything listed here require a MAJOR version bump.

---

## 1. CLI — Command Tree

All of the following commands and flags are stable. New subcommands and flags may be added
without a major version bump; existing ones will not be removed or renamed without deprecation.

### Top-level subcommands

```
waka auth          login | logout | status | show-key | switch
waka stats         today | yesterday | week | month | year | range
waka projects      list | top | show
waka languages     list | top
waka editors       list | top
waka goals         list | show | watch
waka leaderboard   show
waka report        generate | summary
waka dashboard
waka prompt
waka completions   <SHELL>
waka config        get | set | edit | path | reset | doctor
waka cache         clear | info | path
waka update
waka changelog
```

### Global flags (every subcommand)

| Flag                    | Since  | Description                                     |
| ----------------------- | ------ | ----------------------------------------------- |
| `-p, --profile <NAME>`  | v0.1.0 | Switch to a named profile                       |
| `-f, --format <FORMAT>` | v0.1.0 | `table` \| `json` \| `csv` \| `plain`           |
| `--no-cache`            | v0.1.0 | Bypass the local cache                          |
| `--no-color`            | v0.1.0 | Disable color output                            |
| `--quiet`               | v0.1.0 | Suppress non-essential output                   |
| `--verbose`             | v0.1.0 | Enable verbose logging                          |
| `--csv-bom`             | v0.1.0 | Prepend UTF-8 BOM to CSV output (Windows Excel) |

---

## 2. Environment Variables

| Variable               | Since  | Description                         |
| ---------------------- | ------ | ----------------------------------- |
| `WAKATIME_API_KEY`     | v0.1.0 | WakaTime API key (highest priority) |
| `WAKA_API_KEY`         | v0.1.0 | Alias for `WAKATIME_API_KEY`        |
| `WAKA_PROFILE`         | v0.1.0 | Default profile to use              |
| `WAKA_FORMAT`          | v0.1.0 | Default output format               |
| `WAKA_NO_CACHE`        | v0.1.0 | Disable cache if set to `1`         |
| `WAKA_NO_UPDATE_CHECK` | v0.1.0 | Disable update check if set to `1`  |
| `WAKA_CONFIG_DIR`      | v0.1.0 | Override the config directory path  |
| `WAKA_CACHE_DIR`       | v0.1.0 | Override the cache directory path   |
| `NO_COLOR`             | v0.1.0 | Standard — disables color output    |

---

## 3. Exit Codes

These codes are stable. Scripts and tooling may rely on them.

| Code | Name         | Meaning                                 |
| ---- | ------------ | --------------------------------------- |
| `0`  | Success      | Command completed successfully          |
| `1`  | GenericError | Unexpected runtime error                |
| `2`  | UsageError   | Bad arguments or missing required input |
| `3`  | AuthError    | Authentication failure (invalid key)    |
| `4`  | NetworkError | Network request failed                  |
| `5`  | ConfigError  | Configuration file error                |
| `6`  | NotFound     | Requested data not found                |

---

## 4. Config File Schema (`~/.config/waka/config.toml`)

The config file schema is **stable from v0.1.0**. Unknown fields in the config file are
**silently ignored** so that older versions of `waka` can use configs written for newer versions.

```toml
[core]
api_key = "..."          # optional — prefer keychain
api_url  = "..."         # default: https://wakatime.com/api/v1
timeout  = 10            # seconds

[output]
format = "table"         # table | json | csv | plain | tsv
color  = "auto"          # auto | always | never

[cache]
enabled = true
ttl     = 300            # seconds

[display]
week_start = "monday"    # monday | sunday
```

### Multiple profiles

Named profiles are stored as TOML tables in the same file:

```toml
[profiles.work]
api_key = "..."

[profiles.personal]
api_key = "..."
```

---

## 5. `waka-api` Public Types (Rust)

The `waka-api` crate is published independently on crates.io.
All public types in `waka_api` are marked `#[non_exhaustive]` to allow WakaTime to add
new API response fields without breaking downstream code.

### `WakaClient`

```rust
pub struct WakaClient { ... }

impl WakaClient {
    pub fn new(api_key: impl Into<String>) -> Self;
    pub fn with_base_url(self, url: impl Into<String>) -> Self;
    pub fn with_timeout(self, timeout: Duration) -> Self;

    pub async fn me(&self) -> Result<User, ApiError>;
    pub async fn summaries(&self, params: SummaryParams) -> Result<SummaryResponse, ApiError>;
    pub async fn stats(&self, range: StatsRange) -> Result<StatsResponse, ApiError>;
    pub async fn projects(&self) -> Result<Vec<Project>, ApiError>;
    pub async fn goals(&self) -> Result<GoalsResponse, ApiError>;
    pub async fn leaderboard(&self, page: Option<u32>) -> Result<LeaderboardResponse, ApiError>;
}
```

### `SummaryParams`

```rust
pub struct SummaryParams { ... }

impl SummaryParams {
    pub fn today() -> Self;
    pub fn yesterday() -> Self;
    pub fn range(start: NaiveDate, end: NaiveDate) -> Self;
    pub fn with_project(self, project: impl Into<String>) -> Self;
    pub fn with_language(self, language: impl Into<String>) -> Self;
}
```

### `StatsRange` (non-exhaustive enum)

```rust
#[non_exhaustive]
pub enum StatsRange {
    Last7Days,
    Last30Days,
    Last6Months,
    LastYear,
    AllTime,
}
```

### `ApiError` (non-exhaustive enum)

```rust
#[non_exhaustive]
pub enum ApiError {
    Unauthorized,
    Forbidden,
    NotFound,
    RateLimit { retry_after: Option<u64> },
    Server { status: u16, message: Option<String> },
    Network(reqwest::Error),
    Decode(serde_json::Error),
}
```

---

## 6. JSON Output Schema

When `--format json` is used, `waka` outputs a single JSON object on stdout.
These schemas are stable; new fields may be added but existing fields will not be removed or renamed.

### `waka stats today --format json`

```json
{
    "range": { "start": "2024-03-01", "end": "2024-03-01", "text": "Today" },
    "total_seconds": 12345,
    "human_readable_total": "3 hrs 25 mins",
    "daily_average": 12345,
    "projects": [
        { "name": "my-project", "total_seconds": 9000, "percent": 72.9 }
    ],
    "languages": [{ "name": "Rust", "total_seconds": 8000, "percent": 64.9 }],
    "editors": [{ "name": "VS Code", "total_seconds": 7000, "percent": 56.8 }]
}
```

---

## 7. Files Written by `waka`

| Path                          | Purpose                               |
| ----------------------------- | ------------------------------------- |
| `~/.config/waka/config.toml`  | User configuration                    |
| `~/.config/waka/credentials`  | API key fallback (keychain preferred) |
| `~/.cache/waka/<profile>/db/` | sled cache database                   |

`waka` **never modifies** `~/.wakatime.cfg` (read-only).

---

_Last updated: v0.4.0 — any breaking change to the above requires a MAJOR version bump._
