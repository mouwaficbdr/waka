# SPEC_GAPS.md — Specification Gaps and Resolutions

This file documents cases where an implementation decision diverges from or
is not covered by `SPEC.md`. Each entry includes the gap, the resolution
adopted, and a `// TODO(spec):` marker in the affected code if applicable.

---

## §1 — MSRV: 1.82.0 → 1.88.0

**Gap:** `SPEC.md` and `CLAUDE.md §3.1` specify `rust-version = "1.82.0"`.

**Reality:** Several transitive dependencies impose a higher minimum:

| Crate                  | Required MSRV |
| ---------------------- | ------------- |
| `darling` v0.23+       | 1.88.0        |
| `darling_core` v0.23+  | 1.88.0        |
| `darling_macro` v0.23+ | 1.88.0        |
| `instability` v0.3+    | 1.88          |
| `time` v0.3.47+        | 1.88.0        |
| `time-core` v0.1.8+    | 1.88.0        |

These crates are pulled in transitively by `clap`, `ratatui`, and `chrono`.
Pinning older versions would require accepting known security advisories.

**Resolution:** Updated `rust-version` in `Cargo.toml` to `"1.88.0"` and
updated the MSRV CI job accordingly. The effective MSRV is now **1.88.0**.

**Spec action required:** Update `SPEC.md §3` and `CLAUDE.md §3.1` to
reflect `rust-version = "1.88.0"`.

---

## §2 — `Stats::weeks` field is nullable

**Gap:** The `SPEC.md` does not document the shape of the stats response.

**Reality:** The `WakaTime` API `/users/current/stats/{range}` response
includes a `weeks` field that can be `null`. This field is present in real
API responses but not documented in the spec.

**Resolution:** Added `weeks: Option<serde_json::Value>` is _not_ modelled
in the `Stats` struct — the field is silently ignored via serde's default
unknown-field behavior. If this field becomes needed, it should be added
as `Option<Vec<WeeklyStats>>` once the shape is confirmed.

---

## §3 — Flexible date parsing for `waka stats range`

**Gap:** `DEVELOPMENT_PLAN.md §1.3` lists accepting `today`, `yesterday`, `7d ago`,
and `last monday` as values for `--from` / `--to`. `SPEC.md` only shows `YYYY-MM-DD`
in all examples.

**Resolution:** Only `YYYY-MM-DD` is accepted for the `--from` / `--to` flags, matching
every example in `SPEC.md`. If natural-language date parsing is desired it should first
be specified in `SPEC.md` (e.g. using the `chrono-humanize` or `dateparser` crate).

---

## §4 — Cache expired-hit refresh strategy

**Gap:** `DEVELOPMENT_PLAN.md §1.2` specifies "refresh in background, update display"
for expired cache entries. This implies spawning a background task and re-drawing the
terminal — behaviour more suited to a TUI than a CLI command.

**Resolution:** On expired hit the command awaits the refresh (showing a spinner).
On success the fresh result is displayed; on network failure the stale entry is shown
with a `⚠ offline` badge. This matches the SPEC's offline-first intention without
requiring a TUI redraw loop.

---

_New gaps should be appended here as they are discovered._
