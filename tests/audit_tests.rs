use std::fs;
use tempfile::NamedTempFile;
use vaultkey_cli::audit::{AuditAction, AuditLog, make_entry};
use vaultkey_cli::audit_store::AuditStore;

fn temp_log_path() -> String {
    let f = NamedTempFile::new().unwrap();
    let path = f.path().to_string_lossy().to_string();
    drop(f); // release so AuditLog can create it fresh
    path
}

#[test]
fn test_make_entry_fields() {
    let entry = make_entry(AuditAction::BundleCreate, "my_bundle", true, None);
    assert_eq!(entry.target, "my_bundle");
    assert!(entry.success);
    assert!(entry.detail.is_none());
}

#[test]
fn test_record_and_read_single_entry() {
    let path = temp_log_path();
    let log = AuditLog::new(&path);

    let entry = make_entry(AuditAction::SecretAdd, "bundle_a", true, Some("key=foo".into()));
    log.record(entry).unwrap();

    let entries = log.read_entries().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].target, "bundle_a");
    assert!(matches!(entries[0].action, AuditAction::SecretAdd));
    assert_eq!(entries[0].detail.as_deref(), Some("key=foo"));

    fs::remove_file(&path).ok();
}

#[test]
fn test_record_multiple_entries() {
    let path = temp_log_path();
    let log = AuditLog::new(&path);

    for i in 0..5 {
        log.record(make_entry(
            AuditAction::SecretGet,
            format!("bundle_{}", i),
            true,
            None,
        )).unwrap();
    }

    let entries = log.read_entries().unwrap();
    assert_eq!(entries.len(), 5);
    fs::remove_file(&path).ok();
}

#[test]
fn test_read_empty_log() {
    let path = temp_log_path();
    let log = AuditLog::new(&path);
    let entries = log.read_entries().unwrap();
    assert!(entries.is_empty());
}

#[test]
fn test_audit_store_recent() {
    let path = temp_log_path();
    let store = AuditStore::new(&path);

    store.log_bundle_create("alpha", true).unwrap();
    store.log_secret_add("alpha", "db_pass", true).unwrap();
    store.log_secret_get("alpha", "db_pass", true).unwrap();
    store.log_keyring_unlock("main", false).unwrap();

    let all = store.entries().unwrap();
    assert_eq!(all.len(), 4);

    let recent = store.recent(2).unwrap();
    assert_eq!(recent.len(), 2);
    // most recent first
    assert!(matches!(recent[0].action, AuditAction::KeyringUnlock));

    fs::remove_file(&path).ok();
}

#[test]
fn test_audit_store_secret_remove() {
    let path = temp_log_path();
    let store = AuditStore::new(&path);

    store.log_secret_remove("vault", "api_key", true).unwrap();
    let entries = store.entries().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].detail.as_deref(), Some("key=api_key"));
    assert!(matches!(entries[0].action, AuditAction::SecretRemove));

    fs::remove_file(&path).ok();
}
