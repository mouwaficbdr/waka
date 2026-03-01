//! Core configuration types and file I/O for `waka`.
//!
//! # Config file location
//!
//! | Platform | Path |
//! |----------|------|
//! | Linux    | `$XDG_CONFIG_HOME/waka/config.toml` (falls back to `~/.config/waka/config.toml`) |
//! | macOS    | `~/Library/Application Support/waka/config.toml` |
//! | Windows  | `%APPDATA%\waka\config.toml` |

use std::collections::HashMap;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::error::ConfigError;

// ─────────────────────────────────────────────────────────────────────────────

/// Colour-output mode.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorMode {
    /// Enable colours when stdout is a TTY and `NO_COLOR` is not set.
    #[default]
    Auto,
    /// Always enable colours.
    Always,
    /// Disable colours.
    Never,
}

/// Output format for tabular data.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Human-readable table (default).
    #[default]
    Table,
    /// Machine-readable JSON.
    Json,
    /// CSV (comma-separated values).
    Csv,
    /// Plain text, no ANSI, no borders.
    Plain,
    /// TSV (tab-separated values).
    Tsv,
}

/// Which day the week begins on for weekly summaries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WeekStart {
    /// ISO 8601 standard — week begins on Monday.
    #[default]
    Monday,
    /// US convention — week begins on Sunday.
    Sunday,
}

// ─────────────────────────────────────────────────────────────────────────────

/// `[core]` section of `config.toml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CoreConfig {
    /// Name of the active profile.  Defaults to `"default"`.
    pub default_profile: String,
    /// Whether to check for `waka` updates on startup.
    pub update_check: bool,
    /// Reserved — always `false`.  Telemetry is never collected.
    pub telemetry: bool,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            default_profile: "default".to_owned(),
            update_check: true,
            telemetry: false,
        }
    }
}

/// `[output]` section of `config.toml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OutputConfig {
    /// Colour output mode.
    pub color: ColorMode,
    /// Default output format.
    pub format: OutputFormat,
    /// `strftime`-compatible date format string.
    pub date_format: String,
    /// Use 24-hour clock for time values.
    pub time_format_24h: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            color: ColorMode::Auto,
            format: OutputFormat::Table,
            date_format: "%Y-%m-%d".to_owned(),
            time_format_24h: true,
        }
    }
}

/// `[cache]` section of `config.toml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CacheConfig {
    /// Whether caching is enabled.
    pub enabled: bool,
    /// Cache TTL in seconds.
    pub ttl_seconds: u64,
    /// Override path for the `sled`-backed cache store.
    /// `None` means the platform default is used.
    pub path: Option<String>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ttl_seconds: 300,
            path: None,
        }
    }
}

/// `[display]` section of `config.toml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DisplayConfig {
    /// Render ASCII progress bars next to time entries.
    pub show_progress_bar: bool,
    /// Render sparklines in weekly/monthly summaries.
    pub show_sparklines: bool,
    /// First day of the week for weekly summaries.
    pub week_start: WeekStart,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_progress_bar: true,
            show_sparklines: true,
            week_start: WeekStart::Monday,
        }
    }
}

/// A single named profile under `[profiles.<name>]`.
///
/// The API key is **never** stored here — it lives in the keychain or a
/// separate credentials file. See [`crate::credentials::CredentialStore`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProfileConfig {
    /// Base URL of the `WakaTime`-compatible API to target.
    /// Useful for self-hosted instances.
    pub api_url: String,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            api_url: "https://wakatime.com/api/v1".to_owned(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────

/// Top-level `waka` configuration, corresponding to `config.toml`.
///
/// Unknown fields are silently ignored during deserialisation so that newer
/// versions of the tool can add keys without breaking older configs.
///
/// # Example
///
/// ```rust,no_run
/// use waka_config::Config;
///
/// let config = Config::load().unwrap_or_default();
/// println!("format: {:?}", config.output.format);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Core settings.
    pub core: CoreConfig,
    /// Output format and colour settings.
    pub output: OutputConfig,
    /// Cache settings.
    pub cache: CacheConfig,
    /// Display settings (progress bars, sparklines, week start).
    pub display: DisplayConfig,
    /// Named profiles keyed by profile name.
    pub profiles: HashMap<String, ProfileConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert("default".to_owned(), ProfileConfig::default());
        Self {
            core: CoreConfig::default(),
            output: OutputConfig::default(),
            cache: CacheConfig::default(),
            display: DisplayConfig::default(),
            profiles,
        }
    }
}

