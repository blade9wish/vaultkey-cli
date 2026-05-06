use std::path::PathBuf;
use crate::backup::{create_backup, list_backups, restore_backup, prune_backups};
use crate::error::VaultError;

pub fn cmd_backup_create(vault_path: &str, backup_dir: &str, label: Option<&str>) -> Result<(), VaultError> {
    let vp = PathBuf::from(vault_path);
    let bd = PathBuf::from(backup_dir);
    let entry = create_backup(&vp, &bd, label)?;
    println!("Backup created: {}", entry.path.display());
    if let Some(l) = &entry.label {
        println!("  Label     : {}", l);
    }
    println!("  Timestamp : {}", entry.created_at);
    Ok(())
}

pub fn cmd_backup_list(backup_dir: &str) -> Result<(), VaultError> {
    let bd = PathBuf::from(backup_dir);
    let entries = list_backups(&bd)?;
    if entries.is_empty() {
        println!("No backups found in '{}'.", backup_dir);
        return Ok(());
    }
    println!("{:<40} {:<20} {}", "File", "Timestamp", "Label");
    println!("{}", "-".repeat(72));
    for e in &entries {
        let fname = e.path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let label = e.label.as_deref().unwrap_or("-");
        println!("{:<40} {:<20} {}", fname, e.created_at, label);
    }
    Ok(())
}

pub fn cmd_backup_restore(backup_path: &str, vault_path: &str) -> Result<(), VaultError> {
    let bp = PathBuf::from(backup_path);
    let vp = PathBuf::from(vault_path);
    restore_backup(&bp, &vp)?;
    println!("Vault restored from: {}", backup_path);
    Ok(())
}

pub fn cmd_backup_prune(backup_dir: &str, keep: usize) -> Result<(), VaultError> {
    let bd = PathBuf::from(backup_dir);
    let removed = prune_backups(&bd, keep)?;
    if removed == 0 {
        println!("Nothing to prune (backups within limit).");
    } else {
        println!("Pruned {} old backup(s), keeping {}.", removed, keep);
    }
    Ok(())
}
