//! Local cache abstraction for `waka`.
//!
//! A typed, TTL-aware key-value store used by the `waka` CLI to cache
//! `WakaTime` API responses and reduce network calls.
//!
//! Each entry is a small JSON file written atomically (temporary file +
//! rename), so any number of `waka` processes can read and write the cache at
//! the same time without locks: a reader always sees either the previous or
//! the new version of an entry.
//!
//! # Example
//!
//! ```rust,no_run
//! use std::time::Duration;
//! use waka_cache::CacheStore;
//!
//! # fn main() -> Result<(), waka_cache::CacheError> {
//! let store = CacheStore::open("default")?;
//! store.set("summaries:today", &"some data", Duration::from_secs(300))?;
//! if let Some(entry) = store.get::<String>("summaries:today")? {
//!     println!("cached {} ago: {}", entry.age_human(), entry.value);
//! }
//! # Ok(())
//! # }
//! ```

#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::warn;

// ─── Error type ───────────────────────────────────────────────────────────────

/// Errors returned by [`CacheStore`].
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    /// The platform cache directory could not be determined.
    #[error("could not determine cache directory")]
    NoCacheDir,

    /// The cache directory could not be created.
    #[error("failed to open cache directory at {path}: {source}")]
    DbOpen {
        /// Directory that could not be created.
        path: PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },

    /// An entry could not be serialized.
    #[error("cache serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// A filesystem operation failed.
    #[error("cache I/O error: {0}")]
    Io(#[from] std::io::Error),
}

// ─── CacheEntry ───────────────────────────────────────────────────────────────

/// Stored representation of a cached value with provenance metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    /// The cached value.
    pub value: T,
    /// UTC timestamp when this entry was inserted.
    pub inserted_at: DateTime<Utc>,
    /// Time-to-live for this entry.
    #[serde(with = "duration_serde")]
    pub ttl: Duration,
}

impl<T> CacheEntry<T> {
    /// Returns `true` if this entry has exceeded its TTL.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        let age = Utc::now()
            .signed_duration_since(self.inserted_at)
            .to_std()
            // If the duration is negative (clock skew), treat as not expired.
            .unwrap_or(Duration::ZERO);
        age > self.ttl
    }

    /// Returns a human-readable string describing the age of this entry.
    ///
    /// Format: `"3s ago"`, `"4m ago"`, `"2h ago"`, `"1d ago"`.
    #[must_use]
    pub fn age_human(&self) -> String {
        let age = Utc::now()
            .signed_duration_since(self.inserted_at)
            .to_std()
            .unwrap_or(Duration::ZERO);

        let secs = age.as_secs();
        if secs < 60 {
            format!("{secs}s ago")
        } else if secs < 3_600 {
            format!("{}m ago", secs / 60)
        } else if secs < 86_400 {
            format!("{}h ago", secs / 3_600)
        } else {
            format!("{}d ago", secs / 86_400)
        }
    }
}

// ─── CacheInfo ────────────────────────────────────────────────────────────────

/// Summary statistics about the cache store.
#[derive(Debug, Clone)]
pub struct CacheInfo {
    /// Total number of entries currently in the database.
    pub entry_count: usize,
    /// Approximate size of the database on disk, in bytes.
    pub size_on_disk: u64,
    /// UTC timestamp of the most recent write, if any.
    pub last_write: Option<DateTime<Utc>>,
}

// ─── CacheStore ───────────────────────────────────────────────────────────────

/// A per-profile cache stored as one JSON file per key.
#[derive(Debug, Clone)]
pub struct CacheStore {
    /// Directory holding the entry files.
    entries: PathBuf,
    /// Profile cache root (used for [`CacheInfo::size_on_disk`]).
    path: PathBuf,
}

/// Extension of entry files; temporary files use another one so they are
/// never mistaken for entries.
const ENTRY_EXT: &str = "json";