impl Config {
    /// Returns the platform-specific path to `config.toml`.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::NoConfigDir`] if the home directory cannot be
    /// determined (uncommon — usually means a sandboxed environment).
    pub fn path() -> Result<PathBuf, ConfigError> {
        let dirs = ProjectDirs::from("", "", "waka").ok_or(ConfigError::NoConfigDir)?;
        Ok(dirs.config_dir().join("config.toml"))
    }

    /// Returns the platform-specific cache directory for `waka`.
    ///
    /// | Platform | Path |
    /// |----------|------|
    /// | Linux    | `$XDG_CACHE_HOME/waka/` or `~/.cache/waka/` |
    /// | macOS    | `~/Library/Caches/waka/` |
    /// | Windows  | `%LOCALAPPDATA%\waka\cache\` |
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::NoConfigDir`] if the home directory cannot be
    /// determined.
    pub fn cache_dir() -> Result<PathBuf, ConfigError> {
        let dirs = ProjectDirs::from("", "", "waka").ok_or(ConfigError::NoConfigDir)?;
        Ok(dirs.cache_dir().to_path_buf())
    }

    /// Loads the config from the platform default path.
    ///
    /// If the file does not exist, [`Config::default()`] is returned.
    /// Unknown TOML keys are silently ignored.
    ///
    /// # Errors
    ///
    /// Returns an error if the path cannot be determined, the file cannot be
    /// read, or the TOML is malformed.
    pub fn load() -> Result<Self, ConfigError> {
        Self::load_from(&Self::path()?)
    }

    /// Loads the config from an explicit path (useful for testing).
    ///
    /// If the file does not exist, [`Config::default()`] is returned.
    ///
    /// # Errors
    ///
    /// Returns an error if the file exists but cannot be read or parsed.
    pub fn load_from(path: &PathBuf) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&raw)?;
        Ok(config)
    }

    /// Saves the config to the platform default path, creating the directory
    /// if necessary.
    ///
    /// # Errors
    ///
    /// Returns an error if the path cannot be determined, the directory cannot
    /// be created, or the file cannot be written.
    pub fn save(&self) -> Result<(), ConfigError> {
        self.save_to(&Self::path()?)
    }

    /// Saves the config to an explicit path (useful for testing).
    ///
    /// Creates parent directories as needed. Existing files are overwritten.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization or disk write fails.
    pub fn save_to(&self, path: &PathBuf) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let toml = toml::to_string_pretty(self)?;
        std::fs::write(path, toml)?;
        Ok(())
    }

    /// Returns the [`ProfileConfig`] for the active profile (as defined in
    /// `core.default_profile`), falling back to a freshly constructed default
    /// if the profile name is not present in `self.profiles`.
    #[must_use]
    pub fn active_profile(&self) -> &ProfileConfig {
        // `Config::default()` always inserts "default"; this branch is only
        // reachable if the user manually removed all profiles from the TOML.
        // TODO(spec): clarify whether a missing profile should be an error.
        static FALLBACK: std::sync::OnceLock<ProfileConfig> = std::sync::OnceLock::new();
        self.profiles
            .get(&self.core.default_profile)
            .unwrap_or_else(|| FALLBACK.get_or_init(ProfileConfig::default))
    }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("waka_config_test_{name}.toml"))
    }

    fn cleanup(path: &PathBuf) {
        let _ = std::fs::remove_file(path);
    }

    // ── Default values ────────────────────────────────────────────────────────

    #[test]
    fn default_core_profile_is_default() {
        assert_eq!(Config::default().core.default_profile, "default");
    }

    #[test]
    fn default_telemetry_is_false() {
        assert!(!Config::default().core.telemetry);
    }

    #[test]
    fn default_output_format_is_table() {
        assert_eq!(Config::default().output.format, OutputFormat::Table);
    }

    #[test]
    fn default_color_mode_is_auto() {
        assert_eq!(Config::default().output.color, ColorMode::Auto);
    }

    #[test]
    fn default_date_format() {
        assert_eq!(Config::default().output.date_format, "%Y-%m-%d");
    }

    #[test]
    fn default_ttl_is_300() {
        assert_eq!(Config::default().cache.ttl_seconds, 300);
    }

    #[test]
    fn default_cache_enabled() {
        assert!(Config::default().cache.enabled);
    }

    #[test]
    fn default_week_start_is_monday() {
        assert_eq!(Config::default().display.week_start, WeekStart::Monday);
    }

    #[test]
    fn default_profile_api_url() {
        let cfg = Config::default();
        assert_eq!(cfg.active_profile().api_url, "https://wakatime.com/api/v1");
    }

    // ── Round-trip save / load ───────────────────────────────────────────────

    #[test]
    fn save_and_load_roundtrip() {
        let path = temp_path("roundtrip");
        cleanup(&path);

        let original = Config::default();
        original.save_to(&path).expect("save_to should succeed");
        let loaded = Config::load_from(&path).expect("load_from should succeed");
        assert_eq!(original, loaded);

        cleanup(&path);
    }

    #[test]
    fn load_from_nonexistent_returns_default() {
        let path = temp_path("nonexistent_xyz_12345");
        cleanup(&path); // ensure it doesn't exist
        let cfg = Config::load_from(&path).expect("should succeed with default");
        assert_eq!(cfg, Config::default());
    }

    #[test]
    fn load_ignores_unknown_fields() {
        let path = temp_path("unknown_fields");
        cleanup(&path);

        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(
            &path,
            r#"
[core]
default_profile = "default"
totally_unknown_key = "surprise"

[output]
color = "never"
another_unknown = 42
"#,
        )
        .unwrap();

        let cfg = Config::load_from(&path).expect("should tolerate unknown fields");
        // Known fields are parsed correctly.
        assert_eq!(cfg.output.color, ColorMode::Never);
        // Unknown fields are silently dropped (no panic / error).
        cleanup(&path);
    }

    #[test]
    fn load_rejects_invalid_toml() {
        let path = temp_path("invalid_toml");
        cleanup(&path);

        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, "[[not valid toml").unwrap();

        let result = Config::load_from(&path);
        assert!(result.is_err(), "should fail on invalid TOML");
        cleanup(&path);
    }

    #[test]
    fn partial_config_fills_missing_fields_with_defaults() {
        let path = temp_path("partial");
        cleanup(&path);

        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(
            &path,
            r#"
[output]
color = "always"
"#,
        )
        .unwrap();

        let cfg = Config::load_from(&path).expect("partial config should load");
        assert_eq!(cfg.output.color, ColorMode::Always);
        // Fields not in the file default to their spec values.
        assert_eq!(cfg.output.format, OutputFormat::Table);
        assert_eq!(cfg.core.default_profile, "default");
        cleanup(&path);
    }

    // ── Enum serde ───────────────────────────────────────────────────────────

    #[test]
    fn color_mode_round_trip_via_toml() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct W {
            v: ColorMode,
        }
        for mode in [ColorMode::Auto, ColorMode::Always, ColorMode::Never] {
            let s = toml::to_string(&W { v: mode.clone() }).unwrap();
            let back: W = toml::from_str(&s).unwrap();
            assert_eq!(mode, back.v);
        }
    }

    #[test]
    fn output_format_round_trip_via_toml() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct W {
            v: OutputFormat,
        }
        for fmt in [
            OutputFormat::Table,
            OutputFormat::Json,
            OutputFormat::Csv,
            OutputFormat::Plain,
        ] {
            let s = toml::to_string(&W { v: fmt.clone() }).unwrap();
            let back: W = toml::from_str(&s).unwrap();
            assert_eq!(fmt, back.v);
        }
    }

    #[test]
    fn week_start_round_trip_via_toml() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct W {
            v: WeekStart,
        }
        for ws in [WeekStart::Monday, WeekStart::Sunday] {
            let s = toml::to_string(&W { v: ws.clone() }).unwrap();
            let back: W = toml::from_str(&s).unwrap();
            assert_eq!(ws, back.v);
        }
    }

    #[test]
    fn cache_dir_returns_ok_in_normal_environment() {
        // On any platform where the home directory is determinable, cache_dir()
        // should succeed and return a path that ends with "waka".
        // If it errors (sandboxed env), that is acceptable — just not a panic.
        if let Ok(path) = Config::cache_dir() {
            assert!(
                path.ends_with("waka") || path.to_string_lossy().contains("waka"),
                "cache dir should be under a 'waka' directory, got: {path:?}"
            );
        }
    }
}
