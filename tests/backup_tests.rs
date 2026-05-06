use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;
use vaultkey_cli::backup::{create_backup, list_backups, restore_backup, prune_backups};

fn make_vault(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
    let p = dir.join(name);
    fs::write(&p, content).unwrap();
    p
}

#[test]
fn test_create_backup_no_label() {
    let tmp = tempdir().unwrap();
    let vault = make_vault(tmp.path(), "vault.toml", "[secrets]\nfoo = \"bar\"");
    let backup_dir = tmp.path().join("backups");
    let entry = create_backup(&vault, &backup_dir, None).unwrap();
    assert!(entry.path.exists());
    assert!(entry.label.is_none());
    assert!(entry.path.to_str().unwrap().contains("backup_"));
}

#[test]
fn test_create_backup_with_label() {
    let tmp = tempdir().unwrap();
    let vault = make_vault(tmp.path(), "vault.toml", "[secrets]");
    let backup_dir = tmp.path().join("backups");
    let entry = create_backup(&vault, &backup_dir, Some("pre-deploy")).unwrap();
    assert!(entry.path.exists());
    assert_eq!(entry.label.as_deref(), Some("pre-deploy"));
    assert!(entry.path.to_str().unwrap().contains("pre-deploy"));
}

#[test]
fn test_list_backups_empty() {
    let tmp = tempdir().unwrap();
    let backup_dir = tmp.path().join("backups");
    let entries = list_backups(&backup_dir).unwrap();
    assert!(entries.is_empty());
}

#[test]
fn test_list_backups_multiple() {
    let tmp = tempdir().unwrap();
    let vault = make_vault(tmp.path(), "vault.toml", "[secrets]");
    let backup_dir = tmp.path().join("backups");
    create_backup(&vault, &backup_dir, Some("alpha")).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    create_backup(&vault, &backup_dir, Some("beta")).unwrap();
    let entries = list_backups(&backup_dir).unwrap();
    assert_eq!(entries.len(), 2);
}

#[test]
fn test_restore_backup() {
    let tmp = tempdir().unwrap();
    let vault = make_vault(tmp.path(), "vault.toml", "original content");
    let backup_dir = tmp.path().join("backups");
    let entry = create_backup(&vault, &backup_dir, None).unwrap();
    fs::write(&vault, "modified content").unwrap();
    restore_backup(&entry.path, &vault).unwrap();
    let restored = fs::read_to_string(&vault).unwrap();
    assert_eq!(restored, "original content");
}

#[test]
fn test_restore_missing_backup_errors() {
    let tmp = tempdir().unwrap();
    let vault = tmp.path().join("vault.toml");
    let missing = tmp.path().join("nonexistent.toml");
    let result = restore_backup(&missing, &vault);
    assert!(result.is_err());
}

#[test]
fn test_prune_backups_keeps_n() {
    let tmp = tempdir().unwrap();
    let vault = make_vault(tmp.path(), "vault.toml", "[secrets]");
    let backup_dir = tmp.path().join("backups");
    for i in 0..5 {
        create_backup(&vault, &backup_dir, Some(&format!("v{}", i))).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let removed = prune_backups(&backup_dir, 3).unwrap();
    assert_eq!(removed, 2);
    let remaining = list_backups(&backup_dir).unwrap();
    assert_eq!(remaining.len(), 3);
}

#[test]
fn test_prune_backups_no_op_when_under_limit() {
    let tmp = tempdir().unwrap();
    let vault = make_vault(tmp.path(), "vault.toml", "[secrets]");
    let backup_dir = tmp.path().join("backups");
    create_backup(&vault, &backup_dir, None).unwrap();
    let removed = prune_backups(&backup_dir, 5).unwrap();
    assert_eq!(removed, 0);
}