impl CacheStore {
    /// Opens (or creates) the cache store for the named profile.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::NoCacheDir`] if the platform cache directory
    /// cannot be determined, or [`CacheError::DbOpen`] if the directory
    /// cannot be created.
    pub fn open(profile: &str) -> Result<Self, CacheError> {
        Self::open_at(Self::db_path(profile)?)
    }

    /// Opens (or creates) a cache store rooted at an explicit directory.
    ///
    /// Data left there by the previous sled-based cache is removed.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::DbOpen`] if the directory cannot be created.
    pub fn open_at(path: PathBuf) -> Result<Self, CacheError> {
        let entries = path.join("entries");
        std::fs::create_dir_all(&entries).map_err(|source| CacheError::DbOpen {
            path: entries.clone(),
            source,
        })?;
        remove_legacy_sled_files(&path);
        Ok(Self { entries, path })
    }

    /// Returns the cache directory used for the named profile.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::NoCacheDir`] if the platform directories cannot
    /// be determined.
    pub fn db_path(profile: &str) -> Result<PathBuf, CacheError> {
        let dirs = ProjectDirs::from("", "", "waka").ok_or(CacheError::NoCacheDir)?;
        Ok(dirs.cache_dir().join(sanitize_profile(profile)))
    }

    /// Retrieves a cached value by key.
    ///
    /// Returns `Ok(None)` on a cache miss, or on a corrupted entry (which is
    /// removed with a warning).
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::Io`] if the entry exists but cannot be read.
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<CacheEntry<T>>, CacheError> {
        let path = self.entry_path(key);
        let bytes = match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        match serde_json::from_slice::<CacheEntry<T>>(&bytes) {
            Ok(entry) => Ok(Some(entry)),
            Err(err) => {
                warn!(key, %err, "cache entry is corrupted — dropping");
                let _ = std::fs::remove_file(&path);
                Ok(None)
            }
        }
    }

    /// Stores a value under `key` with the given TTL.
    ///
    /// The entry is written to a temporary file and renamed into place, so
    /// concurrent readers never observe a partially written entry.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::Serde`] if serialization fails, or
    /// [`CacheError::Io`] if the file cannot be written.
    pub fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), CacheError> {
        let entry = CacheEntry {
            value,
            inserted_at: Utc::now(),
            ttl,
        };
        let bytes = serde_json::to_vec(&entry)?;

        let path = self.entry_path(key);
        let tmp = path.with_extension(format!(
            "tmp-{}-{}",
            std::process::id(),
            Utc::now().timestamp_nanos_opt().unwrap_or_default()
        ));
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, &path).inspect_err(|_| {
            let _ = std::fs::remove_file(&tmp);
        })?;
        Ok(())
    }

    /// Removes all entries from the cache.
    ///
    /// Returns the number of entries that were removed.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::Io`] if the cache directory cannot be read.
    pub fn clear(&self) -> Result<usize, CacheError> {
        let mut removed = 0;
        for path in self.entry_files()? {
            if std::fs::remove_file(path).is_ok() {
                removed += 1;
            }
        }
        Ok(removed)
    }

    /// Removes entries that were inserted more than `older_than` ago.
    ///
    /// Returns the number of entries removed. Unreadable entries are removed
    /// as well.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::Io`] if the cache directory cannot be read.
    pub fn clear_older_than(&self, older_than: Duration) -> Result<usize, CacheError> {
        let cutoff =
            Utc::now() - chrono::Duration::from_std(older_than).unwrap_or(chrono::Duration::zero());

        let mut removed = 0;
        for path in self.entry_files()? {
            let stale = read_inserted_at(&path).is_none_or(|inserted_at| inserted_at < cutoff);
            if stale && std::fs::remove_file(&path).is_ok() {
                removed += 1;
            }
        }
        Ok(removed)
    }

    /// Returns statistics about the cache (entry count, disk usage, last write).
    #[must_use]
    pub fn info(&self) -> CacheInfo {
        let files = self.entry_files().unwrap_or_default();
        CacheInfo {
            entry_count: files.len(),
            size_on_disk: dir_size_bytes(&self.path),
            last_write: files.iter().filter_map(|p| read_inserted_at(p)).max(),
        }
    }

    /// Path of the file that stores `key`.
    fn entry_path(&self, key: &str) -> PathBuf {
        self.entries.join(entry_file_name(key))
    }

    /// All entry files currently in the store.
    fn entry_files(&self) -> Result<Vec<PathBuf>, CacheError> {
        Ok(std::fs::read_dir(&self.entries)?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == ENTRY_EXT))
            .collect())
    }
}

