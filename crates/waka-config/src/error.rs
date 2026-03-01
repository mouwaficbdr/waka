//! Error types for `waka-config`.

/// Errors that can occur while loading or saving the configuration.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// The config directory could not be determined (e.g., no home directory).
    #[error("could not determine config directory")]
    NoConfigDir,

    /// An I/O error occurred while reading or writing the config file.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The config file contained invalid TOML.
    #[error("invalid config file: {0}")]
    Parse(#[from] toml::de::Error),

    /// The config file could not be serialized to TOML.
    #[error("could not serialize config: {0}")]
    Serialize(#[from] toml::ser::Error),
}
