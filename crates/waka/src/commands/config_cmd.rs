//! `waka config`: get/set/edit/path/reset and the `doctor` diagnostic.

use super::*;
use crate::update::{check_latest_version, version_is_newer};

// ─── config ───────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd is consumed by the match for exhaustive checking.
#[allow(clippy::needless_pass_by_value)]
pub(super) async fn config(cmd: ConfigCommands, global: &GlobalOpts) -> Result<()> {
    match cmd {
        ConfigCommands::Get { key } => {
            let config = load_config()?;
            match key {
                Some(key) => println!("{}", config.get_key(&key)?),
                None => print!("{}", config.to_toml_string()?),
            }
            Ok(())
        }
        ConfigCommands::Set { key, value } => {
            let mut config = load_config()?;
            config.set_key(&key, &value)?;
            config.save().context("failed to save config")?;
            if !global.quiet {
                println!("✓ {key} = {}", config.get_key(&key)?);
            }
            Ok(())
        }
        ConfigCommands::Edit => config_edit(),
        ConfigCommands::Path => {
            println!("{}", Config::path()?.display());
            Ok(())
        }
        ConfigCommands::Reset { confirm } => config_reset(confirm, global),
        ConfigCommands::Doctor => config_doctor(global).await,
    }
}

/// Opens `config.toml` in `$VISUAL` / `$EDITOR` (falling back to `vi`, or
/// `notepad` on Windows), creating it with defaults first if needed, then
/// re-validates the file.
fn config_edit() -> Result<()> {
    let path = Config::path()?;
    if !path.exists() {
        Config::default()
            .save()
            .context("failed to create default config")?;
    }

    let editor = std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or_else(|_| if cfg!(windows) { "notepad" } else { "vi" }.to_owned());
    // Allow editors with arguments, e.g. EDITOR="code --wait".
    let mut parts = editor.split_whitespace();
    let program = parts.next().context("$EDITOR is empty")?;
    let status = std::process::Command::new(program)
        .args(parts)
        .arg(&path)
        .status()
        .with_context(|| format!("failed to launch editor '{editor}'"))?;
    if !status.success() {
        bail!("editor '{editor}' exited with {status}");
    }

    if let Err(e) = Config::load() {
        eprintln!(
            "warning: {} is not valid anymore: {e}\n  Run `waka config edit` again to fix it.",
            path.display()
        );
    }
    Ok(())
}

/// Implements `waka config reset`: overwrites `config.toml` with defaults
/// after confirmation (skipped with `--confirm`).
fn config_reset(confirm: bool, global: &GlobalOpts) -> Result<()> {
    let path = Config::path()?;
    if !confirm {
        if !std::io::stdin().is_terminal() {
            bail!(
                "refusing to reset {} without confirmation; pass --confirm",
                path.display()
            );
        }
        let proceed = inquire::Confirm::new(&format!(
            "Reset {} to defaults? This cannot be undone.",
            path.display()
        ))
        .with_default(false)
        .prompt()
        .context("failed to read confirmation")?;
        if !proceed {
            println!("Aborted.");
            return Ok(());
        }
    }
    Config::default().save().context("failed to save config")?;
    if !global.quiet {
        println!("✓ Config reset to defaults ({})", path.display());
    }
    Ok(())
}

