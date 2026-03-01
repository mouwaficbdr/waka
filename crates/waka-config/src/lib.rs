//! Configuration and credential management for `waka`.
//!
//! Handles loading/saving `config.toml`, resolving XDG/platform paths, and
//! the credential priority chain (env var → keychain → credentials file).
