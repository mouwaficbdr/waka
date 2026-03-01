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

/// Errors that can occur while resolving or storing credentials.
#[derive(Debug, thiserror::Error)]
pub enum CredentialError {
    /// No API key could be found in any of the sources in the priority chain.
    #[error("no API key found — run `waka auth login` or set WAKATIME_API_KEY")]
    NotFound,

    /// The config directory could not be determined.
    #[error("could not determine config directory")]
    NoConfigDir,

    /// The keychain operation failed.
    #[error("keychain error: {0}")]
    Keychain(String),

    /// An I/O error occurred while reading or writing the credentials file.
    #[error("I/O error reading credentials: {0}")]
    Io(#[from] std::io::Error),

    /// The credentials file contained malformed data.
    #[error("malformed credentials file: {0}")]
    Malformed(String),
}
