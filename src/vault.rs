use std::collections::HashMap;
use std::path::Path;

use crate::bundle::Bundle;
use crate::config::Config;
use crate::crypto::CryptoBackend;
use crate::error::VaultError;

/// Represents an open vault session with loaded secrets
pub struct Vault {
    pub name: String,
    pub secrets: HashMap<String, String>,
    pub config: Config,
}

impl Vault {
    /// Open a vault by decrypting the bundle at the configured path
    pub fn open(config: Config, crypto: &dyn CryptoBackend) -> Result<Self, VaultError> {
        let bundle_path = config.bundle_path();
        if !Path::new(&bundle_path).exists() {
            return Err(VaultError::BundleNotFound(bundle_path));
        }

        let bundle = Bundle::load(&bundle_path)?;
        let decrypted = crypto.decrypt(&bundle.ciphertext)?;
        let secrets: HashMap<String, String> =
            toml::from_str(&decrypted).map_err(|e| VaultError::ParseError(e.to_string()))?;

        Ok(Vault {
            name: config.vault_name.clone(),
            secrets,
            config,
        })
    }

    /// Retrieve a secret by key
    pub fn get(&self, key: &str) -> Option<&String> {
        self.secrets.get(key)
    }

    /// Insert or update a secret
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.secrets.insert(key.into(), value.into());
    }

    /// Remove a secret by key, returns true if the key existed
    pub fn remove(&mut self, key: &str) -> bool {
        self.secrets.remove(key).is_some()
    }

    /// List all secret keys
    pub fn list_keys(&self) -> Vec<&String> {
        let mut keys: Vec<&String> = self.secrets.keys().collect();
        keys.sort();
        keys
    }

    /// Persist the vault by encrypting secrets back into a bundle
    pub fn save(&self, crypto: &dyn CryptoBackend) -> Result<(), VaultError> {
        let plaintext =
            toml::to_string(&self.secrets).map_err(|e| VaultError::ParseError(e.to_string()))?;
        let ciphertext = crypto.encrypt(&plaintext)?;
        let bundle = Bundle::new(self.name.clone(), ciphertext);
        bundle.save(&self.config.bundle_path())?;
        Ok(())
    }
}
