use std::fs;
use tempfile::tempdir;
use vaultkey_cli::lock::{
    get_lock_entry, is_locked, lock_vault, unlock_vault, LockEntry,
};

#[test]
fn test_lock_and_unlock() {
    let dir = tempdir().unwrap();
    let base = dir.path();
    assert!(!is_locked(base, "myvault"));
    lock_vault(base, "myvault", None).expect("lock should succeed");
    assert!(is_locked(base, "myvault"));
    unlock_vault(base, "myvault").expect("unlock should succeed");
    assert!(!is_locked(base, "myvault"));
}

#[test]
fn test_lock_with_reason() {
    let dir = tempdir().unwrap();
    let base = dir.path();
    lock_vault(base, "vault1", Some("maintenance".to_string())).unwrap();
    let entry = get_lock_entry(base, "vault1").unwrap();
    assert_eq!(entry.vault_name, "vault1");
    assert_eq!(entry.reason, Some("maintenance".to_string()));
    assert!(entry.locked_at > 0);
}

#[test]
fn test_double_lock_fails() {
    let dir = tempdir().unwrap();
    let base = dir.path();
    lock_vault(base, "vault2", None).unwrap();
    let result = lock_vault(base, "vault2", None);
    assert!(result.is_err());
}

#[test]
fn test_unlock_not_locked_fails() {
    let dir = tempdir().unwrap();
    let base = dir.path();
    let result = unlock_vault(base, "nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_lock_entry_serialization() {
    let entry = LockEntry::new("testvault", Some("deploy freeze".to_string()));
    let serialized = entry.to_toml_string();
    assert!(serialized.contains("testvault"));
    assert!(serialized.contains("deploy freeze"));
    let parsed = LockEntry::from_toml_str(&serialized).unwrap();
    assert_eq!(parsed.vault_name, "testvault");
    assert_eq!(parsed.reason, Some("deploy freeze".to_string()));
}

#[test]
fn test_lock_entry_no_reason_serialization() {
    let entry = LockEntry::new("noreason", None);
    let serialized = entry.to_toml_string();
    let parsed = LockEntry::from_toml_str(&serialized).unwrap();
    assert_eq!(parsed.vault_name, "noreason");
    assert!(parsed.reason.is_none());
}

#[test]
fn test_get_lock_entry_not_found() {
    let dir = tempdir().unwrap();
    let result = get_lock_entry(dir.path(), "missing");
    assert!(result.is_err());
}

#[test]
fn test_multiple_vaults_locked_independently() {
    let dir = tempdir().unwrap();
    let base = dir.path();
    lock_vault(base, "alpha", None).unwrap();
    lock_vault(base, "beta", Some("review".to_string())).unwrap();
    assert!(is_locked(base, "alpha"));
    assert!(is_locked(base, "beta"));
    unlock_vault(base, "alpha").unwrap();
    assert!(!is_locked(base, "alpha"));
    assert!(is_locked(base, "beta"));
}
