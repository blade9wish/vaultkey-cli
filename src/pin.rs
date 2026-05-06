use crate::error::VaultError;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// A pinned secret entry with an optional label and timestamp.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PinnedSecret {
    pub key: String,
    pub label: Option<String>,
    pub pinned_at: u64,
}

/// In-memory store for pinned secrets within a bundle.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PinStore {
    pub pins: HashMap<String, PinnedSecret>,
}

impl PinStore {
    pub fn new() -> Self {
        Self {
            pins: HashMap::new(),
        }
    }

    pub fn pin(&mut self, key: &str, label: Option<String>) -> Result<(), VaultError> {
        if key.trim().is_empty() {
            return Err(VaultError::InvalidInput("Pin key cannot be empty".into()));
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| VaultError::InvalidInput(e.to_string()))?
            .as_secs();
        self.pins.insert(
            key.to_string(),
            PinnedSecret {
                key: key.to_string(),
                label,
                pinned_at: now,
            },
        );
        Ok(())
    }

    pub fn unpin(&mut self, key: &str) -> Result<(), VaultError> {
        if self.pins.remove(key).is_none() {
            return Err(VaultError::NotFound(format!("No pin found for key '{key}'")));
        }
        Ok(())
    }

    pub fn is_pinned(&self, key: &str) -> bool {
        self.pins.contains_key(key)
    }

    pub fn list(&self) -> Vec<&PinnedSecret> {
        let mut pins: Vec<&PinnedSecret> = self.pins.values().collect();
        pins.sort_by_key(|p| p.pinned_at);
        pins
    }

    pub fn get(&self, key: &str) -> Option<&PinnedSecret> {
        self.pins.get(key)
    }
}
