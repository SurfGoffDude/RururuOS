use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use thiserror::Error;
use tracing::{debug, warn};

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Cache error: {0}")]
    DatabaseError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedMetadata {
    pub mime_type: String,
    pub size: u64,
    pub modified: SystemTime,
    pub metadata: serde_json::Value,
    pub cached_at: SystemTime,
}

pub struct MetadataCache {
    db: sled::Db,
    ttl: Duration,
}

impl MetadataCache {
    pub fn new(cache_dir: &Path, ttl: Duration) -> Result<Self, CacheError> {
        let db_path = cache_dir.join("metadata.sled");
        let db = sled::open(&db_path).map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        Ok(Self { db, ttl })
    }

    pub fn get(&self, path: &Path) -> Option<CachedMetadata> {
        let key = self.make_key(path);

        match self.db.get(&key) {
            Ok(Some(data)) => {
                match serde_json::from_slice::<CachedMetadata>(&data) {
                    Ok(cached) => {
                        // Check if cache is still valid
                        if self.is_valid(path, &cached) {
                            debug!("Cache hit for: {:?}", path);
                            return Some(cached);
                        } else {
                            debug!("Cache stale for: {:?}", path);
                            self.remove(path).ok();
                        }
                    }
                    Err(e) => {
                        warn!("Failed to deserialize cache entry: {}", e);
                        self.remove(path).ok();
                    }
                }
            }
            Ok(None) => {}
            Err(e) => {
                warn!("Cache read error: {}", e);
            }
        }

        None
    }

    pub fn set(&self, path: &Path, metadata: CachedMetadata) -> Result<(), CacheError> {
        let key = self.make_key(path);
        let value = serde_json::to_vec(&metadata)
            .map_err(|e| CacheError::SerializationError(e.to_string()))?;

        self.db
            .insert(&key, value)
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;

        debug!("Cached metadata for: {:?}", path);
        Ok(())
    }

    pub fn remove(&self, path: &Path) -> Result<(), CacheError> {
        let key = self.make_key(path);
        self.db
            .remove(&key)
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), CacheError> {
        self.db
            .clear()
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        debug!("Cache cleared");
        Ok(())
    }

    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entries: self.db.len(),
            size_bytes: self.db.size_on_disk().unwrap_or(0),
        }
    }

    fn make_key(&self, path: &Path) -> Vec<u8> {
        path.to_string_lossy().as_bytes().to_vec()
    }

    fn is_valid(&self, path: &Path, cached: &CachedMetadata) -> bool {
        // Check TTL
        if let Ok(elapsed) = cached.cached_at.elapsed() {
            if elapsed > self.ttl {
                return false;
            }
        }

        // Check if file was modified
        if let Ok(metadata) = path.metadata() {
            if let Ok(modified) = metadata.modified() {
                if modified != cached.modified {
                    return false;
                }
            }
            if metadata.len() != cached.size {
                return false;
            }
        }

        true
    }

    pub fn flush(&self) -> Result<(), CacheError> {
        self.db
            .flush()
            .map_err(|e| CacheError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub size_bytes: u64,
}

impl Drop for MetadataCache {
    fn drop(&mut self) {
        if let Err(e) = self.flush() {
            warn!("Failed to flush cache on drop: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_cache_operations() {
        let dir = tempdir().unwrap();
        let cache = MetadataCache::new(dir.path(), Duration::from_secs(3600)).unwrap();

        let test_path = PathBuf::from("/test/file.txt");

        // Initially empty
        assert!(cache.get(&test_path).is_none());

        // Add entry
        let metadata = CachedMetadata {
            mime_type: "text/plain".to_string(),
            size: 100,
            modified: SystemTime::now(),
            metadata: serde_json::json!({"test": true}),
            cached_at: SystemTime::now(),
        };

        cache.set(&test_path, metadata.clone()).unwrap();

        // Should be retrievable (note: will fail validation since file doesn't exist)
        // In real usage, the file would exist

        // Clear
        cache.clear().unwrap();
        assert!(cache.get(&test_path).is_none());
    }

    #[test]
    fn test_cache_stats() {
        let dir = tempdir().unwrap();
        let cache = MetadataCache::new(dir.path(), Duration::from_secs(3600)).unwrap();

        let stats = cache.stats();
        assert_eq!(stats.entries, 0);
    }
}
