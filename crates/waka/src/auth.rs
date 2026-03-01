//! Handlers for `waka auth` subcommands.
//!
//! Implements login (interactive + non-interactive), logout, status, and
//! show-key. All credential I/O goes through [`CredentialStore`]; all
//! network I/O goes through [`WakaClient`].

use std::time::Duration;

use anyhow::{bail, Context as _, Result};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use waka_api::WakaClient;
use waka_config::{CredentialError, CredentialStore};

use crate::cli::{AuthLoginArgs, GlobalOpts};

// ─── Public handlers ──────────────────────────────────────────────────────────

/// Implements `waka auth login`.
///
/// Interactive mode: prompts for the API key without echo.
/// Non-interactive mode: uses the value supplied to `--api-key`.
///
/// In both modes the key is validated against the `WakaTime` API before being
/// saved to the system keychain (or the credentials file as fallback).
pub async fn login(args: AuthLoginArgs, global: &GlobalOpts) -> Result<()> {
    let profile = profile_name(args.profile.as_deref(), global);

    let api_key = if let Some(key) = args.api_key {
        // Non-interactive path.
        if key.trim().is_empty() {
            bail!("--api-key cannot be empty");
        }
        key.trim().to_owned()
    } else {
        // Interactive path — warn if a key is already stored for this profile.
        if !global.quiet {
            let store = CredentialStore::new(&profile);
            match store.get_api_key() {
                Ok(_) => eprintln!(
                    "Warning: a key is already stored for profile '{profile}'.\n\
                     Continue to replace it, or press Ctrl-C to abort."
                ),
                // Expected when no key exists yet — nothing to warn about.
                Err(CredentialError::NotFound) => {}
                Err(e) => eprintln!("Warning: could not check existing credentials: {e}"),
            }
        }

        // Read the API key from stdin without echoing it.
        eprint!("WakaTime API key: ");
        let key = rpassword::read_password().context("failed to read API key from terminal")?;
        if key.trim().is_empty() {
            bail!("API key cannot be empty");
        }
        key.trim().to_owned()
    };

    // Validate the key against the API before saving it.
    let pb = make_spinner("Validating API key …");
    let result = WakaClient::new(&api_key).me().await;
    pb.finish_and_clear();

    let user = result.with_context(|| {
        "API key validation failed.\n\
         Check that the key is correct at https://wakatime.com/settings/account"
    })?;

    // Save to keychain; fall back to the credentials file when unavailable.
    let store = CredentialStore::new(&profile);
    match store.set_api_key(&api_key) {
        Ok(()) => {}
        Err(CredentialError::Keychain(ref e)) => {
            eprintln!(
                "Warning: could not write to system keychain ({e}).\n\
                 Saving to credentials file instead."
            );
            store
                .save_to_credentials_file(&api_key)
                .context("failed to save API key to credentials file")?;
        }
        Err(e) => return Err(e).context("failed to save API key"),
    }

    println!("✓ Logged in as {} (@{})", user.display_name, user.username);
    Ok(())
}

/// Implements `waka auth logout`.
///
/// Removes the stored API key for the active profile from the keychain.
// `unused_async`: logout has no async I/O today; kept async so the
// call-site signature in commands.rs is uniform across all auth handlers.
#[allow(clippy::unused_async)]
pub async fn logout(profile: Option<String>, global: &GlobalOpts) -> Result<()> {
    let profile = profile_name(profile.as_deref(), global);
    let store = CredentialStore::new(&profile);

    store
        .delete_api_key()
        .with_context(|| format!("failed to remove API key for profile '{profile}'"))?;

    if !global.quiet {
        println!("✓ Logged out (profile: {profile})");
    }

    Ok(())
}

/// Implements `waka auth status`.
///
/// Reports whether the active profile has a valid API key, showing the
/// username if authenticated. Never displays the raw key.
pub async fn status(global: &GlobalOpts) -> Result<()> {
    let profile = profile_name(None, global);
    let store = CredentialStore::new(&profile);

    match store.get_api_key() {
        Ok(key) => {
            // Verify the stored key is still accepted by the API.
            let pb = make_spinner("Checking WakaTime connection …");
            let result = WakaClient::new(key.expose()).me().await;
            pb.finish_and_clear();

            match result {
                Ok(user) => {
                    println!(
                        "✓ Logged in as {} (@{}) [profile: {profile}]",
                        user.display_name, user.username
                    );
                }
                Err(e) => {
                    // Key exists but the API rejected it.
                    println!("✗ API key found but validation failed: {e}");
                    println!("  Run `waka auth login` to update your key.");
                }
            }
        }
        Err(CredentialError::NotFound) => {
            println!("✗ Not logged in");
            println!("  Run `waka auth login` to authenticate.");
        }
        Err(e) => {
            bail!("could not read credentials: {e}");
        }
    }

    Ok(())
}

