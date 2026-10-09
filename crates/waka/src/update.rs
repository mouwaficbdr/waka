//! `waka update`: self-update from GitHub Releases.
//!
//! The release archive is only installed after its SHA-256 digest matches the
//! `SHA256SUMS` file published with the same release. The running binary is
//! then replaced with [`self_replace`], which also works on Windows where a
//! running executable cannot be overwritten in place.

use std::time::Duration;

use anyhow::{bail, Context as _, Result};
use sha2::{Digest as _, Sha256};

use crate::cli::GlobalOpts;
use crate::commands::{check_latest_version, version_is_newer, CURRENT_VERSION};
use crate::spinner::make_spinner;

const RELEASES_URL: &str = "https://github.com/mouwaficbdr/waka/releases";

/// Name of the checksum file attached to every release.
const CHECKSUMS_FILE: &str = "SHA256SUMS";

/// Implements `waka update`.
///
/// Downloads the latest release archive for this platform, verifies it
/// against the release's `SHA256SUMS`, and replaces the current binary.
pub(crate) async fn update_self(global: &GlobalOpts) -> Result<()> {
    // 1. Fetch latest version.
    let pb = make_spinner("Checking for updates…");
    let latest = check_latest_version().await;
    pb.finish_and_clear();
    let latest = match latest {
        Ok(Some(v)) => v,
        Ok(None) => {
            if !global.quiet {
                println!("  ✓  No releases found — you are on the latest build.");
            }
            return Ok(());
        }
        Err(e) => bail!("Failed to check for updates: {e}"),
    };

    // 2. Already up-to-date?
    if !version_is_newer(&latest, CURRENT_VERSION) {
        if !global.quiet {
            println!("  ✓  Already on the latest version (v{latest})");
        }
        return Ok(());
    }

    // 3. Homebrew — defer to brew(1).
    if is_homebrew_install() {
        println!("  ℹ  Detected Homebrew installation. Run:\n\n       brew upgrade waka\n");
        return Ok(());
    }

    if !global.quiet {
        println!("  ⬆  Updating waka v{CURRENT_VERSION} → v{latest}");
    }

    // 4. Resolve platform asset.
    let (target, ext) = platform_target()?;
    let archive_name = format!("waka-v{latest}-{target}.{ext}");
    let base = format!("{RELEASES_URL}/download/v{latest}");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .user_agent(concat!("waka/", env!("CARGO_PKG_VERSION")))
        .build()
        .context("failed to build HTTP client")?;

    // 5. Download the checksums first: without them nothing gets installed.
    let pb = make_spinner(format!("Downloading {archive_name}…"));
    let downloads = async {
        let checksums = download(&client, &format!("{base}/{CHECKSUMS_FILE}"))
            .await
            .with_context(|| {
                format!(
                    "could not download {CHECKSUMS_FILE} for v{latest}; refusing to install \
                     an unverified binary.\nDownload it manually from {RELEASES_URL}"
                )
            })?;
        let archive = download(&client, &format!("{base}/{archive_name}")).await?;
        anyhow::Ok((checksums, archive))
    }
    .await;
    pb.finish_and_clear();
    let (checksums, archive) = downloads?;
    let checksums = String::from_utf8(checksums).context("SHA256SUMS is not valid UTF-8")?;

    // 6. Verify, extract, install.
    let expected = expected_checksum(&checksums, &archive_name).with_context(|| {
        format!("{CHECKSUMS_FILE} for v{latest} has no entry for {archive_name}")
    })?;
    verify_sha256(&archive, &expected, &archive_name)?;

    let pb = make_spinner("Installing new binary…");
    let result = extract_binary(&archive).and_then(|binary| install_binary(&binary));
    pb.finish_and_clear();
    result?;

    if !global.quiet {
        println!("  ✓  waka updated to v{latest} (SHA-256 verified)");
    }
    Ok(())
}

