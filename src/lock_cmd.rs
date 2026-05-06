use crate::lock::{get_lock_entry, is_locked, lock_vault, unlock_vault};
use crate::error::VaultError;
use std::path::Path;

pub fn cmd_lock(base_dir: &Path, vault_name: &str, reason: Option<&str>) -> Result<(), VaultError> {
    if is_locked(base_dir, vault_name) {
        return Err(VaultError::AlreadyExists(format!(
            "Vault '{}' is already locked.",
            vault_name
        )));
    }
    lock_vault(base_dir, vault_name, reason.map(|r| r.to_string()))?;
    println!("🔒 Vault '{}' has been locked.", vault_name);
    if let Some(r) = reason {
        println!("   Reason: {}", r);
    }
    Ok(())
}

pub fn cmd_unlock(base_dir: &Path, vault_name: &str) -> Result<(), VaultError> {
    if !is_locked(base_dir, vault_name) {
        return Err(VaultError::NotFound(format!(
            "Vault '{}' is not locked.",
            vault_name
        )));
    }
    unlock_vault(base_dir, vault_name)?;
    println!("🔓 Vault '{}' has been unlocked.", vault_name);
    Ok(())
}

pub fn cmd_lock_status(base_dir: &Path, vault_name: &str) -> Result<(), VaultError> {
    if !is_locked(base_dir, vault_name) {
        println!("Vault '{}' is unlocked.", vault_name);
        return Ok(());
    }
    let entry = get_lock_entry(base_dir, vault_name)?;
    println!("Vault '{}' is locked.", vault_name);
    println!("  Locked at : {}", entry.locked_at);
    if let Some(reason) = &entry.reason {
        println!("  Reason    : {}", reason);
    }
    Ok(())
}