// ─── helpers ──────────────────────────────────────────────────────────────────

/// Maps a cache key to a file name that is valid on every platform.
///
/// Characters outside `[A-Za-z0-9._-]` are percent-encoded, which keeps the
/// mapping injective. Long keys are truncated and suffixed with a stable
/// FNV-1a hash of the full key to stay well within file-name length limits.
fn entry_file_name(key: &str) -> String {
    use std::fmt::Write as _;

    const MAX_STEM: usize = 150;
    let mut stem = String::with_capacity(key.len());
    for b in key.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.') {
            stem.push(char::from(b));
        } else {
            let _ = write!(stem, "%{b:02X}");
        }
    }
    if stem.len() > MAX_STEM {
        let hash = key.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
        });
        stem.truncate(MAX_STEM - 17);
        let _ = write!(stem, "-{hash:016x}");
    }
    format!("{stem}.{ENTRY_EXT}")
}

/// Replaces characters outside `[A-Za-z0-9_-]` so a profile name can never
/// escape the cache directory (e.g. `--profile ../../x`).
fn sanitize_profile(profile: &str) -> String {
    profile
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Reads only the `inserted_at` field of an entry file.
fn read_inserted_at(path: &Path) -> Option<DateTime<Utc>> {
    #[derive(Deserialize)]
    struct Envelope {
        inserted_at: DateTime<Utc>,
    }
    let bytes = std::fs::read(path).ok()?;
    serde_json::from_slice::<Envelope>(&bytes)
        .ok()
        .map(|e| e.inserted_at)
}

/// Best-effort removal of the files the previous sled-based cache kept in the
/// profile directory (`db`, `conf`, `snap.*`, `blobs/`).
fn remove_legacy_sled_files(root: &Path) {
    let _ = std::fs::remove_file(root.join("db"));
    let _ = std::fs::remove_file(root.join("conf"));
    let _ = std::fs::remove_dir_all(root.join("blobs"));
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.filter_map(Result::ok) {
            if entry.file_name().to_string_lossy().starts_with("snap.") {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
}

fn dir_size_bytes(dir: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    entries
        .filter_map(Result::ok)
        .map(|e| {
            let meta = e.metadata().ok();
            if meta.as_ref().is_some_and(std::fs::Metadata::is_dir) {
                dir_size_bytes(&e.path())
            } else {
                meta.map_or(0, |m| m.len())
            }
        })
        .sum()
}

// ─── Duration (de)serialisation ───────────────────────────────────────────────

mod duration_serde {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u64(d.as_secs())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
        let secs = u64::deserialize(d)?;
        Ok(Duration::from_secs(secs))
    }
}

// ─── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    /// A store in a fresh temporary directory, removed when dropped.
    struct TempStore {
        store: CacheStore,
        root: PathBuf,
    }

    impl std::ops::Deref for TempStore {
        type Target = CacheStore;
        fn deref(&self) -> &CacheStore {
            &self.store
        }
    }

    impl Drop for TempStore {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn temp_store() -> TempStore {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("waka-cache-test-{}-{n}", std::process::id()));
        let store = CacheStore::open_at(root.clone()).expect("open temp store");
        TempStore { store, root }
    }

    // ── CacheEntry::is_expired ────────────────────────────────────────────────

    #[test]
    fn entry_not_expired_when_fresh() {
        let entry = CacheEntry {
            value: "hello",
            inserted_at: Utc::now(),
            ttl: Duration::from_secs(300),
        };
        assert!(!entry.is_expired());
    }

    #[test]
    fn entry_expired_when_ttl_exceeded() {
        let entry = CacheEntry {
            value: "hello",
            inserted_at: Utc::now() - chrono::Duration::seconds(400),
            ttl: Duration::from_secs(300),
        };
        assert!(entry.is_expired());
    }

    // ── CacheEntry::age_human ─────────────────────────────────────────────────

    #[test]
    fn age_human_seconds() {
        let entry = CacheEntry {
            value: (),
            inserted_at: Utc::now() - chrono::Duration::seconds(30),
            ttl: Duration::from_secs(300),
        };
        assert!(
            entry.age_human().ends_with("s ago"),
            "{}",
            entry.age_human()
        );
    }

    #[test]
    fn age_human_minutes() {
        let entry = CacheEntry {
            value: (),
            inserted_at: Utc::now() - chrono::Duration::seconds(90),
            ttl: Duration::from_secs(300),
        };
        assert!(
            entry.age_human().ends_with("m ago"),
            "{}",
            entry.age_human()
        );
    }

    #[test]
    fn age_human_hours() {
        let entry = CacheEntry {
            value: (),
            inserted_at: Utc::now() - chrono::Duration::hours(3),
            ttl: Duration::from_secs(300),
        };
        assert!(
            entry.age_human().ends_with("h ago"),
            "{}",
            entry.age_human()
        );
    }

    #[test]
    fn age_human_days() {
        let entry = CacheEntry {
            value: (),
            inserted_at: Utc::now() - chrono::Duration::days(2),
            ttl: Duration::from_secs(300),
        };
        assert!(
            entry.age_human().ends_with("d ago"),
            "{}",
            entry.age_human()
        );
    }

    // ── CacheStore get/set ────────────────────────────────────────────────────

    #[test]
    fn get_returns_none_for_missing_key() {
        let store = temp_store();
        let result = store
            .get::<String>("nonexistent")
            .expect("get must succeed");
        assert!(result.is_none());
    }

    #[test]
    fn set_then_get_roundtrip() {
        let store = temp_store();
        store
            .set("key1", &"hello world", Duration::from_secs(60))
            .expect("set must succeed");
        let entry = store
            .get::<String>("key1")
            .expect("get must succeed")
            .expect("entry must exist");
        assert_eq!(entry.value, "hello world");
        assert!(!entry.is_expired());
    }

    #[test]
    fn set_overwrites_existing_entry() {
        let store = temp_store();
        store.set("k", &"first", Duration::from_secs(60)).unwrap();
        store.set("k", &"second", Duration::from_secs(60)).unwrap();
        let entry = store.get::<String>("k").unwrap().unwrap();
        assert_eq!(entry.value, "second");
    }

    #[test]
    fn get_returns_none_for_corrupted_entry() {
        let store = temp_store();
        // Write raw garbage bytes directly.
        std::fs::write(store.entry_path("bad"), b"not valid json at all").unwrap();
        let result = store.get::<String>("bad").expect("get must not error");
        assert!(result.is_none(), "corrupted entry should return None");
        // Entry should have been removed.
        assert!(
            !store.entry_path("bad").exists(),
            "corrupted entry must be cleaned up"
        );
    }

    // ── clear ─────────────────────────────────────────────────────────────────

    #[test]
    fn clear_removes_all_entries_and_returns_count() {
        let store = temp_store();
        store.set("a", &1u32, Duration::from_secs(60)).unwrap();
        store.set("b", &2u32, Duration::from_secs(60)).unwrap();
        store.set("c", &3u32, Duration::from_secs(60)).unwrap();
        let removed = store.clear().expect("clear must succeed");
        assert_eq!(removed, 3);
        assert_eq!(store.info().entry_count, 0);
    }

    #[test]
    fn clear_on_empty_store_returns_zero() {
        let store = temp_store();
        let removed = store.clear().expect("clear must succeed");
        assert_eq!(removed, 0);
    }

    // ── clear_older_than ──────────────────────────────────────────────────────

    #[test]
    fn clear_older_than_removes_old_and_keeps_fresh() {
        let store = temp_store();

        // Insert a fresh entry.
        store.set("fresh", &"ok", Duration::from_secs(300)).unwrap();

        // Insert a stale entry by writing directly with an old timestamp.
        let old_entry = CacheEntry {
            value: "old",
            inserted_at: Utc::now() - chrono::Duration::hours(2),
            ttl: Duration::from_secs(300),
        };
        let bytes = serde_json::to_vec(&old_entry).unwrap();
        std::fs::write(store.entry_path("stale"), bytes).unwrap();

        let removed = store
            .clear_older_than(Duration::from_secs(3_600))
            .expect("clear_older_than must succeed");

        assert_eq!(removed, 1, "only the stale key should be removed");
        assert!(store.get::<String>("fresh").unwrap().is_some());
        assert!(store.get::<String>("stale").unwrap().is_none());
    }

    // ── info ──────────────────────────────────────────────────────────────────

    #[test]
    fn info_returns_correct_entry_count() {
        let store = temp_store();
        assert_eq!(store.info().entry_count, 0);
        store.set("x", &42u32, Duration::from_secs(60)).unwrap();
        assert_eq!(store.info().entry_count, 1);
    }

    #[test]
    fn info_last_write_is_some_after_insert() {
        let store = temp_store();
        assert!(store.info().last_write.is_none());
        store.set("w", &"data", Duration::from_secs(60)).unwrap();
        assert!(store.info().last_write.is_some());
    }

    // ── duration round-trip ───────────────────────────────────────────────────

    #[test]
    fn duration_survives_serialisation_round_trip() {
        let store = temp_store();
        let ttl = Duration::from_secs(7200);
        store.set("dur", &"value", ttl).unwrap();
        let entry = store.get::<String>("dur").unwrap().unwrap();
        assert_eq!(entry.ttl, ttl);
    }

    // ── file store specifics ──────────────────────────────────────────────────

    #[test]
    fn keys_map_to_safe_distinct_file_names() {
        let a = entry_file_name("summaries:2025-01-06:2025-01-12:project:my/saas");
        let b = entry_file_name("summaries:2025-01-06:2025-01-12:project:my:saas");
        assert_ne!(a, b);
        assert!(!a.contains('/') && !a.contains(':'), "{a}");
        // Very long keys stay within file-name limits and remain distinct.
        let long1 = entry_file_name(&format!("k:{}", "x".repeat(500)));
        let long2 = entry_file_name(&format!("k:{}y", "x".repeat(499)));
        assert!(long1.len() < 200 && long2.len() < 200);
        assert_ne!(long1, long2);
    }

    #[test]
    fn two_handles_on_the_same_store_coexist() {
        // sled held an exclusive lock: a second open failed (WouldBlock), which
        // silently disabled the cache when two waka processes overlapped.
        let first = temp_store();
        let second = CacheStore::open_at(first.root.clone()).expect("second open must succeed");
        first.set("k", &"v", Duration::from_secs(60)).unwrap();
        assert_eq!(second.get::<String>("k").unwrap().unwrap().value, "v");
    }

    #[test]
    fn open_removes_legacy_sled_files() {
        let root = std::env::temp_dir().join(format!("waka-cache-legacy-{}", std::process::id()));
        std::fs::create_dir_all(root.join("blobs")).unwrap();
        std::fs::write(root.join("db"), b"sled").unwrap();
        std::fs::write(root.join("conf"), b"sled").unwrap();
        std::fs::write(root.join("snap.0000000000000001"), b"sled").unwrap();

        let store = CacheStore::open_at(root.clone()).unwrap();
        assert!(!root.join("db").exists() && !root.join("conf").exists());
        assert!(!root.join("blobs").exists() && !root.join("snap.0000000000000001").exists());
        assert_eq!(store.info().entry_count, 0);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn profile_names_cannot_escape_the_cache_dir() {
        assert_eq!(sanitize_profile("work"), "work");
        let s = sanitize_profile("../../etc");
        assert!(!s.contains('/') && !s.contains(".."), "{s}");
    }
}
