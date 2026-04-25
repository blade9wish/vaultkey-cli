//! Keyring module for managing GPG key identifiers and aliases.

use crate::error::VaultKeyError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a GPG key entry with an alias and fingerprint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyEntry {
    pub alias: String,
    pub fingerprint: String,
    pub email: Option<String>,
    pub description: Option<String>,
}

impl KeyEntry {
    pub fn new(alias: impl Into<String>, fingerprint: impl Into<String>) -> Self {
        Self {
            alias: alias.into(),
            fingerprint: fingerprint.into(),
            email: None,
            description: None,
        }
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// In-memory keyring that maps aliases to GPG key entries.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Keyring {
    keys: HashMap<String, KeyEntry>,
}

impl Keyring {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: KeyEntry) -> Result<(), VaultKeyError> {
        if self.keys.contains_key(&entry.alias) {
            return Err(VaultKeyError::Config(format!(
                "Key alias '{}' already exists",
                entry.alias
            )));
        }
        self.keys.insert(entry.alias.clone(), entry);
        Ok(())
    }

    pub fn remove(&mut self, alias: &str) -> Result<KeyEntry, VaultKeyError> {
        self.keys.remove(alias).ok_or_else(|| {
            VaultKeyError::Config(format!("Key alias '{}' not found", alias))
        })
    }

    pub fn get(&self, alias: &str) -> Option<&KeyEntry> {
        self.keys.get(alias)
    }

    pub fn list(&self) -> Vec<&KeyEntry> {
        let mut entries: Vec<&KeyEntry> = self.keys.values().collect();
        entries.sort_by(|a, b| a.alias.cmp(&b.alias));
        entries
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}
