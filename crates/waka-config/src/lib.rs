//! Configuration and credential management for `waka`.
//!
//! Handles loading/saving `config.toml`, resolving XDG/platform paths, and
//! the credential priority chain (env var → keychain → credentials file).

pub mod config;
pub mod error;

pub use config::{
    CacheConfig, ColorMode, Config, CoreConfig, DisplayConfig, OutputConfig, OutputFormat,
    ProfileConfig, WeekStart,
};
pub use error::ConfigError;
