use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;
use crate::error::VaultError;

#[derive(Debug, Clone)]
pub struct BackupEntry {
    pub path: PathBuf,
    pub created_at: String,
    pub label: Option<String>,
}

pub fn create_backup(vault_path: &Path, backup_dir: &Path, label: Option<&str>) -> Result<BackupEntry, VaultError> {
    fs::create_dir_all(backup_dir)
        .map_err(|e| VaultError::Io(e))?;

    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
    let filename = match label {
        Some(l) => format!("backup_{}_{}.toml", l, timestamp),
        None => format!("backup_{}.toml", timestamp),
    };

    let dest = backup_dir.join(&filename);
    fs::copy(vault_path, &dest)
        .map_err(|e| VaultError::Io(e))?;

    Ok(BackupEntry {
        path: dest,
        created_at: timestamp,
        label: label.map(|s| s.to_string()),
    })
}

pub fn list_backups(backup_dir: &Path) -> Result<Vec<BackupEntry>, VaultError> {
    if !backup_dir.exists() {
        return Ok(vec![]);
    }

    let mut entries = vec![];
    for entry in fs::read_dir(backup_dir).map_err(|e| VaultError::Io(e))? {
        let entry = entry.map_err(|e| VaultError::Io(e))?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
            let parts: Vec<&str> = name.splitn(3, '_').collect();
            let created_at = parts.last().unwrap_or(&"").to_string();
            let label = if parts.len() == 3 { Some(parts[1].to_string()) } else { None };
            entries.push(BackupEntry { path, created_at, label });
        }
    }
    entries.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    Ok(entries)
}

pub fn restore_backup(backup_path: &Path, vault_path: &Path) -> Result<(), VaultError> {
    if !backup_path.exists() {
        return Err(VaultError::NotFound(format!("Backup not found: {}", backup_path.display())));
    }
    fs::copy(backup_path, vault_path)
        .map_err(|e| VaultError::Io(e))?;
    Ok(())
}

pub fn prune_backups(backup_dir: &Path, keep: usize) -> Result<usize, VaultError> {
    let mut entries = list_backups(backup_dir)?;
    if entries.len() <= keep {
        return Ok(0);
    }
    let to_remove = entries.len() - keep;
    entries.truncate(to_remove);
    for entry in &entries {
        fs::remove_file(&entry.path).map_err(|e| VaultError::Io(e))?;
    }
    Ok(to_remove)
}
