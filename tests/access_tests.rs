use vaultkey_cli::access::{AccessPolicy, AccessRule, Permission, parse_permissions};
use vaultkey_cli::access_store::AccessStore;
use tempfile::tempdir;

fn make_rule(identity: &str, pattern: &str, perms: Vec<Permission>) -> AccessRule {
    AccessRule {
        identity: identity.to_string(),
        bundle_pattern: pattern.to_string(),
        permissions: perms,
    }
}

#[test]
fn test_add_and_check_permission() {
    let mut policy = AccessPolicy::new();
    policy.add_rule(make_rule("alice", "prod-*", vec![Permission::Read, Permission::Write]));
    assert!(policy.check("alice", "prod-secrets", &Permission::Read));
    assert!(policy.check("alice", "prod-secrets", &Permission::Write));
    assert!(!policy.check("alice", "prod-secrets", &Permission::Delete));
}

#[test]
fn test_wildcard_pattern_all() {
    let mut policy = AccessPolicy::new();
    policy.add_rule(make_rule("admin", "*", vec![Permission::Admin]));
    assert!(policy.check("admin", "any-bundle", &Permission::Admin));
    assert!(policy.check("admin", "another", &Permission::Admin));
}

#[test]
fn test_exact_pattern() {
    let mut policy = AccessPolicy::new();
    policy.add_rule(make_rule("bob", "staging", vec![Permission::Read]));
    assert!(policy.check("bob", "staging", &Permission::Read));
    assert!(!policy.check("bob", "staging-extra", &Permission::Read));
}

#[test]
fn test_remove_rule() {
    let mut policy = AccessPolicy::new();
    policy.add_rule(make_rule("carol", "dev-*", vec![Permission::Write]));
    assert!(policy.remove_rule("carol", "dev-*"));
    assert!(!policy.check("carol", "dev-app", &Permission::Write));
    assert!(!policy.remove_rule("carol", "dev-*"));
}

#[test]
fn test_list_for_identity() {
    let mut policy = AccessPolicy::new();
    policy.add_rule(make_rule("dave", "prod-*", vec![Permission::Read]));
    policy.add_rule(make_rule("dave", "staging", vec![Permission::Write]));
    policy.add_rule(make_rule("eve", "*", vec![Permission::Admin]));
    let dave_rules = policy.list_for_identity("dave");
    assert_eq!(dave_rules.len(), 2);
}

#[test]
fn test_summary() {
    let mut policy = AccessPolicy::new();
    policy.add_rule(make_rule("frank", "prod-*", vec![Permission::Read]));
    policy.add_rule(make_rule("frank", "dev-*", vec![Permission::Write]));
    let summary = policy.summary();
    assert_eq!(summary["frank"].len(), 2);
}

#[test]
fn test_parse_permissions_valid() {
    let perms = parse_permissions(&["read", "write", "delete", "admin"]).unwrap();
    assert_eq!(perms.len(), 4);
    assert!(perms.contains(&Permission::Admin));
}

#[test]
fn test_parse_permissions_invalid() {
    let result = parse_permissions(&["superuser"]);
    assert!(result.is_err());
}

#[test]
fn test_store_roundtrip() {
    let dir = tempdir().unwrap();
    let store = AccessStore::new(dir.path().join("access.toml"));
    let mut policy = AccessPolicy::new();
    policy.add_rule(make_rule("grace", "vault-*", vec![Permission::Read]));
    store.save(&policy).unwrap();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.rules.len(), 1);
    assert!(loaded.check("grace", "vault-main", &Permission::Read));
}

#[test]
fn test_store_empty_if_missing() {
    let dir = tempdir().unwrap();
    let store = AccessStore::new(dir.path().join("nonexistent.toml"));
    let policy = store.load().unwrap();
    assert!(policy.rules.is_empty());
}
