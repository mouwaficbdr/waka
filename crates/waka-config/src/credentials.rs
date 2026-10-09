//! Credential storage and resolution for `waka`.
//!
//! API keys are resolved in priority order and **never** logged or displayed.
//! Use [`Sensitive`] to hold them safely.

use std::path::PathBuf;

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use directories::ProjectDirs;

use crate::error::CredentialError;

// ─── Sensitive wrapper ────────────────────────────────────────────────────────

/// A `String` wrapper whose `Debug` implementation redacts the value.
///
/// Use this type whenever an API key must be stored in a struct or passed
/// across function boundaries to prevent accidental logging.
///
/// # Example
///
/// ```rust
/// use waka_config::Sensitive;
///
/// let key = Sensitive::new("waka_supersecret");
/// assert_eq!(format!("{key:?}"), "Sensitive(\"[REDACTED]\")");
/// assert_eq!(key.expose(), "waka_supersecret");
/// ```
#[derive(Clone)]
pub struct Sensitive(String);

impl Sensitive {
    /// Wraps an API key.
    #[must_use]
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    /// Returns the raw key value.
    ///
    /// Use only where the key is required (e.g., constructing the HTTP client).
    /// Avoid binding the result to a variable that lives longer than necessary.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Sensitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Sensitive").field(&"[REDACTED]").finish()
    }
}

impl std::fmt::Display for Sensitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

// ─── Keychain constants ───────────────────────────────────────────────────────

/// Keychain service name shared across all installations.
const KEYRING_SERVICE: &str = "waka";

// ─── CredentialStore ─────────────────────────────────────────────────────────

/// Resolves and stores API keys following the priority chain from SPEC.md §5.2.
///
/// Priority (highest → lowest):
/// 1. Explicit key supplied at construction time (e.g., `--api-key` flag)
/// 2. `WAKATIME_API_KEY` environment variable
/// 3. `WAKA_API_KEY` environment variable
/// 4. System keychain (macOS Keychain / Windows Credential Manager / Linux
///    kernel keyring backed by the Secret Service)
/// 5. Per-profile `credentials` file in the config directory (base64-obfuscated,
///    `0600`); still tried when the keychain itself is unavailable
/// 6. `~/.wakatime.cfg` (read-only, backwards-compatibility)
///
/// # Example
///
/// ```rust,no_run
/// use waka_config::CredentialStore;
///
/// # fn example() -> Result<(), waka_config::CredentialError> {
/// let store = CredentialStore::new("default");
/// let key = store.get_api_key()?;
/// // key.expose() gives the raw string — use immediately, don't store long-term
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct CredentialStore {
    /// Profile name — used as the keychain username.
    profile: String,
    /// Explicit key supplied at construction time (e.g., from `--api-key`).
    explicit_key: Option<Sensitive>,
}

impl CredentialStore {
    /// Creates a credential store for the named profile.
    ///
    /// To pass in a key from `--api-key`, use [`CredentialStore::with_explicit_key`].
    #[must_use]
    pub fn new(profile: &str) -> Self {
        Self {
            profile: profile.to_owned(),
            explicit_key: None,
        }
    }

    /// Creates a credential store pre-loaded with an explicit API key.
    ///
    /// When `get_api_key()` is called the explicit key is returned immediately
    /// without consulting any other source. This is the code path for
    /// `waka --api-key <KEY>`.
    #[must_use]
    pub fn with_explicit_key(profile: &str, key: &str) -> Self {
        Self {
            profile: profile.to_owned(),
            explicit_key: Some(Sensitive::new(key)),
        }
    }

    // ── Resolution ────────────────────────────────────────────────────────────

    /// Resolves the API key by walking the priority chain.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::NotFound`] if no key is found in any source.
    /// Returns other variants for I/O or keychain failures.
    /// Resolves the API key by walking the priority chain.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::NotFound`] if no key is found in any source.
    /// Returns other variants for I/O or keychain failures.
    pub fn get_api_key(&self) -> Result<Sensitive, CredentialError> {
        self.resolve(
            std::env::var("WAKATIME_API_KEY").ok().as_deref(),
            std::env::var("WAKA_API_KEY").ok().as_deref(),
        )
    }

