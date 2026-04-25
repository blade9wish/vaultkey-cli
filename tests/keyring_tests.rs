use vaultkey_cli::keyring::{KeyEntry, Keyring};
use vaultkey_cli::keyring_store::KeyringStore;
use vaultkey_cli::keyring_cmd::{cmd_add_key, cmd_remove_key, cmd_list_keys};
use tempfile::tempdir;

#[test]
fn test_keyentry_builder() {
    let entry = KeyEntry::new("alice", "ABCD1234")
        .with_email("alice@example.com")
        .with_description("Alice's signing key");
    assert_eq!(entry.alias, "alice");
    assert_eq!(entry.fingerprint, "ABCD1234");
    assert_eq!(entry.email.as_deref(), Some("alice@example.com"));
    assert_eq!(entry.description.as_deref(), Some("Alice's signing key"));
}

#[test]
fn test_keyring_add_and_get() {
    let mut kr = Keyring::new();
    let entry = KeyEntry::new("bob", "DEADBEEF");
    kr.add(entry).unwrap();
    let found = kr.get("bob").unwrap();
    assert_eq!(found.fingerprint, "DEADBEEF");
}

#[test]
fn test_keyring_duplicate_alias_error() {
    let mut kr = Keyring::new();
    kr.add(KeyEntry::new("dup", "AAA111")).unwrap();
    let result = kr.add(KeyEntry::new("dup", "BBB222"));
    assert!(result.is_err());
}

#[test]
fn test_keyring_remove() {
    let mut kr = Keyring::new();
    kr.add(KeyEntry::new("charlie", "C0FFEE")).unwrap();
    assert_eq!(kr.len(), 1);
    kr.remove("charlie").unwrap();
    assert!(kr.is_empty());
}

#[test]
fn test_keyring_remove_nonexistent() {
    let mut kr = Keyring::new();
    let result = kr.remove("ghost");
    assert!(result.is_err());
}

#[test]
fn test_keyring_list_sorted() {
    let mut kr = Keyring::new();
    kr.add(KeyEntry::new("zebra", "ZZZ")).unwrap();
    kr.add(KeyEntry::new("apple", "AAA")).unwrap();
    kr.add(KeyEntry::new("mango", "MMM")).unwrap();
    let list = kr.list();
    assert_eq!(list[0].alias, "apple");
    assert_eq!(list[1].alias, "mango");
    assert_eq!(list[2].alias, "zebra");
}

#[test]
fn test_keyring_store_roundtrip() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("keyring.toml");
    let store = KeyringStore::new(&path);

    cmd_add_key(&store, "dave", "DA7E1234", Some("dave@test.com"), None).unwrap();
    cmd_add_key(&store, "eve", "EEE56789", None, Some("Eve's key")).unwrap();

    let kr = store.load().unwrap();
    assert_eq!(kr.len(), 2);
    assert!(kr.get("dave").is_some());
    assert!(kr.get("eve").is_some());
}

#[test]
fn test_keyring_store_load_empty_if_missing() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("nonexistent.toml");
    let store = KeyringStore::new(&path);
    let kr = store.load().unwrap();
    assert!(kr.is_empty());
}

#[test]
fn test_cmd_remove_key() {
    let dir = tempdir().unwrap();
    let store = KeyringStore::new(dir.path().join("keyring.toml"));
    cmd_add_key(&store, "frank", "FF001122", None, None).unwrap();
    cmd_remove_key(&store, "frank").unwrap();
    let kr = store.load().unwrap();
    assert!(kr.get("frank").is_none());
}
