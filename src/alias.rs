use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::VaultError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AliasMap {
    pub entries: HashMap<String, String>,
}

impl AliasMap {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn add(&mut self, alias: &str, target: &str) -> Result<(), VaultError> {
        if alias.is_empty() || target.is_empty() {
            return Err(VaultError::InvalidInput(
                "Alias and target must not be empty".into(),
            ));
        }
        if self.entries.contains_key(alias) {
            return Err(VaultError::InvalidInput(
                format!("Alias '{}' already exists", alias),
            ));
        }
        self.entries.insert(alias.to_string(), target.to_string());
        Ok(())
    }

    pub fn remove(&mut self, alias: &str) -> Result<(), VaultError> {
        if self.entries.remove(alias).is_none() {
            return Err(VaultError::NotFound(format!("Alias '{}' not found", alias)));
        }
        Ok(())
    }

    pub fn resolve(&self, alias: &str) -> Option<&String> {
        self.entries.get(alias)
    }

    pub fn list(&self) -> Vec<(&String, &String)> {
        let mut pairs: Vec<_> = self.entries.iter().collect();
        pairs.sort_by_key(|(k, _)| k.as_str());
        pairs
    }

    pub fn rename(&mut self, old: &str, new_alias: &str) -> Result<(), VaultError> {
        let target = self
            .entries
            .remove(old)
            .ok_or_else(|| VaultError::NotFound(format!("Alias '{}' not found", old)))?;
        if self.entries.contains_key(new_alias) {
            self.entries.insert(old.to_string(), target);
            return Err(VaultError::InvalidInput(
                format!("Alias '{}' already exists", new_alias),
            ));
        }
        self.entries.insert(new_alias.to_string(), target);
        Ok(())
    }
}
