//! Local cache abstraction for `waka`.
//!
//! Wraps [`sled`] to provide a typed, TTL-aware key-value store used by the
//! `waka` CLI to cache `WakaTime` API responses and reduce network calls.
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

use std::path::PathBuf;
use std::time::Duration;

use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::warn;

// ─── Error type ───────────────────────────────────────────────────────────────

/// Errors that can occur when interacting with the cache store.
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    /// The platform cache directory could not be determined (unusual — sandbox).
    #[error("could not determine cache directory")]
    NoCacheDir,

    /// The sled database could not be opened.
    ///
    /// This is returned only on the initial open; if the DB is corrupted after
    /// opening, operations degrade gracefully and log warnings instead of
    /// returning this error.
    #[error("failed to open cache database at {path}: {source}")]
    DbOpen {
        /// Path that was attempted.
        path: PathBuf,
        /// Underlying sled error.
        source: sled::Error,
    },

    /// A serialization or deserialization failure.
    #[error("cache serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// A low-level sled I/O error.
    #[error("cache I/O error: {0}")]
    Io(#[from] sled::Error),
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

/// A typed, TTL-aware local cache backed by [`sled`].
///
/// Each profile gets its own `sled` database under the platform cache directory:
///
/// | Platform | Path |
/// |----------|------|
/// | Linux   | `~/.cache/waka/<profile>/` |
/// | macOS   | `~/Library/Caches/waka/<profile>/` |
/// | Windows | `%LOCALAPPDATA%\waka\<profile>\` |
///
/// If the database is corrupted, read operations return `None` and write
/// operations are silently skipped — the cache is best-effort, never fatal.
#[derive(Debug, Clone)]
pub struct CacheStore {
    db: sled::Db,
    /// Path to the sled database directory (used for [`CacheInfo::size_on_disk`]).
    path: PathBuf,
}

impl CacheStore {
    /// Opens (or creates) the cache store for the named profile.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::NoCacheDir`] if the platform cache directory
    /// cannot be determined, or [`CacheError::DbOpen`] if sled fails to open.
    pub fn open(profile: &str) -> Result<Self, CacheError> {
        let path = Self::db_path(profile)?;
        std::fs::create_dir_all(&path).ok();

        match sled::open(&path) {
            Ok(db) => Ok(Self { db, path }),
            Err(source) => Err(CacheError::DbOpen { path, source }),
        }
    }

    /// Returns the filesystem path used for the named profile's sled database.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::NoCacheDir`] if the platform directories cannot
    /// be determined.
    pub fn db_path(profile: &str) -> Result<PathBuf, CacheError> {
        let dirs = ProjectDirs::from("", "", "waka").ok_or(CacheError::NoCacheDir)?;
        Ok(dirs.cache_dir().join(profile))
    }

    /// Retrieves a cached value by key.
    ///
    /// Returns `Ok(None)` on a cache miss, on corrupted data (with a warning),
    /// or if the key does not exist.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::Io`] on a low-level sled failure.
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<CacheEntry<T>>, CacheError> {
        let Some(bytes) = self.db.get(key)? else {
            return Ok(None);
        };

        match serde_json::from_slice::<CacheEntry<T>>(&bytes) {
            Ok(entry) => Ok(Some(entry)),
            Err(err) => {
                warn!(key, %err, "cache entry is corrupted — dropping");
                // Remove corrupted entry to avoid repeated warnings.
                let _ = self.db.remove(key);
                Ok(None)
            }
        }
    }

    /// Stores a value under `key` with the given TTL.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::Serde`] if serialization fails, or
    /// [`CacheError::Io`] on a sled write failure.
    pub fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), CacheError> {
        let entry = CacheEntry {
            value,
            inserted_at: Utc::now(),
            ttl,
        };
        let bytes = serde_json::to_vec(&entry)?;
        self.db.insert(key, bytes)?;
        Ok(())
    }

    /// Removes all entries from the cache.
    ///
    /// Returns the number of entries that were removed.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::Io`] on a sled failure.
    pub fn clear(&self) -> Result<usize, CacheError> {
        let count = self.db.len();
        self.db.clear()?;
        Ok(count)
    }

    /// Removes entries that were inserted more than `older_than` ago.
    ///
    /// Returns the number of entries removed.
    ///
    /// # Errors
    ///
    /// Returns [`CacheError::Io`] on a sled failure.
    pub fn clear_older_than(&self, older_than: Duration) -> Result<usize, CacheError> {
        // Envelope used to decode only `inserted_at` without full deserialisation.
        #[derive(Deserialize)]
        struct Envelope {
            inserted_at: DateTime<Utc>,
        }

        let cutoff =
            Utc::now() - chrono::Duration::from_std(older_than).unwrap_or(chrono::Duration::zero());

        let mut removed = 0usize;
        for result in self.db.iter() {
            let (k, v) = match result {
                Ok(pair) => pair,
                Err(err) => {
                    warn!(%err, "error iterating cache entries — skipping");
                    continue;
                }
            };

            if let Ok(env) = serde_json::from_slice::<Envelope>(&v) {
                if env.inserted_at < cutoff && self.db.remove(&k).is_ok() {
                    removed += 1;
                }
            } else {
                warn!(key = ?k, "cache entry has no inserted_at — removing");
                if self.db.remove(&k).is_ok() {
                    removed += 1;
                }
            }
        }

        Ok(removed)
    }

    /// Returns summary statistics about the cache store.
    #[must_use]
    pub fn info(&self) -> CacheInfo {
        // Envelope used to decode only `inserted_at` without full deserialisation.
        #[derive(Deserialize)]
        struct Envelope {
            inserted_at: DateTime<Utc>,
        }

        let entry_count = self.db.len();

        // Approximate disk size: sum the sizes of all sled data files.
        let size_on_disk = dir_size_bytes(&self.path);

        // Find the most recently inserted entry.
        let last_write = self
            .db
            .iter()
            .filter_map(Result::ok)
            .filter_map(|(_, v)| serde_json::from_slice::<Envelope>(&v).ok())
            .map(|e| e.inserted_at)
            .max();

        CacheInfo {
            entry_count,
            size_on_disk,
            last_write,
        }
    }
}

