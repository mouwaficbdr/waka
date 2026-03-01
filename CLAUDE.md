# CLAUDE.md — Instructions for Claude Code Agent

> This file is your **primary directive**. Read it entirely before touching a single file.
> It governs how you think, how you code, how you test, and how you commit.
> When in doubt: re-read this file.

---

## 0. Project Identity

You are the **sole developer** of `waka`, an open source WakaTime CLI written in Rust.
The full specification lives in `SPEC.md`. The phased development plan lives in `DEVELOPMENT_PLAN.md`.
The human mouwaficbdr does not write code — your output IS the project.

This means:

- Your code must be **production quality**, not "good enough for now"
- Your commits are **permanent history**, not drafts
- Your architecture decisions **cannot be undone easily** — think before acting
- The codebase must be **understandable by a future open source contributor** who has never spoken to you

---

## 1. Mindset & Approach

### 1.1 Spec-Driven Development

**Never invent requirements.** Every feature, every flag, every behavior must trace back to `SPEC.md`.
Before implementing anything, ask: _"Where is this specified?"_
If it's not in `SPEC.md`, open a `## Unspecified` note in your work log before proceeding.

### 1.2 Think Before You Code

For any non-trivial task (anything beyond fixing a typo or adding a simple field):

1. **Read** the relevant section of `SPEC.md`
2. **Read** the existing code in the affected crate(s)
3. **Write** a short implementation plan as a comment block or as a doc comment on the public API
4. **Implement** in the smallest logical increment possible
5. **Test** before moving on
6. **Commit** with a clear message

Do not write 500 lines and then test. Write 50, test, commit, repeat.

### 1.3 Progressive & Atomic Progress

Each unit of work must be:

- **Atomic** : one logical concern per commit
- **Green** : all tests pass before committing
- **Documented** : public APIs have doc comments
- **Reviewed** : run `cargo clippy` before committing

The project advances phase by phase as defined in `DEVELOPMENT_PLAN.md`.
**Do not start Phase 2 until Phase 1 is complete and green.**
Check off tasks in `DEVELOPMENT_PLAN.md` as you complete them.

---

## 2. Repository Structure

```
waka/
├── CLAUDE.md                ← You are here
├── SPEC.md                  ← The law. Read before implementing anything.
├── DEVELOPMENT_PLAN.md      ← Your task list. Check off as you go.
├── CONVENTIONS.md           ← Commit format, code style, naming
├── CHANGELOG.md             ← Auto-maintained via git-cliff
├── README.md                ← Public face of the project
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── LICENSE                  ← MIT
├── Cargo.toml               ← Workspace root
├── Cargo.lock
├── dist-workspace.toml      ← cargo-dist config
├── .cargo/config.toml       ← Cross-compilation, build flags
├── .clippy.toml             ← Clippy config
├── rustfmt.toml             ← Formatter config
├── cliff.toml               ← git-cliff CHANGELOG config
├── deny.toml                ← cargo-deny config
├── .github/
│   ├── copilot-instructions.md
│   └── workflows/
│       ├── ci.yml
│       ├── release.yml
│       └── audit.yml
├── crates/
│   ├── waka/                ← Binary entrypoint
│   ├── waka-api/            ← Public HTTP client lib
│   ├── waka-config/         ← Config & credentials
│   ├── waka-cache/          ← Local cache abstraction
│   ├── waka-render/         ← All output renderers
│   └── waka-tui/            ← Ratatui dashboard
├── completions/             ← Shell completions (generated)
├── man/                     ← Man pages
├── docs/                    ← mdBook documentation
└── tests/
    ├── integration/         ← Workspace-level integration tests
    └── fixtures/            ← Mock API JSON responses
```

---

## 3. Rust Standards

### 3.1 Edition & MSRV

```toml
edition = "2021"
rust-version = "1.82.0"   # Minimum Supported Rust Version — never break below this
```

### 3.2 Code Quality Rules

These are **non-negotiable**:

- `#![deny(missing_docs)]` on all `lib.rs` files in public crates (`waka-api`, `waka-render`)
- `#![deny(clippy::all, clippy::pedantic)]` in `lib.rs` — fix every warning, no `#[allow(...)]` without a comment explaining why
- No `unwrap()` or `expect()` in library code — use `?` and proper error types
- `unwrap()` is only acceptable in tests and in `main()` for truly unrecoverable startup errors (with a comment)
- No `clone()` without thinking — if you're cloning, ask if a reference or lifetime annotation is cleaner
- No `println!` in library code — use `tracing` for diagnostics

### 3.3 Error Handling

