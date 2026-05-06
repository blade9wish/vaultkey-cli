use crate::error::VaultError;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpireEntry {
    pub key: String,
    pub expires_at: DateTime<Utc>,
    pub notify_before_secs: Option<i64>,
}

impl ExpireEntry {
    pub fn new(key: &str, expires_at: DateTime<Utc>, notify_before_secs: Option<i64>) -> Self {
        Self {
            key: key.to_string(),
            expires_at,
            notify_before_secs,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn is_expiring_soon(&self) -> bool {
        if let Some(secs) = self.notify_before_secs {
            let threshold = self.expires_at - Duration::seconds(secs);
            Utc::now() >= threshold && !self.is_expired()
        } else {
            false
        }
    }

    pub fn seconds_remaining(&self) -> i64 {
        (self.expires_at - Utc::now()).num_seconds().max(0)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExpireRegistry {
    pub entries: HashMap<String, ExpireEntry>,
}

impl ExpireRegistry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn set(&mut self, entry: ExpireEntry) {
        self.entries.insert(entry.key.clone(), entry);
    }

    pub fn get(&self, key: &str) -> Option<&ExpireEntry> {
        self.entries.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Result<(), VaultError> {
        self.entries.remove(key).ok_or_else(|| {
            VaultError::NotFound(format!("No expiry entry for key: {}", key))
        })?;
        Ok(())
    }

    pub fn expired_keys(&self) -> Vec<&ExpireEntry> {
        self.entries.values().filter(|e| e.is_expired()).collect()
    }

    pub fn expiring_soon_keys(&self) -> Vec<&ExpireEntry> {
        self.entries.values().filter(|e| e.is_expiring_soon()).collect()
    }

    pub fn purge_expired(&mut self) -> Vec<String> {
        let expired: Vec<String> = self.entries
            .values()
            .filter(|e| e.is_expired())
            .map(|e| e.key.clone())
            .collect();
        for key in &expired {
            self.entries.remove(key);
        }
        expired
    }
}
