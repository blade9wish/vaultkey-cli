use vaultkey_cli::alias::AliasMap;

#[test]
fn test_add_and_resolve_alias() {
    let mut map = AliasMap::new();
    map.add("prod", "production/secrets").unwrap();
    assert_eq!(map.resolve("prod"), Some(&"production/secrets".to_string()));
}

#[test]
fn test_add_duplicate_alias_fails() {
    let mut map = AliasMap::new();
    map.add("prod", "production/secrets").unwrap();
    let result = map.add("prod", "other/path");
    assert!(result.is_err());
}

#[test]
fn test_add_empty_alias_fails() {
    let mut map = AliasMap::new();
    assert!(map.add("", "target").is_err());
    assert!(map.add("alias", "").is_err());
}

#[test]
fn test_remove_alias() {
    let mut map = AliasMap::new();
    map.add("dev", "dev/secrets").unwrap();
    map.remove("dev").unwrap();
    assert!(map.resolve("dev").is_none());
}

#[test]
fn test_remove_nonexistent_alias_fails() {
    let mut map = AliasMap::new();
    let result = map.remove("ghost");
    assert!(result.is_err());
}

#[test]
fn test_list_aliases_sorted() {
    let mut map = AliasMap::new();
    map.add("zebra", "z/path").unwrap();
    map.add("alpha", "a/path").unwrap();
    map.add("middle", "m/path").unwrap();
    let list = map.list();
    let keys: Vec<&str> = list.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(keys, vec!["alpha", "middle", "zebra"]);
}

#[test]
fn test_resolve_missing_returns_none() {
    let map = AliasMap::new();
    assert!(map.resolve("missing").is_none());
}

#[test]
fn test_rename_alias() {
    let mut map = AliasMap::new();
    map.add("old", "some/target").unwrap();
    map.rename("old", "new").unwrap();
    assert!(map.resolve("old").is_none());
    assert_eq!(map.resolve("new"), Some(&"some/target".to_string()));
}

#[test]
fn test_rename_to_existing_fails() {
    let mut map = AliasMap::new();
    map.add("a", "path/a").unwrap();
    map.add("b", "path/b").unwrap();
    let result = map.rename("a", "b");
    assert!(result.is_err());
    // original should still be intact
    assert_eq!(map.resolve("a"), Some(&"path/a".to_string()));
}

#[test]
fn test_rename_nonexistent_fails() {
    let mut map = AliasMap::new();
    assert!(map.rename("ghost", "new").is_err());
}

#[test]
fn test_multiple_aliases_independent() {
    let mut map = AliasMap::new();
    map.add("prod", "prod/vault").unwrap();
    map.add("staging", "staging/vault").unwrap();
    map.add("dev", "dev/vault").unwrap();
    assert_eq!(map.list().len(), 3);
    map.remove("staging").unwrap();
    assert_eq!(map.list().len(), 2);
    assert!(map.resolve("staging").is_none());
}
