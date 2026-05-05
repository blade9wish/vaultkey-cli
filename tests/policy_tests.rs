use std::collections::HashSet;
use tempfile::NamedTempFile;
use vaultkey_cli::policy::Policy;
use vaultkey_cli::policy_cmd::{cmd_policy_check, cmd_policy_list, cmd_policy_show};
use vaultkey_cli::policy_store::PolicyStore;

fn sample_policy() -> Policy {
    let mut p = Policy::new("strict");
    p.required_tags = vec!["env".to_string()];
    p.forbidden_keys = vec!["password".to_string()];
    p.max_secret_length = Some(64);
    p.allowed_key_prefixes = vec!["app_".to_string(), "db_".to_string()];
    p
}

#[test]
fn test_validate_key_forbidden() {
    let p = sample_policy();
    assert!(p.validate_key("password").is_err());
}

#[test]
fn test_validate_key_allowed_prefix() {
    let p = sample_policy();
    assert!(p.validate_key("app_secret").is_ok());
    assert!(p.validate_key("db_host").is_ok());
    assert!(p.validate_key("other_key").is_err());
}

#[test]
fn test_validate_secret_length() {
    let p = sample_policy();
    assert!(p.validate_secret_length("short").is_ok());
    assert!(p.validate_secret_length(&"x".repeat(65)).is_err());
    assert!(p.validate_secret_length(&"x".repeat(64)).is_ok());
}

#[test]
fn test_validate_tags_missing() {
    let p = sample_policy();
    let tags: HashSet<String> = HashSet::new();
    assert!(p.validate_tags(&tags).is_err());
}

#[test]
fn test_validate_tags_present() {
    let p = sample_policy();
    let tags: HashSet<String> = vec!["env".to_string()].into_iter().collect();
    assert!(p.validate_tags(&tags).is_ok());
}

#[test]
fn test_cmd_policy_check_pass() {
    let p = sample_policy();
    let result = cmd_policy_check(&p, "app_token", "abc", &["env".to_string()]);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_policy_check_fail_forbidden_key() {
    let p = sample_policy();
    let result = cmd_policy_check(&p, "password", "secret", &["env".to_string()]);
    assert!(result.is_err());
}

#[test]
fn test_cmd_policy_show_contains_name() {
    let p = sample_policy();
    let output = cmd_policy_show(&p);
    assert!(output.contains("strict"));
    assert!(output.contains("64"));
    assert!(output.contains("app_"));
}

#[test]
fn test_cmd_policy_list_empty() {
    let output = cmd_policy_list(&[]);
    assert!(output.contains("No policies"));
}

#[test]
fn test_policy_store_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let store = PolicyStore::new(tmp.path());
    let p = sample_policy();
    store.upsert(p.clone()).unwrap();
    let loaded = store.find("strict").unwrap();
    assert!(loaded.is_some());
    assert_eq!(loaded.unwrap().name, "strict");
}

#[test]
fn test_policy_store_remove() {
    let tmp = NamedTempFile::new().unwrap();
    let store = PolicyStore::new(tmp.path());
    store.upsert(sample_policy()).unwrap();
    let removed = store.remove("strict").unwrap();
    assert!(removed);
    let all = store.load_all().unwrap();
    assert!(all.is_empty());
}