    /// Core resolution logic — accepts env overrides so it can be tested
    /// without mutating global process state.
    pub(crate) fn resolve(
        &self,
        wakatime_key: Option<&str>,
        waka_key: Option<&str>,
    ) -> Result<Sensitive, CredentialError> {
        // 1. Explicit key (--api-key flag)
        if let Some(k) = &self.explicit_key {
            return Ok(k.clone());
        }

        // 2. WAKATIME_API_KEY
        if let Some(k) = wakatime_key.filter(|s| !s.is_empty()) {
            return Ok(Sensitive::new(k));
        }

        // 3. WAKA_API_KEY
        if let Some(k) = waka_key.filter(|s| !s.is_empty()) {
            return Ok(Sensitive::new(k));
        }

        // 4. System keychain. An unavailable keychain (no Secret Service on a
        // headless machine, locked keyring, …) must not hide the file fallback
        // below, which is exactly where `auth login` stores the key then.
        let keychain_error = match self.keychain_get() {
            Ok(k) => return Ok(Sensitive::new(k)),
            Err(CredentialError::NotFound) => None,
            Err(e) => Some(e),
        };

        // 5. Per-profile credentials file
        if let Some(path) = credentials_file_path(&self.profile) {
            match read_credentials_file(&path) {
                Ok(Some(k)) => return Ok(Sensitive::new(k)),
                Ok(None) | Err(CredentialError::Io(_)) => {} // absent or no key
                Err(e) => return Err(e),
            }
        }

        // 6. ~/.wakatime.cfg (compat, read-only)
        if let Some(k) = read_wakatime_cfg_at(default_wakatime_cfg_path().as_ref()) {
            return Ok(Sensitive::new(k));
        }

        // Nothing found: surface the keychain failure if there was one, as it
        // is more actionable than a plain "not found".
        Err(keychain_error.unwrap_or(CredentialError::NotFound))
    }

    /// Stores the API key in the system keychain.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::Keychain`] if the system keychain is
    /// unavailable or the write fails.
    pub fn set_api_key(&self, key: &str) -> Result<(), CredentialError> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, &self.profile)
            .map_err(|e| CredentialError::Keychain(e.to_string()))?;
        entry
            .set_password(key)
            .map_err(|e| CredentialError::Keychain(e.to_string()))
    }

    /// Deletes the API key from the system keychain and removes the profile's
    /// fallback credentials file.
    ///
    /// Silently succeeds if neither exists. An unreachable keychain is only an
    /// error when there was no file to remove either.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::Keychain`] if the deletion fails for a
    /// reason other than the entry not existing.
    pub fn delete_api_key(&self) -> Result<(), CredentialError> {
        let keychain_result = keyring::Entry::new(KEYRING_SERVICE, &self.profile)
            .and_then(|entry| entry.delete_credential());

        // Also remove the profile's fallback file, otherwise the key would
        // still be resolved after `waka auth logout`.
        let mut removed_file = false;
        if let Some(path) = credentials_file_path(&self.profile) {
            match std::fs::remove_file(&path) {
                Ok(()) => removed_file = true,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(CredentialError::Io(e)),
            }
        }

        match keychain_result {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            // The keychain is unreachable, so the key cannot be stored there:
            // removing the file was the whole logout.
            Err(_) if removed_file => Ok(()),
            Err(e) => Err(CredentialError::Keychain(e.to_string())),
        }
    }

    /// Saves the API key to the credentials file (base64-obfuscated, 0600).
    ///
    /// Intended as a fallback when the keychain is unavailable.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::NoConfigDir`] if the config path cannot be
    /// resolved, or an I/O error if the write fails.
    pub fn save_to_credentials_file(&self, key: &str) -> Result<(), CredentialError> {
        let path = credentials_file_path(&self.profile).ok_or(CredentialError::NoConfigDir)?;
        write_credentials_file(&path, key)
    }

    // ── Keychain helpers ──────────────────────────────────────────────────────

    fn keychain_get(&self) -> Result<String, CredentialError> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, &self.profile)
            .map_err(|e| CredentialError::Keychain(e.to_string()))?;
        match entry.get_password() {
            Ok(k) => Ok(k),
            Err(keyring::Error::NoEntry) => Err(CredentialError::NotFound),
            Err(e) => Err(CredentialError::Keychain(e.to_string())),
        }
    }
}

// ─── Path helpers ─────────────────────────────────────────────────────────────

/// Returns the fallback credentials file for `profile`.
///
/// The `default` profile keeps the historical `credentials` name; other
/// profiles get their own `credentials-<profile>` file so that keys never
/// leak from one profile to another.
fn credentials_file_path(profile: &str) -> Option<PathBuf> {
    ProjectDirs::from("", "", "waka").map(|d| d.config_dir().join(credentials_file_name(profile)))
}

