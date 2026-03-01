# CONVENTIONS.md — `waka` Coding & Project Conventions

> Single source of truth for all style, naming, and process decisions.
> When in doubt: reference this file before making a judgment call.

---

## 1. Rust Code Style

### 1.1 Formatter

`rustfmt` with the following `rustfmt.toml`:

```toml
edition = "2021"
max_width = 100
use_field_init_shorthand = true
use_try_shorthand = true
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
wrap_comments = true
comment_width = 100
format_code_in_doc_comments = true
```

Run before every commit: `cargo fmt --all`
In CI: `cargo fmt --all -- --check` (fails if diff)

### 1.2 Naming

| Item                 | Convention             | Example                                  |
| -------------------- | ---------------------- | ---------------------------------------- |
| Types, enums, traits | `PascalCase`           | `WakaClient`, `ApiError`, `OutputFormat` |
| Functions, methods   | `snake_case`           | `fetch_summary`, `render_table`          |
| Constants            | `SCREAMING_SNAKE_CASE` | `DEFAULT_TTL_SECONDS`                    |
| Modules              | `snake_case`           | `waka_api`, `summary_renderer`           |
| Files                | `snake_case`           | `client.rs`, `render_options.rs`         |
| Crates               | `kebab-case`           | `waka-api`, `waka-cache`                 |

### 1.3 Module Organization

Within each crate, prefer this layout:

```
src/
├── lib.rs          # pub use re-exports + top-level doc comment
├── client.rs       # Main client struct
├── types.rs        # All data types (or types/ directory for large crates)
├── error.rs        # Error enum
└── endpoints/
    ├── mod.rs
    ├── summaries.rs
    └── projects.rs
```

`lib.rs` re-exports everything the user needs at the crate root:

```rust
pub use client::WakaClient;
pub use error::ApiError;
pub use types::*;
```

### 1.4 Error Handling Patterns

**In library crates (`waka-api`, `waka-cache`, `waka-config`, `waka-render`):**

```rust
// Define with thiserror
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Unauthorized: invalid API key")]
    Unauthorized,
    #[error("Rate limited — retry after {retry_after:?} seconds")]
    RateLimit { retry_after: Option<u64> },
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}

// Propagate with ?
pub async fn me(&self) -> Result<User, ApiError> {
    let response = self.get("/users/current", &[]).await?;
    Ok(response)
}
```

**In binary crate (`waka`):**

```rust
// Use anyhow for command handlers
async fn handle_stats_today(args: StatsTodayArgs, ctx: Context) -> anyhow::Result<()> {
    let summary = ctx.client.summaries(SummaryParams::today()).await
        .context("Failed to fetch today's stats")?;
    // ...
    Ok(())
}

// In main(): map to exit codes
fn main() {
    if let Err(err) = run() {
        eprintln!("{}", format_user_error(&err));
        std::process::exit(exit_code_for(&err));
    }
}
```

### 1.5 Imports

Group imports in this order (enforced by `rustfmt` with `group_imports = "StdExternalCrate"`):

1. `std::` imports
2. External crate imports
3. Local (`crate::` / `super::`) imports

```rust
use std::collections::HashMap;
use std::time::Duration;

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::client::WakaClient;
use crate::types::SummaryResponse;
```

### 1.6 Lifetimes & mouwaficbdrship

- Prefer owned types in structs that are stored (avoid lifetime parameters in public API types)
- Use `&str` in function arguments, `String` in struct fields
- Use `Cow<'_, str>` only when the allocation-or-not decision must be deferred

### 1.7 Clippy Config

`.clippy.toml`:

```toml
msrv = "1.82.0"
```

In `lib.rs` of each crate:

```rust
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]  // Common in Rust — acceptable
#![allow(clippy::must_use_candidate)]       // Too noisy for our use case
```

All other `#[allow(...)]` must have a comment explaining why.

---

## 2. Commit Conventions

### 2.1 Format

```
<type>(<scope>): <description>

[optional body — wrapped at 72 chars]

[optional footer]
```

**Rules:**

- Description: lowercase, imperative mood, no period at the end
- Max 72 chars for the subject line
- Body: explain _why_, not _what_
- `BREAKING CHANGE:` in footer triggers MAJOR version bump

### 2.2 Types Reference

