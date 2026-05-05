use vaultkey_cli::ttl::{TtlEntry, TtlStore};
use std::thread;
use std::time::Duration;

#[test]
test_ttl_entry_not_expired_immediately() {
    let entry = TtlEntry::new("db_password", 3600).unwrap();
    assert!(!entry.is_expired());
    assert!(entry.seconds_remaining() > 0);
}

#[test]
fn test_ttl_entry_expired() {
    // expires in 0 seconds — should be immediately expired
    let entry = TtlEntry::new("old_key", 0).unwrap();
    // Give the clock a moment
    thread::sleep(Duration::from_millis(10));
    assert!(entry.is_expired());
    assert_eq!(entry.seconds_remaining(), 0);
}

#[test]
fn test_ttl_store_set_and_get() {
    let mut store = TtlStore::new();
    store.set("api_key", 300).unwrap();
    let entry = store.get("api_key");
    assert!(entry.is_some());
    assert_eq!(entry.unwrap().key, "api_key");
}

#[test]
fn test_ttl_store_overwrite() {
    let mut store = TtlStore::new();
    store.set("token", 100).unwrap();
    store.set("token", 9999).unwrap();
    assert_eq!(store.entries.len(), 1);
    assert!(store.get("token").unwrap().seconds_remaining() > 100);
}

#[test]
fn test_ttl_store_remove_existing() {
    let mut store = TtlStore::new();
    store.set("secret", 500).unwrap();
    let removed = store.remove("secret");
    assert!(removed);
    assert!(store.get("secret").is_none());
}

#[test]
fn test_ttl_store_remove_nonexistent() {
    let mut store = TtlStore::new();
    let removed = store.remove("ghost");
    assert!(!removed);
}

#[test]
fn test_ttl_store_expired_keys() {
    let mut store = TtlStore::new();
    store.set("live_key", 3600).unwrap();
    store.set("dead_key", 0).unwrap();
    thread::sleep(Duration::from_millis(10));
    let expired = store.expired_keys();
    assert!(expired.contains(&"dead_key".to_string()));
    assert!(!expired.contains(&"live_key".to_string()));
}

#[test]
fn test_ttl_store_purge_expired() {
    let mut store = TtlStore::new();
    store.set("keep", 3600).unwrap();
    store.set("remove_me", 0).unwrap();
    thread::sleep(Duration::from_millis(10));
    let purged = store.purge_expired();
    assert_eq!(purged.len(), 1);
    assert_eq!(purged[0], "remove_me");
    assert_eq!(store.entries.len(), 1);
    assert!(store.get("keep").is_some());
}

#[test]
fn test_ttl_store_empty_purge() {
    let mut store = TtlStore::new();
    store.set("active", 9999).unwrap();
    let purged = store.purge_expired();
    assert!(purged.is_empty());
    assert_eq!(store.entries.len(), 1);
}