/// File name used by [`credentials_file_path`]. Characters outside
/// `[A-Za-z0-9_-]` are replaced so a profile name can never escape the config
/// directory (e.g. `--profile ../../x`).
fn credentials_file_name(profile: &str) -> String {
    if profile == "default" {
        return "credentials".to_owned();
    }
    let safe: String = profile
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    format!("credentials-{safe}")
}

fn default_wakatime_cfg_path() -> Option<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()?;
    Some(PathBuf::from(home).join(".wakatime.cfg"))
}

// ─── File readers ─────────────────────────────────────────────────────────────

/// Reads an API key from a `~/.config/waka/credentials`-formatted file.
///
/// Format: `api_key=<base64-encoded key>` (lines starting with `#` are ignored).
pub(crate) fn read_credentials_file(path: &PathBuf) -> Result<Option<String>, CredentialError> {
    let content = std::fs::read_to_string(path)?;
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(encoded) = line.strip_prefix("api_key=") {
            let bytes = B64
                .decode(encoded.trim())
                .map_err(|e| CredentialError::Malformed(e.to_string()))?;
            let key =
                String::from_utf8(bytes).map_err(|e| CredentialError::Malformed(e.to_string()))?;
            return Ok(Some(key));
        }
    }
    Ok(None)
}

/// Writes an API key to a credentials file (base64-obfuscated, 0600 on Unix).
pub(crate) fn write_credentials_file(path: &PathBuf, key: &str) -> Result<(), CredentialError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let encoded = B64.encode(key.as_bytes());
    let content = format!("api_key={encoded}\n");

    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    // Create the file as 0600 on Unix so the key is never readable by other
    // users, not even between creation and a later chmod.
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;

    // `mode` only applies when the file is created: tighten a pre-existing
    // file before writing the key into it.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
    }
    std::io::Write::write_all(&mut file, content.as_bytes())?;

    Ok(())
}