/// Implements `waka auth show-key`.
///
/// Prints the stored API key in masked form. The raw key is **never** printed.
// `unused_async`: show_key has no async I/O; kept async for handler uniformity.
#[allow(clippy::unused_async)]
pub async fn show_key(global: &GlobalOpts) -> Result<()> {
    let profile = profile_name(None, global);
    let store = CredentialStore::new(&profile);

    match store.get_api_key() {
        Ok(key) => {
            println!("{}", mask_key(key.expose()));
            Ok(())
        }
        Err(CredentialError::NotFound) => {
            bail!(
                "No API key found for profile '{profile}'.\n\
                 Run `waka auth login` to authenticate."
            );
        }
        Err(e) => {
            bail!("could not read credentials: {e}");
        }
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Returns the effective profile name using the priority: explicit CLI arg >
/// global `--profile` flag > `"default"`.
fn profile_name(explicit: Option<&str>, global: &GlobalOpts) -> String {
    explicit
        .or(global.profile.as_deref())
        .unwrap_or("default")
        .to_owned()
}

/// Returns a masked representation of the API key safe for display.
///
/// Shows the first 5 characters and last 4 characters separated by `****`.
/// If the key is too short to mask safely the entire value is replaced with
/// `[REDACTED]`.
///
/// # Examples
///
/// ```text
/// "waka_1234567890abcdef1234567890abcdef12345678" → "waka_****5678"
/// "short" → "[REDACTED]"
/// ```
fn mask_key(key: &str) -> String {
    const SHOW_PREFIX: usize = 5;
    const SHOW_SUFFIX: usize = 4;

    if key.len() <= SHOW_PREFIX + SHOW_SUFFIX {
        return "[REDACTED]".to_owned();
    }

    let prefix = &key[..SHOW_PREFIX];
    let suffix = &key[key.len() - SHOW_SUFFIX..];
    format!("{prefix}****{suffix}")
}

/// Creates a spinner that writes to stderr.
///
/// `indicatif` automatically hides the spinner when stderr is not a TTY, so
/// callers do not need to branch on terminal detection.
fn make_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr());
    pb.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_spinner()),
    );
    pb.set_message(message.to_owned());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn default_global() -> GlobalOpts {
        GlobalOpts {
            profile: None,
            format: None,
            no_cache: false,
            no_color: false,
            quiet: false,
            verbose: false,
            csv_bom: false,
        }
    }

    // ── profile_name ─────────────────────────────────────────────────────────

    #[test]
    fn profile_name_explicit_takes_priority() {
        let global = GlobalOpts {
            profile: Some("global_prof".to_owned()),
            ..default_global()
        };
        assert_eq!(profile_name(Some("cli_prof"), &global), "cli_prof");
    }

    #[test]
    fn profile_name_falls_back_to_global_flag() {
        let global = GlobalOpts {
            profile: Some("my_profile".to_owned()),
            ..default_global()
        };
        assert_eq!(profile_name(None, &global), "my_profile");
    }

    #[test]
    fn profile_name_defaults_to_default() {
        let global = default_global();
        assert_eq!(profile_name(None, &global), "default");
    }

    // ── mask_key ─────────────────────────────────────────────────────────────

    #[test]
    fn mask_key_typical_wakatime_key() {
        let key = "waka_1234567890abcdef1234567890abcdef12345678";
        let masked = mask_key(key);
        assert!(masked.starts_with("waka_"), "should preserve prefix");
        assert!(masked.ends_with("5678"), "should preserve suffix");
        assert!(masked.contains("****"), "should contain mask marker");
        // The middle portion of the key must not appear.
        assert!(
            !masked.contains("1234567890abcdef"),
            "middle must be hidden"
        );
    }

    #[test]
    fn mask_key_short_key_is_fully_redacted() {
        assert_eq!(mask_key("short"), "[REDACTED]");
    }

    #[test]
    fn mask_key_exactly_at_boundary_is_redacted() {
        // 9 chars = SHOW_PREFIX (5) + SHOW_SUFFIX (4) — not strictly greater.
        let key = "123456789";
        assert_eq!(mask_key(key), "[REDACTED]");
    }

    #[test]
    fn mask_key_one_over_boundary_produces_mask() {
        // 10 chars = 5 + 1 + 4 — just enough to add a masked middle.
        let masked = mask_key("1234567890");
        assert_eq!(masked, "12345****7890");
    }
}
