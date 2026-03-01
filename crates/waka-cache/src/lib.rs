//! Local cache abstraction for `waka`.
//!
//! Wraps [`sled`] to provide a typed, TTL-aware key-value store used by the
//! `waka` CLI to cache `WakaTime` API responses and reduce network calls.
