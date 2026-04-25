//! Persistence layer for the Keyring — load/save to TOML files.

use crate::error::VaultKeyError;
use crate::keyring::Keyring;
use std::fs;
use std::path::{Path, PathBuf};

/// Manages loading and saving a `Keyring` to disk in TOML format.
pub struct KeyringStore {
    path: PathBuf,
}

impl KeyringStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Load keyring from the TOML file. Returns an empty keyring if file does not exist.
    pub fn load(&self) -> Result<Keyring, VaultKeyError> {
        if !self.path.exists() {
            return Ok(Keyring::new());
        }
        let content = fs::read_to_string(&self.path).map_err(|e| {
            VaultKeyError::Io(format!("Failed to read keyring file: {}", e))
        })?;
        toml::from_str(&content).map_err(|e| {
            VaultKeyError::Config(format!("Failed to parse keyring TOML: {}", e))
        })
    }

    /// Save the keyring to the TOML file, creating parent directories if needed.
    pub fn save(&self, keyring: &Keyring) -> Result<(), VaultKeyError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                VaultKeyError::Io(format!("Failed to create keyring directory: {}", e))
            })?;
        }
        let content = toml::to_string_pretty(keyring).map_err(|e| {
            VaultKeyError::Config(format!("Failed to serialize keyring: {}", e))
        })?;
        fs::write(&self.path, content).map_err(|e| {
            VaultKeyError::Io(format!("Failed to write keyring file: {}", e))
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
