use chrono::{Duration, Utc};
use vaultkey_cli::expire::{ExpireEntry, ExpireRegistry};

#[test]
test_expire_entry_not_expired() {
    let expires_at = Utc::now() + Duration::seconds(300);
    let entry = ExpireEntry::new("db_pass", expires_at, None);
    assert!(!entry.is_expired());
    assert!(entry.seconds_remaining() > 0);
}

#[test]
fn test_expire_entry_is_expired() {
    let expires_at = Utc::now() - Duration::seconds(10);
    let entry = ExpireEntry::new("old_key", expires_at, None);
    assert!(entry.is_expired());
    assert_eq!(entry.seconds_remaining(), 0);
}

#[test]
fn test_expire_entry_expiring_soon() {
    let expires_at = Utc::now() + Duration::seconds(30);
    let entry = ExpireEntry::new("api_key", expires_at, Some(60));
    assert!(entry.is_expiring_soon());
    assert!(!entry.is_expired());
}

#[test]
fn test_expire_entry_not_expiring_soon_if_threshold_not_reached() {
    let expires_at = Utc::now() + Duration::seconds(200);
    let entry = ExpireEntry::new("token", expires_at, Some(60));
    assert!(!entry.is_expiring_soon());
}

#[test]
fn test_registry_set_and_get() {
    let mut registry = ExpireRegistry::new();
    let expires_at = Utc::now() + Duration::seconds(100);
    let entry = ExpireEntry::new("secret", expires_at, None);
    registry.set(entry);
    assert!(registry.get("secret").is_some());
    assert!(registry.get("missing").is_none());
}

#[test]
fn test_registry_remove_existing() {
    let mut registry = ExpireRegistry::new();
    let expires_at = Utc::now() + Duration::seconds(100);
    registry.set(ExpireEntry::new("key1", expires_at, None));
    assert!(registry.remove("key1").is_ok());
    assert!(registry.get("key1").is_none());
}

#[test]
fn test_registry_remove_missing() {
    let mut registry = ExpireRegistry::new();
    assert!(registry.remove("ghost").is_err());
}

#[test]
fn test_registry_expired_keys() {
    let mut registry = ExpireRegistry::new();
    registry.set(ExpireEntry::new("old", Utc::now() - Duration::seconds(5), None));
    registry.set(ExpireEntry::new("fresh", Utc::now() + Duration::seconds(500), None));
    let expired = registry.expired_keys();
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0].key, "old");
}

#[test]
fn test_registry_purge_expired() {
    let mut registry = ExpireRegistry::new();
    registry.set(ExpireEntry::new("stale", Utc::now() - Duration::seconds(1), None));
    registry.set(ExpireEntry::new("live", Utc::now() + Duration::seconds(300), None));
    let purged = registry.purge_expired();
    assert_eq!(purged.len(), 1);
    assert_eq!(purged[0], "stale");
    assert!(registry.get("live").is_some());
    assert!(registry.get("stale").is_none());
}

#[test]
fn test_registry_expiring_soon_keys() {
    let mut registry = ExpireRegistry::new();
    registry.set(ExpireEntry::new("soon", Utc::now() + Duration::seconds(20), Some(60)));
    registry.set(ExpireEntry::new("later", Utc::now() + Duration::seconds(3600), Some(60)));
    let soon = registry.expiring_soon_keys();
    assert_eq!(soon.len(), 1);
    assert_eq!(soon[0].key, "soon");
}
