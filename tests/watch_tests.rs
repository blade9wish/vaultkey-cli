use std::path::PathBuf;
use std::time::Duration;
use std::thread;
use vaultkey_cli::watch::{WatchRegistry, compute_checksum};

#[test]
fn test_register_and_list() {
    let mut registry = WatchRegistry::new();
    let path = PathBuf::from("/tmp/secret.toml");
    registry.register("db_pass".to_string(), path.clone(), "abc123".to_string()).unwrap();
    let entries = registry.list();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, "db_pass");
    assert_eq!(entries[0].checksum, "abc123");
}

#[test]
fn test_unregister_existing() {
    let mut registry = WatchRegistry::new();
    registry.register("api_key".to_string(), PathBuf::from("/tmp/api.toml"), "xyz".to_string()).unwrap();
    assert!(registry.unregister("api_key").is_ok());
    assert_eq!(registry.list().len(), 0);
}

#[test]
fn test_unregister_missing_returns_error() {
    let mut registry = WatchRegistry::new();
    let result = registry.unregister("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_check_changed_same_checksum() {
    let mut registry = WatchRegistry::new();
    registry.register("token".to_string(), PathBuf::from("/tmp/token.toml"), "hash1".to_string()).unwrap();
    let changed = registry.check_changed("token", "hash1").unwrap();
    assert!(!changed);
}

#[test]
fn test_check_changed_different_checksum() {
    let mut registry = WatchRegistry::new();
    registry.register("token".to_string(), PathBuf::from("/tmp/token.toml"), "hash1".to_string()).unwrap();
    let changed = registry.check_changed("token", "hash2").unwrap();
    assert!(changed);
}

#[test]
fn test_check_changed_missing_key() {
    let registry = WatchRegistry::new();
    assert!(registry.check_changed("ghost", "anything").is_err());
}

#[test]
fn test_update_checksum() {
    let mut registry = WatchRegistry::new();
    registry.register("cert".to_string(), PathBuf::from("/tmp/cert.pem"), "old".to_string()).unwrap();
    registry.update_checksum("cert", "new".to_string()).unwrap();
    assert!(!registry.check_changed("cert", "new").unwrap());
    assert!(registry.check_changed("cert", "old").unwrap());
}

#[test]
fn test_stale_entries() {
    let mut registry = WatchRegistry::new();
    registry.register("old_key".to_string(), PathBuf::from("/tmp/old.toml"), "v1".to_string()).unwrap();
    thread::sleep(Duration::from_millis(50));
    let stale = registry.stale_entries(Duration::from_millis(10));
    assert_eq!(stale.len(), 1);
    assert_eq!(stale[0].key, "old_key");
}

#[test]
fn test_compute_checksum_deterministic() {
    let c1 = compute_checksum("hello world");
    let c2 = compute_checksum("hello world");
    assert_eq!(c1, c2);
}

#[test]
fn test_compute_checksum_differs_on_different_input() {
    let c1 = compute_checksum("secret_a");
    let c2 = compute_checksum("secret_b");
    assert_ne!(c1, c2);
}
