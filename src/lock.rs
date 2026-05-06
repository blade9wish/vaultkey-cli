use crate::error::VaultError;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct LockEntry {
    pub vault_name: String,
    pub locked_at: u64,
    pub reason: Option<String>,
}

impl LockEntry {
    pub fn new(vault_name: &str, reason: Option<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        LockEntry {
            vault_name: vault_name.to_string(),
            locked_at: now,
            reason,
        }
    }

    pub fn to_toml_string(&self) -> String {
        let reason_str = match &self.reason {
            Some(r) => format!("reason = \"{}\"\n", r),
            None => String::new(),
        };
        format!(
            "vault_name = \"{}\"\nlocked_at = {}\n{}",
            self.vault_name, self.locked_at, reason_str
        )
    }

    pub fn from_toml_str(content: &str) -> Result<Self, VaultError> {
        let mut vault_name = String::new();
        let mut locked_at: u64 = 0;
        let mut reason: Option<String> = None;
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("vault_name = ") {
                vault_name = val.trim_matches('"').to_string();
            } else if let Some(val) = line.strip_prefix("locked_at = ") {
                locked_at = val.trim().parse().unwrap_or(0);
            } else if let Some(val) = line.strip_prefix("reason = ") {
                reason = Some(val.trim_matches('"').to_string());
            }
        }
        if vault_name.is_empty() {
            return Err(VaultError::Parse("missing vault_name in lock file".into()));
        }
        Ok(LockEntry { vault_name, locked_at, reason })
    }
}

fn lock_path(base_dir: &Path, vault_name: &str) -> PathBuf {
    base_dir.join(format!("{}.lock", vault_name))
}

pub fn lock_vault(base_dir: &Path, vault_name: &str, reason: Option<String>) -> Result<(), VaultError> {
    let path = lock_path(base_dir, vault_name);
    if path.exists() {
        return Err(VaultError::AlreadyExists(format!("vault '{}' is already locked", vault_name)));
    }
    let entry = LockEntry::new(vault_name, reason);
    fs::write(&path, entry.to_toml_string())
        .map_err(|e| VaultError::Io(e.to_string()))?;
    Ok(())
}

pub fn unlock_vault(base_dir: &Path, vault_name: &str) -> Result<(), VaultError> {
    let path = lock_path(base_dir, vault_name);
    if !path.exists() {
        return Err(VaultError::NotFound(format!("vault '{}' is not locked", vault_name)));
    }
    fs::remove_file(&path).map_err(|e| VaultError::Io(e.to_string()))?;
    Ok(())
}

pub fn is_locked(base_dir: &Path, vault_name: &str) -> bool {
    lock_path(base_dir, vault_name).exists()
}

pub fn get_lock_entry(base_dir: &Path, vault_name: &str) -> Result<LockEntry, VaultError> {
    let path = lock_path(base_dir, vault_name);
    let content = fs::read_to_string(&path).map_err(|e| VaultError::Io(e.to_string()))?;
    LockEntry::from_toml_str(&content)
}