| Type       | When to use                                   |
| ---------- | --------------------------------------------- |
| `feat`     | New user-facing feature                       |
| `fix`      | Bug fix                                       |
| `docs`     | Documentation only (comments, README, mdBook) |
| `test`     | Add or update tests                           |
| `refactor` | Code restructuring without behavior change    |
| `perf`     | Performance improvement                       |
| `style`    | Formatting, whitespace only                   |
| `chore`    | Build scripts, dependencies, tooling config   |
| `ci`       | CI/CD workflow changes                        |
| `revert`   | Reverts a previous commit                     |

### 2.3 Scopes Reference

| Scope         | Maps to                     |
| ------------- | --------------------------- |
| `api`         | `waka-api` crate            |
| `cache`       | `waka-cache` crate          |
| `config`      | `waka-config` crate         |
| `render`      | `waka-render` crate         |
| `tui`         | `waka-tui` crate            |
| `auth`        | `waka auth` command         |
| `stats`       | `waka stats` command        |
| `projects`    | `waka projects` command     |
| `goals`       | `waka goals` command        |
| `report`      | `waka report` command       |
| `prompt`      | `waka prompt` command       |
| `completions` | Shell completions           |
| `cli`         | General CLI / `waka` binary |
| `ci`          | CI/CD workflows             |
| `deps`        | Dependency updates          |
| `docs`        | Documentation               |

### 2.4 Good Commit Examples

```
feat(api): implement summaries endpoint with date range support

Add WakaClient::summaries() taking SummaryParams which supports
today(), yesterday(), and arbitrary date ranges via for_range().

Includes integration tests against mock server with real API fixtures.
```

```
fix(cache): handle corrupted sled database gracefully

Previously, a corrupted sled DB caused a panic at startup. Now we
detect the error, log a warning, and fall back to an empty in-memory
cache so the user can still use the tool with live API data.

Fixes: unexpected panics reported in dev testing
```

```
chore(deps): update reqwest from 0.13.1 to 0.13.2

Patch update, changelog at https://github.com/seanmonstar/reqwest/releases
```

### 2.5 Bad Commit Examples (never do these)

```
✗ WIP
✗ fix stuff
✗ update
✗ fixed the bug
✗ feat: Add new feature to the API client for getting summaries
✗ [skip ci] temp commit
```

---

## 3. Versioning

### 3.1 Semantic Versioning

