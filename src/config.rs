use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::error::VaultError;

/// Top-level configuration for a vaultkey bundle
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VaultConfig {
    pub vault: VaultMeta,
    pub gpg: GpgConfig,
    #[serde(default)]
    pub secrets: Vec<SecretEntry>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VaultMeta {
    pub name: String,
    pub description: Option<String>,
    pub version: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GpgConfig {
    pub key_id: String,
    pub armor: bool,
    #[serde(default = "default_gpg_home")]
    pub gpg_home: String,
}

fn default_gpg_home() -> String {
    "~/.gnupg".to_string()
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SecretEntry {
    pub name: String,
    pub file: String,
    pub description: Option<String>,
}

impl VaultConfig {
    /// Load and parse a VaultConfig from a TOML file path
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, VaultError> {
        let contents = fs::read_to_string(&path)
            .map_err(|e| VaultError::ConfigRead(path.as_ref().display().to_string(), e))?;
        let config: VaultConfig = toml::from_str(&contents)
            .map_err(|e| VaultError::ConfigParse(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// Basic semantic validation of the config
    pub fn validate(&self) -> Result<(), VaultError> {
        if self.vault.name.trim().is_empty() {
            return Err(VaultError::Validation("vault.name must not be empty".into()));
        }
        if self.gpg.key_id.trim().is_empty() {
            return Err(VaultError::Validation("gpg.key_id must not be empty".into()));
        }
        for secret in &self.secrets {
            if secret.name.trim().is_empty() || secret.file.trim().is_empty() {
                return Err(VaultError::Validation(
                    format!("secret entry has empty name or file: {:?}", secret.name)
                ));
            }
        }
        Ok(())
    }
}
