use crate::error::VaultError;
use crate::notify::{NotifyChannel, NotifyRule};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NotifyStore {
    pub rules: Vec<NotifyRule>,
}

impl NotifyStore {
    pub fn load(path: &Path) -> Result<Self, VaultError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(path).map_err(VaultError::Io)?;
        toml::from_str(&raw).map_err(|e| VaultError::Config(e.to_string()))
    }

    pub fn save(&self, path: &Path) -> Result<(), VaultError> {
        let raw = toml::to_string_pretty(self)
            .map_err(|e| VaultError::Config(e.to_string()))?;
        fs::write(path, raw).map_err(VaultError::Io)
    }

    pub fn add_rule(&mut self, event: &str, channel: NotifyChannel, filter: Option<String>) {
        self.rules.push(NotifyRule {
            event: event.to_string(),
            channel,
            filter,
        });
    }

    pub fn remove_rules_for_event(&mut self, event: &str) -> usize {
        let before = self.rules.len();
        self.rules.retain(|r| r.event != event);
        before - self.rules.len()
    }

    pub fn list(&self) -> &[NotifyRule] {
        &self.rules
    }
}