`MAJOR.MINOR.PATCH` — strict [SemVer 2.0](https://semver.org/)

| What changed                                  | Bump                       |
| --------------------------------------------- | -------------------------- |
| Backwards-incompatible CLI flag/subcommand    | MAJOR                      |
| Backwards-incompatible config file schema     | MAJOR                      |
| Backwards-incompatible `waka-api` public type | MAJOR                      |
| New command, flag, or feature                 | MINOR                      |
| Bug fix                                       | PATCH                      |
| Documentation, CI, tooling                    | no version bump (tag only) |

### 3.2 Pre-1.0 Rules

Before `v1.0.0`: MINOR bumps may contain breaking changes (per SemVer spec). Document them clearly in `CHANGELOG.md` with a `BREAKING CHANGE` note.

### 3.3 Git Tags

```bash
git tag -a v0.2.0 -m "v0.2.0 — Phase 1: Completeness"
git push origin v0.2.0
```

Tags trigger the release CI workflow.

---

## 4. Branch Naming

```
main                          ← always green
phase/0-foundations           ← phase work
phase/1-completeness
feat/<short-description>      ← new feature
fix/<short-description>       ← bug fix
docs/<short-description>      ← docs only
refactor/<short-description>  ← refactor
chore/<short-description>     ← tooling/deps
```

---

## 5. File Naming & Organization

| File                     | Purpose                                                               |
| ------------------------ | --------------------------------------------------------------------- |
| `Cargo.toml` (workspace) | Workspace members, shared dependencies via `[workspace.dependencies]` |
| `Cargo.toml` (crate)     | Crate metadata, inherits from workspace                               |
| `src/lib.rs`             | Crate root, `pub use` re-exports, crate-level doc                     |
| `src/main.rs`            | Binary entry point only — thin, delegates to lib                      |
| `src/error.rs`           | Error types                                                           |
| `src/types.rs`           | Data types (if small), or `src/types/` directory                      |
| `tests/integration/`     | Integration test files, one per feature area                          |
| `tests/fixtures/`        | JSON files — never modify programmatically                            |

---

## 6. Documentation Standards

### 6.1 Crate Root (`lib.rs`)

Every public crate must start with:

````rust
//! # waka-api
//!
//! WakaTime API client for Rust.
//!
//! ## Quick Start
//!
//! ```rust
//! use waka_api::WakaClient;
//!
//! let client = WakaClient::new("your-api-key");
//! let user = client.me().await?;
//! println!("Hello, {}!", user.display_name);
//! ```
````

### 6.2 Doc Comment Style

````rust
/// Short one-line description (imperative mood).
///
/// Optional longer description. Explain *why* and any non-obvious
/// behavior. Do not restate what the code obviously does.
///
/// # Errors
///
/// Returns [`ApiError::Unauthorized`] if the key is invalid.
///
/// # Panics
///
/// Panics if ... (only document real panics — remove this section if none)
///
/// # Examples
///
/// ```rust
/// let result = do_the_thing("input")?;
/// assert_eq!(result, expected);
/// ```
pub fn do_the_thing(input: &str) -> Result<Output, ApiError> {
````

---

## 7. Testing Conventions

### 7.1 Test Names

Use descriptive names that read as English sentences:

```rust
#[test]
fn duration_formats_zero_seconds_as_empty_string() { ... }

#[test]
fn duration_formats_90_seconds_as_1m() { ... }

#[test]
fn client_returns_unauthorized_on_401_response() { ... }

#[test]
fn cache_returns_stale_entry_after_ttl_expires() { ... }
```

### 7.2 Test Structure (AAA Pattern)

```rust
#[test]
fn it_does_the_thing() {
    // Arrange
    let input = SummaryData { total_seconds: 24120, /* ... */ };
    let opts = RenderOptions::default();

    // Act
    let output = render_table(&input, &opts);

    // Assert
    assert!(output.contains("6h 42m"));
    insta::assert_snapshot!(output);
}
```

### 7.3 Integration Test Structure

```rust
// tests/integration/test_summaries.rs
use waka_api::{WakaClient, SummaryParams};
use wiremock::{MockServer, Mock, ResponseTemplate};

#[tokio::test]
async fn summaries_today_returns_parsed_response() {
    // Arrange
    let server = MockServer::start().await;
    let fixture = include_str!("../fixtures/summaries_today.json");

    Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path_regex(r"/summaries"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture, "application/json"))
        .mount(&server)
        .await;

    let client = WakaClient::with_base_url("test-key", &server.uri()).unwrap();

    // Act
    let result = client.summaries(SummaryParams::today()).await;

    // Assert
    assert!(result.is_ok());
    let summary = result.unwrap();
    assert!(!summary.data.is_empty());
}
```

---

## 8. Dependency Conventions

### 8.1 Workspace-Level Dependencies

All shared dependencies are declared in the workspace `Cargo.toml` with pinned versions:

```toml
[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
reqwest = { version = "0.13", features = ["json", "gzip", "stream"] }
```

Crates inherit with:

```toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
```

This ensures one version per dependency across the workspace.

### 8.2 Feature Flags

Prefer minimal feature sets. Add features only when needed:

```toml
# ✓ Explicit, minimal
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time"] }

# ✗ Lazy
tokio = { version = "1", features = ["full"] }
# (acceptable only in the binary crate)
```

---

## 9. Open Source Project Conventions

### 9.1 Issue Labels

| Label                 | Color  | Purpose                    |
| --------------------- | ------ | -------------------------- |
| `bug`                 | red    | Something isn't working    |
| `feature`             | blue   | New feature request        |
| `documentation`       | teal   | Documentation improvement  |
| `good first issue`    | green  | Good for newcomers         |
| `help wanted`         | yellow | Extra attention needed     |
| `breaking change`     | orange | Would break existing users |
| `phase/0`, `phase/1`… | grey   | Tracks which phase         |
| `wontfix`             | white  | Will not be fixed          |
| `duplicate`           | grey   | Duplicate issue            |

### 9.2 PR Review Checklist (for maintainers)

- [ ] Tests added for new functionality
- [ ] `cargo clippy` passes
- [ ] `cargo fmt` applied
- [ ] Doc comments on new public items
- [ ] Conventional commit message
- [ ] CHANGELOG will be auto-generated (no manual edit needed)
- [ ] `SPEC.md` referenced if implementing a spec feature
- [ ] Breaking change noted if applicable
