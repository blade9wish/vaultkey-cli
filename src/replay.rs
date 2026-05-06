//! Replay protection module — tracks nonces to prevent secret re-use attacks.

use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::VaultError;

/// A nonce entry with a timestamp for expiry tracking.
#[derive(Debug, Clone)]
pub struct NonceEntry {
    pub nonce: String,
    pub created_at: u64,
}

/// In-memory replay guard that stores seen nonces.
#[derive(Debug, Default)]
pub struct ReplayGuard {
    seen: HashSet<String>,
    window_secs: u64,
    entries: Vec<NonceEntry>,
}

impl ReplayGuard {
    /// Create a new guard with a given replay window in seconds.
    pub fn new(window_secs: u64) -> Self {
        Self {
            seen: HashSet::new(),
            window_secs,
            entries: Vec::new(),
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Purge nonces older than the replay window.
    pub fn purge_expired(&mut self) {
        let cutoff = Self::now().saturating_sub(self.window_secs);
        self.entries.retain(|e| e.created_at >= cutoff);
        self.seen = self.entries.iter().map(|e| e.nonce.clone()).collect();
    }

    /// Check and register a nonce. Returns error if already seen within window.
    pub fn check_and_register(&mut self, nonce: &str) -> Result<(), VaultError> {
        self.purge_expired();
        if self.seen.contains(nonce) {
            return Err(VaultError::Generic(format!(
                "Replay detected: nonce '{}' has already been used",
                nonce
            )));
        }
        let entry = NonceEntry {
            nonce: nonce.to_string(),
            created_at: Self::now(),
        };
        self.seen.insert(nonce.to_string());
        self.entries.push(entry);
        Ok(())
    }

    /// Returns how many nonces are currently tracked.
    pub fn active_count(&self) -> usize {
        self.seen.len()
    }
}
