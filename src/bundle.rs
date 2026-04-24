use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::VaultError;

/// Represents an encrypted secret bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub name: String,
    pub description: Option<String>,
    pub secrets: HashMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Bundle {
    pub fn new(name: impl Into<String>, description: Option<String>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Bundle {
            name: name.into(),
            description,
            secrets: HashMap::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn add_secret(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let now = chrono::Utc::now().to_rfc3339();
        self.secrets.insert(key.into(), value.into());
        self.updated_at = now;
    }

    pub fn remove_secret(&mut self, key: &str) -> Result<(), VaultError> {
        if self.secrets.remove(key).is_none() {
            return Err(VaultError::NotFound(format!("Secret '{}' not found in bundle '{}'", key, self.name)));
        }
        self.updated_at = chrono::Utc::now().to_rfc3339();
        Ok(())
    }

    pub fn get_secret(&self, key: &str) -> Result<&str, VaultError> {
        self.secrets
            .get(key)
            .map(|v| v.as_str())
            .ok_or_else(|| VaultError::NotFound(format!("Secret '{}' not found in bundle '{}'", key, self.name)))
    }

    pub fn to_toml(&self) -> Result<String, VaultError> {
        toml::to_string_pretty(self).map_err(|e| VaultError::Serialization(e.to_string()))
    }

    pub fn from_toml(content: &str) -> Result<Self, VaultError> {
        toml::from_str(content).map_err(|e| VaultError::Deserialization(e.to_string()))
    }
}
