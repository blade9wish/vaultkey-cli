use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use crate::error::VaultError;

#[derive(Debug, Clone)]
pub struct WatchEntry {
    pub key: String,
    pub path: PathBuf,
    pub last_modified: SystemTime,
    pub checksum: String,
}

#[derive(Debug, Default)]
pub struct WatchRegistry {
    pub entries: HashMap<String, WatchEntry>,
}

impl WatchRegistry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: String, path: PathBuf, checksum: String) -> Result<(), VaultError> {
        let last_modified = SystemTime::now();
        self.entries.insert(
            key.clone(),
            WatchEntry {
                key,
                path,
                last_modified,
                checksum,
            },
        );
        Ok(())
    }

    pub fn unregister(&mut self, key: &str) -> Result<(), VaultError> {
        if self.entries.remove(key).is_none() {
            return Err(VaultError::NotFound(format!("Watch entry '{}' not found", key)));
        }
        Ok(())
    }

    pub fn check_changed(&self, key: &str, current_checksum: &str) -> Result<bool, VaultError> {
        match self.entries.get(key) {
            Some(entry) => Ok(entry.checksum != current_checksum),
            None => Err(VaultError::NotFound(format!("Watch entry '{}' not found", key))),
        }
    }

    pub fn update_checksum(&mut self, key: &str, new_checksum: String) -> Result<(), VaultError> {
        match self.entries.get_mut(key) {
            Some(entry) => {
                entry.checksum = new_checksum;
                entry.last_modified = SystemTime::now();
                Ok(())
            }
            None => Err(VaultError::NotFound(format!("Watch entry '{}' not found", key))),
        }
    }

    pub fn stale_entries(&self, max_age: Duration) -> Vec<&WatchEntry> {
        let now = SystemTime::now();
        self.entries
            .values()
            .filter(|e| {
                now.duration_since(e.last_modified)
                    .map(|age| age > max_age)
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn list(&self) -> Vec<&WatchEntry> {
        self.entries.values().collect()
    }
}

pub fn compute_checksum(data: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
