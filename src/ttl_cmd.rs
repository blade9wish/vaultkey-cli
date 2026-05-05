use crate::ttl::TtlStore;
use crate::error::VaultError;
use std::path::Path;
use std::fs;

const TTL_FILE: &str = "ttl.json";

fn load_store(vault_dir: &str) -> Result<TtlStore, VaultError> {
    let path = Path::new(vault_dir).join(TTL_FILE);
    if !path.exists() {
        return Ok(TtlStore::new());
    }
    let data = fs::read_to_string(&path)
        .map_err(|e| VaultError::Io(e.to_string()))?;
    serde_json::from_str(&data)
        .map_err(|e| VaultError::Generic(e.to_string()))
}

fn save_store(vault_dir: &str, store: &TtlStore) -> Result<(), VaultError> {
    let path = Path::new(vault_dir).join(TTL_FILE);
    let data = serde_json::to_string_pretty(store)
        .map_err(|e| VaultError::Generic(e.to_string()))?;
    fs::write(&path, data)
        .map_err(|e| VaultError::Io(e.to_string()))
}

/// Set a TTL (in seconds) on a secret key.
pub fn cmd_ttl_set(vault_dir: &str, key: &str, seconds: u64) -> Result<(), VaultError> {
    let mut store = load_store(vault_dir)?;
    store.set(key, seconds)?;
    save_store(vault_dir, &store)?;
    println!("TTL set: '{}' expires in {} second(s).", key, seconds);
    Ok(())
}

/// Remove the TTL from a secret key.
pub fn cmd_ttl_remove(vault_dir: &str, key: &str) -> Result<(), VaultError> {
    let mut store = load_store(vault_dir)?;
    if store.remove(key) {
        save_store(vault_dir, &store)?;
        println!("TTL removed for '{}'.", key);
    } else {
        println!("No TTL entry found for '{}'.", key);
    }
    Ok(())
}

/// List all TTL entries and their remaining time.
pub fn cmd_ttl_list(vault_dir: &str) -> Result<(), VaultError> {
    let store = load_store(vault_dir)?;
    if store.entries.is_empty() {
        println!("No TTL entries.");
        return Ok(());
    }
    println!("{:<30} {:>15}  {}", "Key", "Remaining (s)", "Status");
    println!("{}", "-".repeat(58));
    for entry in &store.entries {
        let remaining = entry.seconds_remaining();
        let status = if entry.is_expired() { "EXPIRED" } else { "active" };
        println!("{:<30} {:>15}  {}", entry.key, remaining, status);
    }
    Ok(())
}

/// Purge all expired entries from the TTL store.
pub fn cmd_ttl_purge(vault_dir: &str) -> Result<(), VaultError> {
    let mut store = load_store(vault_dir)?;
    let removed = store.purge_expired();
    if removed.is_empty() {
        println!("No expired TTL entries to purge.");
    } else {
        save_store(vault_dir, &store)?;
        for key in &removed {
            println!("Purged expired key: '{}'", key);
        }
        println!("Purged {} expired entry/entries.", removed.len());
    }
    Ok(())
}
