use crate::error::VaultError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Policy {
    pub name: String,
    pub required_tags: Vec<String>,
    pub forbidden_keys: Vec<String>,
    pub max_secret_length: Option<usize>,
    pub allowed_key_prefixes: Vec<String>,
}

impl Policy {
    pub fn new(name: impl Into<String>) -> Self {
        Policy {
            name: name.into(),
            required_tags: vec![],
            forbidden_keys: vec![],
            max_secret_length: None,
            allowed_key_prefixes: vec![],
        }
    }

    pub fn validate_key(&self, key: &str) -> Result<(), VaultError> {
        if self.forbidden_keys.contains(&key.to_string()) {
            return Err(VaultError::PolicyViolation(format!(
                "Key '{}' is forbidden by policy '{}'",
                key, self.name
            )));
        }
        if !self.allowed_key_prefixes.is_empty() {
            let allowed = self
                .allowed_key_prefixes
                .iter()
                .any(|prefix| key.starts_with(prefix.as_str()));
            if !allowed {
                return Err(VaultError::PolicyViolation(format!(
                    "Key '{}' does not match any allowed prefix in policy '{}'",
                    key, self.name
                )));
            }
        }
        Ok(())
    }

    pub fn validate_secret_length(&self, value: &str) -> Result<(), VaultError> {
        if let Some(max) = self.max_secret_length {
            if value.len() > max {
                return Err(VaultError::PolicyViolation(format!(
                    "Secret value exceeds max length {} defined in policy '{}'",
                    max, self.name
                )));
            }
        }
        Ok(())
    }

    pub fn validate_tags(&self, tags: &HashSet<String>) -> Result<(), VaultError> {
        for required in &self.required_tags {
            if !tags.contains(required) {
                return Err(VaultError::PolicyViolation(format!(
                    "Required tag '{}' missing per policy '{}'",
                    required, self.name
                )));
            }
        }
        Ok(())
    }
}