/// Runs a full diagnostic check and prints a human-readable report.
///
/// Checks performed (in order):
/// 1. Config file found at the platform-specific path
/// 2. API key present in the credential priority chain
/// 3. API key valid (calls `/users/current`)
/// 4. API reachable (measures round-trip time)
/// 5. Cache directory writable
/// 6. Shell completions installed for the active shell
/// 7. Update check (compares version with GitHub Releases)
// The function has many sequential checks that are intentionally laid out in order.
#[allow(clippy::too_many_lines)]
async fn config_doctor(global: &GlobalOpts) -> Result<()> {
    let use_color = !global.no_color && should_use_color();
    // A broken config is reported as an issue below; diagnose with defaults.
    let config_result = waka_config::Config::load();
    let config = config_result.as_ref().cloned().unwrap_or_default();
    let profile = resolve_profile(global, &config);
    let profile = profile.as_str();
    let mut issues: u32 = 0;
    let mut warnings: u32 = 0;

    // ── colour helpers ───────────────────────────────────────────────────────
    let ok_mark = if use_color {
        console::style("✓").green().to_string()
    } else {
        "✓".to_owned()
    };
    let warn_mark = if use_color {
        console::style("⚠").yellow().to_string()
    } else {
        "⚠".to_owned()
    };
    let fail_mark = if use_color {
        console::style("✗").red().to_string()
    } else {
        "✗".to_owned()
    };

    // ── 1. Config file ───────────────────────────────────────────────────────
    match waka_config::Config::path() {
        Ok(path) => {
            if let Err(e) = &config_result {
                println!(
                    "  {fail_mark}  Config file at {} is invalid: {e}",
                    path.display()
                );
                issues += 1;
            } else if path.exists() {
                println!("  {ok_mark}  Config file found at {}", path.display());
            } else {
                println!(
                    "  {warn_mark}  Config file not found at {} (using defaults)",
                    path.display()
                );
                warnings += 1;
            }
        }
        Err(e) => {
            println!("  {fail_mark}  Could not determine config path: {e}");
            issues += 1;
        }
    }

    // ── 2. API key ───────────────────────────────────────────────────────────
    let store = waka_config::CredentialStore::new(profile);
    let api_key_result = store.get_api_key();
    let api_key = if let Ok(key) = &api_key_result {
        println!("  {ok_mark}  API key found in credential store");
        Some(key.expose().to_owned())
    } else {
        println!("  {fail_mark}  No API key found — run `waka auth login` to authenticate");
        issues += 1;
        None
    };

    // ── 3 & 4. API key valid + reachability ──────────────────────────────────
    if let Some(key) = api_key {
        let api_url_normalized = profile_api_url(&config, profile);
        match waka_api::WakaClient::with_base_url(&key, &api_url_normalized) {
            Err(e) => {
                println!("  {fail_mark}  Could not build API client: {e}");
                issues += 1;
            }
            Ok(client) => {
                let t0 = std::time::Instant::now();
                match client.me().await {
                    Ok(user_resp) => {
                        let elapsed_ms = t0.elapsed().as_millis();
                        let identity = user_resp.email.as_deref().unwrap_or(&user_resp.username);
                        println!("  {ok_mark}  API key is valid (authenticated as {identity})");
                        println!("  {ok_mark}  API reachable (ping: {elapsed_ms}ms)");
                    }
                    Err(waka_api::ApiError::Unauthorized) => {
                        let elapsed_ms = t0.elapsed().as_millis();
                        println!(
                            "  {fail_mark}  API key is invalid or expired — run `waka auth login`"
                        );
                        // API IS reachable even if unauthorized.
                        println!("  {ok_mark}  API reachable (ping: {elapsed_ms}ms)");
                        issues += 1;
                    }
                    Err(e) => {
                        println!("  {fail_mark}  API unreachable: {e}");
                        println!("  {fail_mark}  Could not validate API key (network error)");
                        issues += 1;
                    }
                }
            }
        }
    } else {
        // Skip reachability if we have no key.
        println!("  {warn_mark}  API reachability check skipped (no API key)");
        warnings += 1;
    }

    // ── 5. Cache directory ───────────────────────────────────────────────────
    match waka_config::Config::cache_dir() {
        Ok(cache_path) => {
            // Try to create the directory if it does not exist.
            if let Err(e) = std::fs::create_dir_all(&cache_path) {
                println!(
                    "  {fail_mark}  Cache directory not writable at {}: {e}",
                    cache_path.display()
                );
                issues += 1;
            } else {
                // Quick writability probe: create then remove a temp file.
                let probe = cache_path.join(".waka_write_probe");
                match std::fs::write(&probe, b"") {
                    Ok(()) => {
                        let _ = std::fs::remove_file(&probe);
                        println!(
                            "  {ok_mark}  Cache directory writable at {}",
                            cache_path.display()
                        );
                    }
                    Err(e) => {
                        println!(
                            "  {fail_mark}  Cache directory not writable at {}: {e}",
                            cache_path.display()
                        );
                        issues += 1;
                    }
                }
            }
        }
        Err(e) => {
            println!("  {fail_mark}  Could not determine cache directory: {e}");
            issues += 1;
        }
    }

    // ── 6. Shell completions ─────────────────────────────────────────────────
    let detected_shell = std::env::var("SHELL").ok().and_then(|s| {
        std::path::Path::new(&s)
            .file_name()
            .and_then(|n| n.to_str())
            .map(std::string::ToString::to_string)
    });

    match detected_shell.as_deref() {
        Some("zsh") => {
            let zfunc = dirs_check_zsh_completions();
            if zfunc {
                println!("  {ok_mark}  Shell completions installed (zsh)");
            } else {
                println!(
                    "  {warn_mark}  Shell completions not found for zsh — run `waka completions zsh`"
                );
                warnings += 1;
            }
        }
        Some("bash") => {
            let found = dirs_check_bash_completions();
            if found {
                println!("  {ok_mark}  Shell completions installed (bash)");
            } else {
                println!(
                    "  {warn_mark}  Shell completions not found for bash — run `waka completions bash`"
                );
                warnings += 1;
            }
        }
        Some("fish") => {
            let fish_path = directories::BaseDirs::new()
                .map(|d| d.home_dir().join(".config/fish/completions/waka.fish"));
            if fish_path.as_ref().is_some_and(|p| p.exists()) {
                println!("  {ok_mark}  Shell completions installed (fish)");
            } else {
                println!(
                    "  {warn_mark}  Shell completions not found for fish — run `waka completions fish`"
                );
                warnings += 1;
            }
        }
        Some(shell) => {
            println!("  {warn_mark}  Shell completions check skipped (unsupported shell: {shell})");
            warnings += 1;
        }
        None => {
            println!("  {warn_mark}  Shell completions check skipped ($SHELL not set)");
            warnings += 1;
        }
    }

    // ── 7. Version / update check ────────────────────────────────────────────
    let current = env!("CARGO_PKG_VERSION");
    match check_latest_version().await {
        Ok(Some(latest)) if latest != current && version_is_newer(&latest, current) => {
            println!(
                "  {warn_mark}  waka v{current} installed — v{latest} available (run: waka update)"
            );
            warnings += 1;
        }
        Ok(_) => {
            println!("  {ok_mark}  waka v{current} is up to date");
        }
        Err(_) => {
            // Update check failure is non-critical — silently skip.
            println!("  {warn_mark}  Could not check for updates (network unavailable?)");
            warnings += 1;
        }
    }

    // ── Summary ──────────────────────────────────────────────────────────────
    if issues == 0 && warnings == 0 {
        println!("  {ok_mark}  No known issues");
    } else if issues > 0 {
        let label = if issues == 1 { "issue" } else { "issues" };
        println!("  {fail_mark}  {issues} {label} found — check the output above");
    }

    Ok(())
}

/// Returns `true` if the zsh completion file exists in a standard `$fpath`
/// location.
fn dirs_check_zsh_completions() -> bool {
    // Common user-level locations
    let Some(base) = directories::BaseDirs::new() else {
        return false;
    };
    let home = base.home_dir();
    let candidates = [
        home.join(".zfunc/_waka"),
        home.join(".zfunc/waka.zsh"),
        home.join(".local/share/zsh/site-functions/_waka"),
        std::path::PathBuf::from("/usr/local/share/zsh/site-functions/_waka"),
        std::path::PathBuf::from("/usr/share/zsh/site-functions/_waka"),
    ];
    candidates.iter().any(|p| p.exists())
}

/// Returns `true` if the bash completion file can be found.
fn dirs_check_bash_completions() -> bool {
    let Some(base) = directories::BaseDirs::new() else {
        return false;
    };
    let home = base.home_dir();
    let candidates = [
        home.join(".local/share/bash-completion/completions/waka"),
        home.join(".bash_completion.d/waka"),
        std::path::PathBuf::from("/usr/local/share/bash-completion/completions/waka"),
        std::path::PathBuf::from("/usr/share/bash-completion/completions/waka"),
    ];
    candidates.iter().any(|p| p.exists())
}