/// GETs `url` and returns the body, failing on non-success statuses.
async fn download(client: &reqwest::Client, url: &str) -> Result<Vec<u8>> {
    let resp = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("request to {url} failed"))?;
    if !resp.status().is_success() {
        bail!("download failed (HTTP {}): {url}", resp.status());
    }
    Ok(resp
        .bytes()
        .await
        .with_context(|| format!("failed to read {url}"))?
        .to_vec())
}

/// Returns the lowercase hex digest listed for `file_name` in a `sha256sum`
/// style checksum file (`<hex>  <name>` or `<hex> *<name>`).
fn expected_checksum(checksums: &str, file_name: &str) -> Option<String> {
    checksums.lines().find_map(|line| {
        let (hex, name) = line.trim().split_once(char::is_whitespace)?;
        let name = name.trim_start().trim_start_matches('*');
        (name == file_name && hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()))
            .then(|| hex.to_ascii_lowercase())
    })
}

/// Fails unless the SHA-256 digest of `bytes` equals `expected_hex`.
fn verify_sha256(bytes: &[u8], expected_hex: &str, name: &str) -> Result<()> {
    let actual = Sha256::digest(bytes)
        .iter()
        .fold(String::with_capacity(64), |mut acc, b| {
            use std::fmt::Write as _;
            let _ = write!(acc, "{b:02x}");
            acc
        });
    if actual != expected_hex {
        bail!(
            "checksum mismatch for {name}: expected {expected_hex}, got {actual}.\n\
             The download may be corrupted or tampered with; nothing was installed."
        );
    }
    Ok(())
}

/// Whether the binary lives under a Homebrew-managed path.
fn is_homebrew_install() -> bool {
    std::env::current_exe().is_ok_and(|exe| {
        let p = exe.to_string_lossy();
        p.contains("/Cellar/") || p.contains("/homebrew/")
    })
}

/// Returns the release asset target triple and archive extension for the
/// current platform, e.g. `("x86_64-unknown-linux-gnu", "tar.gz")`.
fn platform_target() -> Result<(&'static str, &'static str)> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Ok(("x86_64-unknown-linux-gnu", "tar.gz")),
        ("linux", "aarch64") => Ok(("aarch64-unknown-linux-gnu", "tar.gz")),
        ("macos", "x86_64") => Ok(("x86_64-apple-darwin", "tar.gz")),
        ("macos", "aarch64") => Ok(("aarch64-apple-darwin", "tar.gz")),
        ("windows", "x86_64") => Ok(("x86_64-pc-windows-msvc", "zip")),
        (os, arch) => bail!("Unsupported platform {os}/{arch}. Update manually:\n{RELEASES_URL}"),
    }
}

/// Extracts the `waka` binary from a release `.tar.gz` archive.
#[cfg(not(target_os = "windows"))]
fn extract_binary(archive: &[u8]) -> Result<Vec<u8>> {
    use std::io::Read as _;

    let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(archive));
    for entry in tar.entries().context("failed to read tar entries")? {
        let mut entry = entry.context("corrupt tar entry")?;
        let is_waka = entry
            .path()
            .context("invalid tar entry path")?
            .file_name()
            .is_some_and(|n| n == "waka");
        if is_waka && entry.header().entry_type().is_file() {
            let mut binary = Vec::new();
            entry
                .read_to_end(&mut binary)
                .context("failed to read the new binary")?;
            return Ok(binary);
        }
    }
    bail!("Could not find the 'waka' binary in the release archive")
}

/// Extracts `waka.exe` from a release `.zip` archive.
#[cfg(target_os = "windows")]
fn extract_binary(archive: &[u8]) -> Result<Vec<u8>> {
    use std::io::Read as _;

    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(archive))
        .context("failed to open zip archive")?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).context("corrupt zip entry")?;
        let name = file.name().to_owned();
        if file.is_file() && (name == "waka.exe" || name.ends_with("/waka.exe")) {
            let mut binary = Vec::new();
            file.read_to_end(&mut binary)
                .context("failed to read the new binary")?;
            return Ok(binary);
        }
    }
    bail!("Could not find 'waka.exe' in the release archive")
}