/// Reads an API key from a `WakaTime` CLI-style INI config file at `path`.
///
/// Looks for `api_key = <value>` under the `[settings]` section.
/// Returns `None` if the file is absent or the key is not present.
pub(crate) fn read_wakatime_cfg_at(path: Option<&PathBuf>) -> Option<String> {
    let path = path?;
    let content = std::fs::read_to_string(path).ok()?;
    let mut in_settings = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[settings]" {
            in_settings = true;
            continue;
        }
        if line.starts_with('[') {
            in_settings = false;
            continue;
        }
        if in_settings {
            if let Some(rest) = line.strip_prefix("api_key") {
                let key = rest.trim_start_matches([' ', '=']).trim().to_owned();
                if !key.is_empty() {
                    return Some(key);
                }
            }
        }
    }
    None
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    // ── Sensitive ─────────────────────────────────────────────────────────────

    #[test]
    fn sensitive_debug_redacts_value() {
        let s = Sensitive::new("super_secret_key");
        assert_eq!(format!("{s:?}"), "Sensitive(\"[REDACTED]\")");
    }

    #[test]
    fn sensitive_display_redacts_value() {
        let s = Sensitive::new("super_secret_key");
        assert_eq!(format!("{s}"), "[REDACTED]");
    }

    #[test]
    fn sensitive_expose_returns_raw() {
        let s = Sensitive::new("waka_abc123");
        assert_eq!(s.expose(), "waka_abc123");
    }

    // ── Explicit key ──────────────────────────────────────────────────────────

    #[test]
    fn with_explicit_key_returns_it_immediately() {
        let store = CredentialStore::with_explicit_key("default", "my_key");
        let key = store.get_api_key().expect("should return explicit key");
        assert_eq!(key.expose(), "my_key");
    }

    // ── Env var priority (via resolve()) ──────────────────────────────────────
    //
    // resolve() accepts env-var values directly so we never mutate the process
    // environment (which would require `unsafe` and cause race conditions in
    // parallel tests).

    #[test]
    fn wakatime_api_key_is_resolved() {
        let store = CredentialStore::new("default");
        let key = store
            .resolve(Some("wakatime_env_key"), None)
            .expect("WAKATIME_API_KEY should be resolved");
        assert_eq!(key.expose(), "wakatime_env_key");
    }

    #[test]
    fn waka_api_key_fallback_when_wakatime_absent() {
        let store = CredentialStore::new("default");
        let key = store
            .resolve(None, Some("waka_env_key"))
            .expect("WAKA_API_KEY should be resolved");
        assert_eq!(key.expose(), "waka_env_key");
    }

    #[test]
    fn wakatime_api_key_takes_priority_over_waka_api_key() {
        let store = CredentialStore::new("default");
        let key = store
            .resolve(Some("higher_priority"), Some("lower_priority"))
            .expect("WAKATIME_API_KEY should win");
        assert_eq!(key.expose(), "higher_priority");
    }

    #[test]
    fn explicit_key_takes_priority_over_env_vars() {
        let store = CredentialStore::with_explicit_key("default", "cli_key");
        let key = store
            .resolve(Some("env_key"), Some("other_env_key"))
            .expect("explicit key should win");
        assert_eq!(key.expose(), "cli_key");
    }

    #[test]
    fn empty_env_var_is_skipped() {
        let store = CredentialStore::new("default");
        // Empty WAKATIME_API_KEY should fall through to WAKA_API_KEY.
        let key = store
            .resolve(Some(""), Some("fallback"))
            .expect("should fall through to WAKA_API_KEY");
        assert_eq!(key.expose(), "fallback");
    }

    // ── Credentials file ──────────────────────────────────────────────────────

    #[test]
    fn credentials_file_names_are_per_profile_and_safe() {
        assert_eq!(credentials_file_name("default"), "credentials");
        assert_eq!(credentials_file_name("work"), "credentials-work");
        assert_eq!(credentials_file_name("my-team_2"), "credentials-my-team_2");
        let traversal = credentials_file_name("../../etc/x");
        assert!(
            !traversal.contains('/') && !traversal.contains(".."),
            "{traversal}"
        );
    }

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("waka_creds_{name}"))
    }

    fn remove(path: &PathBuf) {
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn credentials_file_round_trip() {
        let path = temp_path("roundtrip");
        remove(&path);

        write_credentials_file(&path, "waka_rt_key").expect("write should succeed");
        let result = read_credentials_file(&path).expect("read should succeed");
        assert_eq!(result.as_deref(), Some("waka_rt_key"));

        remove(&path);
    }

    #[cfg(unix)]
    #[test]
    fn credentials_file_is_private_even_if_it_existed() {
        use std::os::unix::fs::PermissionsExt as _;

        let path = temp_path("perms");
        std::fs::write(&path, "old").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();

        write_credentials_file(&path, "waka_perm_key").expect("write should succeed");
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        remove(&path);
    }

    #[test]
    fn credentials_file_ignores_comments_and_blank_lines() {
        let path = temp_path("comments");
        remove(&path);

        let encoded = B64.encode("waka_commented".as_bytes());
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "# this is a comment").unwrap();
        writeln!(f).unwrap();
        writeln!(f, "api_key={encoded}").unwrap();
        drop(f);

        let result = read_credentials_file(&path).expect("should parse");
        assert_eq!(result.as_deref(), Some("waka_commented"));

        remove(&path);
    }

    #[test]
    fn credentials_file_returns_none_when_no_api_key_line() {
        let path = temp_path("nokey");
        remove(&path);

        std::fs::write(&path, "# empty\n").unwrap();
        let result = read_credentials_file(&path).expect("should not error");
        assert!(result.is_none());

        remove(&path);
    }

    #[test]
    fn credentials_file_returns_error_on_bad_base64() {
        let path = temp_path("badbase64");
        remove(&path);

        std::fs::write(&path, "api_key=!!not_valid!!\n").unwrap();
        let result = read_credentials_file(&path);
        assert!(result.is_err(), "should fail on bad base64");

        remove(&path);
    }

    // ── ~/.wakatime.cfg ───────────────────────────────────────────────────────

    #[test]
    fn wakatime_cfg_parses_api_key() {
        let path = temp_path("wakatime.cfg");
        remove(&path);

        std::fs::write(&path, "[settings]\napi_key = waka_compat_key\n").unwrap();
        let result = read_wakatime_cfg_at(Some(&path));
        assert_eq!(result.as_deref(), Some("waka_compat_key"));

        remove(&path);
    }

    #[test]
    fn wakatime_cfg_returns_none_for_absent_file() {
        let path = temp_path("wakatime_absent_xyzzy.cfg");
        remove(&path);
        assert!(read_wakatime_cfg_at(Some(&path)).is_none());
    }

    #[test]
    fn wakatime_cfg_returns_none_when_path_is_none() {
        assert!(read_wakatime_cfg_at(None).is_none());
    }

    #[test]
    fn wakatime_cfg_ignores_key_outside_settings() {
        let path = temp_path("wakatime_sections.cfg");
        remove(&path);

        std::fs::write(
            &path,
            "[other]\napi_key = wrong_key\n[settings]\napi_key = correct_key\n",
        )
        .unwrap();

        let result = read_wakatime_cfg_at(Some(&path));
        assert_eq!(result.as_deref(), Some("correct_key"));

        remove(&path);
    }
}