- `waka-api` and all lib crates: define custom error types with `thiserror`
- Binary crate `waka`: use `anyhow` for error propagation in command handlers
- Errors surfaced to the user must be **human-readable** — see `SPEC.md §10.2`
- Never expose raw HTTP errors or serde errors to the user without wrapping

```rust
// ✓ Good — user-facing error
Error: Could not connect to WakaTime API
Reason: Request timed out after 10 seconds

// ✗ Bad — never expose this to the user
error sending request for url (https://...): error trying to connect: tcp connect error: ...
```

### 3.4 Async

- Use `tokio` as the async runtime — `#[tokio::main]` in `main.rs`
- Prefer `async fn` over `impl Future` for readability
- No blocking calls inside async contexts — use `tokio::task::spawn_blocking` if needed
- Keep async code at the boundary (HTTP, file I/O) — core logic should be sync

### 3.5 Documentation

Every public item in `waka-api` and `waka-render` must have a doc comment:

````rust
/// Fetches the coding summary for a given date range.
///
/// # Errors
/// Returns [`ApiError::Unauthorized`] if the API key is invalid.
/// Returns [`ApiError::RateLimit`] if the rate limit is exceeded.
///
/// # Example
/// ```rust
/// let client = WakaClient::new("my-api-key");
/// let summary = client.summaries(SummaryParams::today()).await?;
/// ```
pub async fn summaries(&self, params: SummaryParams) -> Result<SummaryResponse, ApiError> {
````

---

## 4. Testing Standards

### 4.1 Philosophy

**No feature without a test.** Every public function in library crates must have at least one unit test.
Integration tests validate the full command flow against mock API responses.

### 4.2 Test Organization

```rust
// In each module — unit tests live next to the code they test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_formatting_zero() { ... }

    #[test]
    fn test_duration_formatting_hours_and_minutes() { ... }
}
```

Integration tests live in `tests/integration/` and use `wiremock` for HTTP mocking.

### 4.3 Test Fixtures

All mock API responses live in `tests/fixtures/`. They are **real anonymized responses** from the WakaTime API, not invented structures. Keep them up to date.

```
tests/fixtures/
├── summaries_today.json
├── summaries_week.json
├── stats_last_7_days.json
├── projects.json
├── goals.json
├── user_current.json
└── errors/
    ├── 401_unauthorized.json
    ├── 429_rate_limit.json
    └── 500_server_error.json
```

### 4.4 Snapshot Testing

Use `insta` for render output testing. When adding a new renderer or changing output format:

```rust
#[test]
fn test_today_table_render() {
    let data = load_fixture("summaries_today.json");
    let output = render_table(&data, &RenderOptions::default());
    insta::assert_snapshot!(output);
}
```

Run `cargo insta review` to approve new/changed snapshots. Snapshots are committed to the repo.

### 4.5 Running Tests

```bash
cargo nextest run              # All tests (preferred — faster than cargo test)
cargo test                     # Fallback
cargo test -p waka-api         # Tests for a single crate
cargo insta test               # Snapshot tests
cargo insta review             # Review changed snapshots
```

**All tests must pass before any commit.**

---

## 5. Git & Commit Standards

### 5.1 Conventional Commits

**Every commit must follow this format exactly:**

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

**Types:**

- `feat` — new feature (triggers MINOR version bump)
- `fix` — bug fix (triggers PATCH version bump)
- `docs` — documentation only
- `test` — adding or fixing tests
- `refactor` — code change that neither fixes a bug nor adds a feature
- `chore` — build process, dependency updates, tooling
- `ci` — CI/CD changes
- `perf` — performance improvement
- `style` — formatting, missing semicolons (no logic change)

**Scopes** (match the crate or feature):
`api`, `cache`, `config`, `render`, `tui`, `auth`, `stats`, `projects`, `goals`, `report`, `prompt`, `completions`, `ci`, `docs`, `deps`

**Examples:**

```
feat(api): implement summaries endpoint with pagination
fix(cache): handle corrupted sled database gracefully
test(render): add snapshot tests for today table output
docs(api): add doc comments to WakaClient public methods
refactor(config): extract credential storage into dedicated module
chore(deps): update reqwest to 0.13.2
feat(auth): implement keychain storage via keyring crate

BREAKING CHANGE: renamed WakaClient::get_stats to WakaClient::stats
```

### 5.2 Commit Cadence

Commit **early and often**. A good rule of thumb:

- After writing a new module skeleton
- After implementing a function and its tests
- After fixing a bug
- After adding documentation
- **Never** accumulate more than ~100 lines of untested code before committing

### 5.3 Pre-Commit Checklist

Before **every** commit, verify:

```bash
# 1. Formatter
cargo fmt --all -- --check

# 2. Linter (zero warnings allowed)
cargo clippy --all-targets --all-features -- -D warnings

# 3. Tests
cargo nextest run

# 4. Build in release mode (catches issues fmt/clippy miss)
cargo build --release

# 5. If you touched waka-api or waka-render: check docs
cargo doc --no-deps -p waka-api
```

If any of these fail, **do not commit**. Fix the issue first.

### 5.4 Branch Strategy

```
main          ← always green, always releasable
  └── phase/0-foundations        ← Phase 0 work
  └── phase/1-completeness       ← Phase 1 work
  └── feat/dashboard-tui         ← Large feature branches
  └── fix/cache-corruption       ← Bug fix branches
```

Merge to `main` only when a phase or feature is complete and green.

---

## 6. Development Workflow — Step by Step

For every task in `DEVELOPMENT_PLAN.md`, follow this exact process:

### Step 1: Understand

- Read the task description in `DEVELOPMENT_PLAN.md`
- Read the relevant section in `SPEC.md`
- Read the existing code that will be affected
- If something is unclear: write a `// QUESTION:` comment and make a conservative decision, note it

### Step 2: Plan

Write a brief implementation note before coding (as a doc comment on the module, or a `// PLAN:` block):

```
// PLAN: Implement SummaryParams builder
// - Start/end dates are required
// - project and language are optional filters
// - Implements Display for use in query string
// - Unit tests: today(), range(), with_project()
```

### Step 3: Implement

- Write the public API first (types, function signatures, doc comments)
- Then write the tests
- Then implement the logic
- This order keeps you honest about the API design

### Step 4: Test

```bash
cargo nextest run -p <crate>
```

Green? Proceed. Red? Fix before moving on. Never move on with failing tests.

### Step 5: Polish

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo doc --no-deps   # spot missing doc comments
```

### Step 6: Commit

Write a clear conventional commit. Reference the phase/task if helpful.

### Step 7: Check off

Mark the task as done in `DEVELOPMENT_PLAN.md` with `[x]`.

---

## 7. Architecture Decisions & Guardrails

### 7.1 Crate Responsibilities — Hard Rules

| Crate         | Can import                                       | Cannot import                                          |
| ------------- | ------------------------------------------------ | ------------------------------------------------------ |
| `waka-api`    | `reqwest`, `serde`, `thiserror`, `chrono`        | `waka-cache`, `waka-config`, `waka-render`, `waka-tui` |
| `waka-cache`  | `sled`, `serde`, `chrono`                        | `waka-api`, `waka-render`, `waka-tui`                  |
| `waka-config` | `directories`, `toml`, `keyring`                 | `waka-api`, `waka-cache`, `waka-render`, `waka-tui`    |
| `waka-render` | `comfy-table`, `owo-colors`                      | `waka-api`, `waka-cache`, `waka-config`, `waka-tui`    |
| `waka-tui`    | `ratatui`, `crossterm`, `waka-api`, `waka-cache` | `waka-render` (has its own rendering)                  |
| `waka` (bin)  | Everything                                       | —                                                      |

These boundaries keep `waka-api` publishable independently on crates.io.

### 7.2 No Circular Dependencies

If you find yourself needing a circular dependency, you have a design problem. Stop. Re-read the architecture. The answer is to extract a shared types crate or reconsider which crate owns the type.

### 7.3 API Stability

`waka-api` public types are **stable from v0.1.0**. Do not rename, remove, or change the type of public fields without a MAJOR version bump. Adding optional fields is fine.

### 7.4 Config File Stability

The shape of `~/.config/waka/config.toml` is **stable from v0.1.0**. Unknown fields must be silently ignored (use `#[serde(deny_unknown_fields)]` only on internal structs, never on user-facing config).

### 7.5 Exit Codes

Always use the exit codes defined in `SPEC.md §Annexe B`. Never exit with code 0 on an error.

```rust
// In waka/src/main.rs
std::process::exit(ExitCode::AuthError as i32);
```

---

## 8. User-Facing Output Rules

### 8.1 Color & TTY Detection

```rust
// Always detect, never assume
let use_color = match config.output.color {
    ColorMode::Auto => console::colors_enabled() && std::io::stdout().is_terminal(),
    ColorMode::Always => true,
    ColorMode::Never => false,
};
```

`NO_COLOR` env var is handled automatically by the `console` crate — do not re-implement this.

### 8.2 Piped Output

When stdout is not a TTY (piped to another command or file), default to `--format=plain` and no colors. The `comfy-table` crate handles this automatically with `ContentArrangement::Disabled`.

### 8.3 Spinner Protocol

Any operation that makes a network request **must** show a spinner. Use `indicatif`:

```rust
let pb = ProgressBar::new_spinner();
pb.set_message("Fetching stats from WakaTime API...");
pb.enable_steady_tick(Duration::from_millis(80));

let result = client.summaries(params).await?;

pb.finish_and_clear();
```

**If stdout is not a TTY, do not show a spinner.**

### 8.4 Error Output

All errors go to **stderr**, never stdout. This preserves the Unix piping contract.

```rust
eprintln!("{}", format_error(err));
```

---

## 9. Security Rules

These are **absolute** — no exceptions, no "just for testing":

1. **Never log the API key**. Not in `--verbose`, not in error messages, not in debug builds.
2. **Never store the API key in plain text** unless it's the last-resort fallback with 0600 permissions.
3. **Always use rustls** — never enable the `native-tls` feature of reqwest.
4. **Never disable TLS certificate verification** — not even in tests (use wiremock instead).
5. **The `WAKA_INSECURE` env var must not exist** — there is no insecure mode.

---

## 10. Dependency Management

### 10.1 Adding a New Dependency

Before adding any crate, ask:

1. Is this **really necessary** or can I implement it in 20 lines?
2. Is it **actively maintained** (last commit < 6 months)?
3. Does it have a **compatible license** (MIT, Apache-2.0, BSD)?
4. Does it add a **native dependency** that would break static compilation? (if yes: find an alternative)
5. Run `cargo deny check` after adding it

### 10.2 Avoid

- Any crate that pulls in `openssl` (use `rustls-tls` features instead)
- Any crate that requires Python, Node.js, or system libraries
- Crates with `unsafe` code unless they're foundational (e.g., `tokio`, `serde`)
- Crates with < 1000 downloads/month (unless absolutely necessary)

### 10.3 Keep Lean

The final binary should be < 10MB. Check binary size in release mode after adding dependencies:

```bash
cargo build --release
ls -lh target/release/waka
```

---

## 11. Documentation Rules

### 11.1 README.md

Must always reflect the **current state** of the project, not the planned state. If a feature isn't implemented yet, it is **not in the README**. A `## Roadmap` section lists what's coming.

### 11.2 CHANGELOG.md

Maintained by `git-cliff` from conventional commits. Never manually edit `CHANGELOG.md`.
Generate with: `git cliff --output CHANGELOG.md`

### 11.3 In-Code Documentation

- All public items in `waka-api` and `waka-render`: doc comments mandatory
- Private functions: doc comments recommended for non-obvious logic
- Complex algorithms: inline comments explaining the _why_, not the _what_

```rust
// ✓ Good — explains why
// WakaTime API uses seconds internally but the display spec requires h/m/s format.
// We round down to the nearest minute to match the web dashboard behavior.
let minutes = total_seconds / 60;

// ✗ Bad — explains what (already obvious from code)
// Divide by 60 to get minutes
let minutes = total_seconds / 60;
```

---

## 12. When You're Stuck or Uncertain

If you encounter a situation not covered by this document or `SPEC.md`:

1. **Default to simplicity** — the simpler implementation is almost always correct
2. **Default to explicitness** — prefer `match` over `if let` chains, explicit error types over `Box<dyn Error>`
3. **Default to safety** — if a user-facing behavior is ambiguous, choose the behavior that least risks data loss or credential exposure
4. **Leave a `// TODO(spec):` comment** if you're making a decision that should be validated against the spec

When you discover a gap or inconsistency in `SPEC.md`, do not silently paper over it. Add an entry to a `SPEC_GAPS.md` file (create it if it doesn't exist) documenting the ambiguity and your resolution.

---

## 13. Phase Completion Criteria

A phase is **complete** when all of the following are true:

- [ ] All tasks in `DEVELOPMENT_PLAN.md` for that phase are checked off
- [ ] `cargo nextest run` passes with zero failures
- [ ] `cargo clippy --all-targets -- -D warnings` produces zero warnings
- [ ] `cargo fmt --all -- --check` produces no diff
- [ ] `cargo doc --no-deps` produces no warnings
- [ ] `cargo build --release` succeeds for the host platform
- [ ] All new public APIs have doc comments
- [ ] `README.md` reflects the current feature set accurately
- [ ] `DEVELOPMENT_PLAN.md` is updated with `[x]` on all completed tasks

Only then: merge to `main`, tag the release if appropriate, start the next phase.

---

_This file is the contract between you and the project. Respect it on every task, every commit, every decision._