/// Writes `binary` next to the running executable and swaps it in.
///
/// [`self_replace::self_replace`] renames over the current executable on Unix
/// and uses the rename-then-schedule-deletion dance on Windows, where a
/// running `.exe` cannot be overwritten.
fn install_binary(binary: &[u8]) -> Result<()> {
    let current = std::env::current_exe().context("cannot determine current executable path")?;
    let dir = current
        .parent()
        .context("current executable has no parent directory")?;
    let staged = dir.join(format!(
        ".waka-update-{}{}",
        std::process::id(),
        std::env::consts::EXE_SUFFIX
    ));

    std::fs::write(&staged, binary).with_context(|| {
        format!(
            "cannot write to {} — run with elevated privileges or reinstall manually",
            dir.display()
        )
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&staged, std::fs::Permissions::from_mode(0o755))
            .context("cannot set executable permission")?;
    }

    let result = self_replace::self_replace(&staged);
    let _ = std::fs::remove_file(&staged);
    result.context("failed to replace the running binary")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUMS: &str = "\
d2a84f4b8b650937ec8f73cd8be2c74add5a911ba64df27458ed8229da804a26  waka-v2.1.0-x86_64-unknown-linux-gnu.tar.gz
0000000000000000000000000000000000000000000000000000000000000000 *waka-v2.1.0-x86_64-pc-windows-msvc.zip
";

    #[test]
    fn expected_checksum_finds_entry_in_both_formats() {
        assert_eq!(
            expected_checksum(SUMS, "waka-v2.1.0-x86_64-unknown-linux-gnu.tar.gz").as_deref(),
            Some("d2a84f4b8b650937ec8f73cd8be2c74add5a911ba64df27458ed8229da804a26")
        );
        assert!(expected_checksum(SUMS, "waka-v2.1.0-x86_64-pc-windows-msvc.zip").is_some());
        assert!(expected_checksum(SUMS, "waka-v2.1.0-aarch64-apple-darwin.tar.gz").is_none());
        // A name that merely contains the asset name must not match.
        assert!(expected_checksum(SUMS, "x86_64-unknown-linux-gnu.tar.gz").is_none());
    }

    #[test]
    fn expected_checksum_rejects_malformed_digests() {
        assert!(expected_checksum("nothex  a.tar.gz\n", "a.tar.gz").is_none());
        assert!(expected_checksum("abcd  a.tar.gz\n", "a.tar.gz").is_none());
    }

    #[test]
    fn verify_sha256_accepts_match_and_rejects_mismatch() {
        // SHA-256("hello world")
        let good = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert!(verify_sha256(b"hello world", good, "x").is_ok());
        let err = verify_sha256(b"hello world!", good, "x").unwrap_err();
        assert!(err.to_string().contains("checksum mismatch"), "{err}");
    }

    #[cfg(not(target_os = "windows"))]
    fn tar_gz(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut builder = tar::Builder::new(flate2::write::GzEncoder::new(
            Vec::new(),
            flate2::Compression::fast(),
        ));
        for (path, data) in entries {
            let mut header = tar::Header::new_gnu();
            header.set_size(data.len() as u64);
            header.set_mode(0o755);
            header.set_cksum();
            builder.append_data(&mut header, path, *data).unwrap();
        }
        builder.into_inner().unwrap().finish().unwrap()
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn extract_binary_finds_waka_in_archive() {
        let archive = tar_gz(&[("README.md", b"docs"), ("dist/waka", b"\x7fELF binary")]);
        assert_eq!(extract_binary(&archive).unwrap(), b"\x7fELF binary");
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn extract_binary_errors_without_waka() {
        let archive = tar_gz(&[("waka-other", b"x")]);
        assert!(extract_binary(&archive).is_err());
    }
}
