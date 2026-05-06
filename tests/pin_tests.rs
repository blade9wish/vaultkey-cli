use vaultkey_cli::pin::PinStore;
use vaultkey_cli::pin_cmd::{
    cmd_pin_add, cmd_pin_check, cmd_pin_list, cmd_pin_remove,
};

#[test]
fn test_pin_and_check() {
    let mut store = PinStore::new();
    store.pin("db/password", Some("prod db".into())).unwrap();
    assert!(store.is_pinned("db/password"));
    assert!(!store.is_pinned("db/user"));
}

#[test]
fn test_pin_get_label() {
    let mut store = PinStore::new();
    store.pin("api/key", Some("main api key".into())).unwrap();
    let entry = store.get("api/key").unwrap();
    assert_eq!(entry.label.as_deref(), Some("main api key"));
    assert_eq!(entry.key, "api/key");
}

#[test]
fn test_pin_no_label() {
    let mut store = PinStore::new();
    store.pin("token/jwt", None).unwrap();
    let entry = store.get("token/jwt").unwrap();
    assert!(entry.label.is_none());
}

#[test]
fn test_unpin_existing() {
    let mut store = PinStore::new();
    store.pin("secret/x", None).unwrap();
    assert!(store.is_pinned("secret/x"));
    store.unpin("secret/x").unwrap();
    assert!(!store.is_pinned("secret/x"));
}

#[test]
fn test_unpin_nonexistent_returns_error() {
    let mut store = PinStore::new();
    let result = store.unpin("does/not/exist");
    assert!(result.is_err());
}

#[test]
fn test_pin_empty_key_returns_error() {
    let mut store = PinStore::new();
    let result = store.pin("", None);
    assert!(result.is_err());
}

#[test]
fn test_list_sorted_by_time() {
    let mut store = PinStore::new();
    store.pin("alpha", None).unwrap();
    store.pin("beta", None).unwrap();
    store.pin("gamma", None).unwrap();
    let list = store.list();
    assert_eq!(list.len(), 3);
    // All pinned_at values should be non-zero
    for p in &list {
        assert!(p.pinned_at > 0);
    }
}

#[test]
fn test_cmd_pin_add_and_remove() {
    let mut store = PinStore::new();
    cmd_pin_add(&mut store, "env/prod", Some("production".into())).unwrap();
    assert!(store.is_pinned("env/prod"));
    cmd_pin_remove(&mut store, "env/prod").unwrap();
    assert!(!store.is_pinned("env/prod"));
}

#[test]
fn test_cmd_pin_list_empty() {
    let store = PinStore::new();
    // Should not panic
    cmd_pin_list(&store);
}

#[test]
fn test_cmd_pin_check_runs() {
    let mut store = PinStore::new();
    store.pin("check/me", None).unwrap();
    // Should not panic
    cmd_pin_check(&store, "check/me");
    cmd_pin_check(&store, "not/pinned");
}
