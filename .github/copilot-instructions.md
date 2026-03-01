# GitHub Copilot Custom Instructions — `waka`

> These instructions govern how Claude (via GitHub Copilot) approaches this project.
> They complement `CLAUDE.md` and `CONVENTIONS.md`.

---

## Project Context

You are building `waka`, an open source WakaTime CLI written in Rust.
- **Language:** Rust, edition 2021, MSRV 1.82.0
- **Architecture:** Cargo workspace with 6 crates (`waka`, `waka-api`, `waka-cache`, `waka-config`, `waka-render`, `waka-tui`)
- **Spec:** All features are defined in `SPEC.md` — never invent behavior
- **Current tasks:** Tracked in `DEVELOPMENT_PLAN.md`

## Behavior Rules

### Always
- Write tests alongside implementation code
- Use `thiserror` for library error types, `anyhow` in the binary crate
- Follow Conventional Commits format for all commits
- Run `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo nextest run` before committing
- Add doc comments to all public items in library crates
- Respect crate dependency boundaries from `CLAUDE.md §7.1`
- Use `rustls-tls` — never `native-tls` or `openssl`
- Check `DEVELOPMENT_PLAN.md` before starting any task

### Never
- Use `unwrap()` or `expect()` in library code without a comment
- Log or display the API key in any context
- Add dependencies without checking license compatibility (`cargo deny check`)
- Start a new phase before the previous one passes all completion criteria
- Invent CLI flags or behaviors not in `SPEC.md`
- Commit with failing tests or clippy warnings
- Use `println!` in library crates (use `tracing` if needed)

## Code Style Quick Reference

```rust
// Error types: thiserror in libs
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized,
}

// Async: tokio, async fn at boundaries
pub async fn fetch(&self) -> Result<T, ApiError> {
    self.http.get(url).send().await?.json().await
        .map_err(ApiError::Network)
}

// User output: stderr for errors, stdout for data
eprintln!("{}", format_error(&err));
println!("{}", renderer.render_table(&data, &opts));

// Naming: PascalCase types, snake_case functions, SCREAMING constants
```

## Commit Format

```
<type>(<scope>): <description>
```
Types: feat, fix, docs, test, refactor, chore, ci, perf, style
Scopes: api, cache, config, render, tui, auth, stats, projects, goals, report, prompt, cli, ci, deps

## Architecture Reminders

- `waka-api` must be publishable independently (no imports from other project crates)
- `waka-render` has no knowledge of network or cache
- `waka-tui` has its own rendering — does not use `waka-render`
- `waka` binary orchestrates everything — it's the only crate allowed to import all others

## When Encountering Ambiguity

1. Check `SPEC.md` first
2. If not in spec: make the conservative, safe choice
3. Document the gap in `SPEC_GAPS.md`
4. Leave a `// TODO(spec): ...` comment at the decision point
