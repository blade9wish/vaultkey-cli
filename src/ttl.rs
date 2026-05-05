use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::error::VaultError;

/// Represents a time-to-live entry for a secret key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TtlEntry {
    pub key: String,
    pub expires_at: u64, // Unix timestamp
}

impl TtlEntry {
    /// Create a new TTL entry expiring `seconds` from now.
    pub fn new(key: &str, seconds: u64) -> Result<Self, VaultError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| VaultError::Generic(e.to_string()))?
            .as_secs();
        Ok(TtlEntry {
            key: key.to_string(),
            expires_at: now + seconds,
        })
    }

    /// Returns true if this entry has expired.
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now >= self.expires_at
    }

    /// Seconds remaining before expiry; returns 0 if already expired.
    pub fn seconds_remaining(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.expires_at.saturating_sub(now)
    }
}

/// Manages a collection of TTL entries.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TtlStore {
    pub entries: Vec<TtlEntry>,
}

impl TtlStore {
    pub fn new() -> Self {
        TtlStore { entries: Vec::new() }
    }

    /// Insert or update a TTL entry for the given key.
    pub fn set(&mut self, key: &str, seconds: u64) -> Result<(), VaultError> {
        let entry = TtlEntry::new(key, seconds)?;
        self.entries.retain(|e| e.key != key);
        self.entries.push(entry);
        Ok(())
    }

    /// Remove the TTL entry for a key.
    pub fn remove(&mut self, key: &str) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.key != key);
        self.entries.len() < before
    }

    /// Return all expired keys.
    pub fn expired_keys(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter(|e| e.is_expired())
            .map(|e| e.key.clone())
            .collect()
    }

    /// Purge all expired entries, returning the removed keys.
    pub fn purge_expired(&mut self) -> Vec<String> {
        let expired = self.expired_keys();
        self.entries.retain(|e| !e.is_expired());
        expired
    }

    /// Look up a TTL entry by key.
    pub fn get(&self, key: &str) -> Option<&TtlEntry> {
        self.entries.iter().find(|e| e.key == key)
    }
}
