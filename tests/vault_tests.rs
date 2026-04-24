use std::collections::HashMap;

use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::config::Config;
use vaultkey_cli::crypto::CryptoBackend;
use vaultkey_cli::error::VaultError;
use vaultkey_cli::vault::Vault;

/// A simple no-op crypto backend for testing
struct PlaintextCrypto;

impl CryptoBackend for PlaintextCrypto {
    fn encrypt(&self, data: &str) -> Result<Vec<u8>, VaultError> {
        Ok(data.as_bytes().to_vec())
    }

    fn decrypt(&self, data: &[u8]) -> Result<String, VaultError> {
        String::from_utf8(data.to_vec()).map_err(|e| VaultError::CryptoError(e.to_string()))
    }
}

fn make_config(dir: &tempfile::TempDir) -> Config {
    Config {
        vault_name: "test-vault".to_string(),
        bundle_dir: dir.path().to_string_lossy().to_string(),
        gpg_key_id: None,
    }
}

#[test]
fn test_vault_set_and_get() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_config(&dir);
    let crypto = PlaintextCrypto;

    // Bootstrap an empty bundle so Vault::open can load it
    let empty_toml = "";
    let bundle = Bundle::new("test-vault".to_string(), empty_toml.as_bytes().to_vec());
    bundle.save(&config.bundle_path()).unwrap();

    let mut vault = Vault::open(config.clone(), &crypto).unwrap();
    vault.set("api_key", "supersecret");

    assert_eq!(vault.get("api_key"), Some(&"supersecret".to_string()));
    assert_eq!(vault.get("missing"), None);
}

#[test]
fn test_vault_remove() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_config(&dir);
    let crypto = PlaintextCrypto;

    let initial = "db_pass = \"hunter2\"\n";
    let bundle = Bundle::new("test-vault".to_string(), initial.as_bytes().to_vec());
    bundle.save(&config.bundle_path()).unwrap();

    let mut vault = Vault::open(config.clone(), &crypto).unwrap();
    assert!(vault.remove("db_pass"));
    assert!(!vault.remove("db_pass")); // already removed
    assert_eq!(vault.get("db_pass"), None);
}

#[test]
fn test_vault_save_and_reload() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_config(&dir);
    let crypto = PlaintextCrypto;

    let empty = "";
    let bundle = Bundle::new("test-vault".to_string(), empty.as_bytes().to_vec());
    bundle.save(&config.bundle_path()).unwrap();

    let mut vault = Vault::open(config.clone(), &crypto).unwrap();
    vault.set("token", "abc123");
    vault.save(&crypto).unwrap();

    let reloaded = Vault::open(config.clone(), &crypto).unwrap();
    assert_eq!(reloaded.get("token"), Some(&"abc123".to_string()));
}

#[test]
fn test_vault_list_keys_sorted() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_config(&dir);
    let crypto = PlaintextCrypto;

    let toml = "zebra = \"z\"\napple = \"a\"\nmango = \"m\"\n";
    let bundle = Bundle::new("test-vault".to_string(), toml.as_bytes().to_vec());
    bundle.save(&config.bundle_path()).unwrap();

    let vault = Vault::open(config, &crypto).unwrap();
    let keys = vault.list_keys();
    assert_eq!(keys, vec!["apple", "mango", "zebra"]);
}