// ─── helpers ──────────────────────────────────────────────────────────────────

/// Recursively sums the sizes of all files in `dir`.
fn dir_size_bytes(dir: &PathBuf) -> u64 {
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

    /// Opens an in-memory sled database (requires no filesystem path).
    fn in_memory_store() -> CacheStore {
        let db = sled::Config::default()
            .temporary(true)
            .open()
            .expect("in-memory sled must succeed");
        CacheStore {
            db,
            path: PathBuf::from("/tmp/waka-test-cache"),
        }
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
        let store = in_memory_store();
        let result = store
            .get::<String>("nonexistent")
            .expect("get must succeed");
        assert!(result.is_none());
    }

    #[test]
    fn set_then_get_roundtrip() {
        let store = in_memory_store();
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
        let store = in_memory_store();
        store.set("k", &"first", Duration::from_secs(60)).unwrap();
        store.set("k", &"second", Duration::from_secs(60)).unwrap();
        let entry = store.get::<String>("k").unwrap().unwrap();
        assert_eq!(entry.value, "second");
    }

    #[test]
    fn get_returns_none_for_corrupted_entry() {
        let store = in_memory_store();
        // Insert raw garbage bytes directly.
        store
            .db
            .insert("bad", b"not valid json at all".as_slice())
            .unwrap();
        let result = store.get::<String>("bad").expect("get must not error");
        assert!(result.is_none(), "corrupted entry should return None");
        // Entry should have been removed.
        assert!(
            store.db.get("bad").unwrap().is_none(),
            "corrupted entry must be cleaned up"
        );
    }

    // ── clear ─────────────────────────────────────────────────────────────────

    #[test]
    fn clear_removes_all_entries_and_returns_count() {
        let store = in_memory_store();
        store.set("a", &1u32, Duration::from_secs(60)).unwrap();
        store.set("b", &2u32, Duration::from_secs(60)).unwrap();
        store.set("c", &3u32, Duration::from_secs(60)).unwrap();
        let removed = store.clear().expect("clear must succeed");
        assert_eq!(removed, 3);
        assert_eq!(store.db.len(), 0);
    }

    #[test]
    fn clear_on_empty_store_returns_zero() {
        let store = in_memory_store();
        let removed = store.clear().expect("clear must succeed");
        assert_eq!(removed, 0);
    }

    // ── clear_older_than ──────────────────────────────────────────────────────

    #[test]
    fn clear_older_than_removes_old_and_keeps_fresh() {
        let store = in_memory_store();

        // Insert a fresh entry.
        store.set("fresh", &"ok", Duration::from_secs(300)).unwrap();

        // Insert a stale entry by writing directly with an old timestamp.
        let old_entry = CacheEntry {
            value: "old",
            inserted_at: Utc::now() - chrono::Duration::hours(2),
            ttl: Duration::from_secs(300),
        };
        let bytes = serde_json::to_vec(&old_entry).unwrap();
        store.db.insert("stale", bytes.as_slice()).unwrap();

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
        let store = in_memory_store();
        assert_eq!(store.info().entry_count, 0);
        store.set("x", &42u32, Duration::from_secs(60)).unwrap();
        assert_eq!(store.info().entry_count, 1);
    }

    #[test]
    fn info_last_write_is_some_after_insert() {
        let store = in_memory_store();
        assert!(store.info().last_write.is_none());
        store.set("w", &"data", Duration::from_secs(60)).unwrap();
        assert!(store.info().last_write.is_some());
    }

    // ── duration round-trip ───────────────────────────────────────────────────

    #[test]
    fn duration_survives_serialisation_round_trip() {
        let store = in_memory_store();
        let ttl = Duration::from_secs(7200);
        store.set("dur", &"value", ttl).unwrap();
        let entry = store.get::<String>("dur").unwrap().unwrap();
        assert_eq!(entry.ttl, ttl);
    }
}
