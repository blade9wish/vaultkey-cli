use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::prune::{prune_by_prefix, prune_empty_values, prune_by_keys};
use std::collections::HashMap;

fn make_bundle(entries: Vec<(&str, &str)>) -> Bundle {
    let mut bundle = Bundle::new("test-bundle".to_string());
    for (k, v) in entries {
        bundle.secrets.insert(k.to_string(), v.to_string());
    }
    bundle
}

#[test]
fn test_prune_by_prefix_removes_matching() {
    let mut bundle = make_bundle(vec![
        ("dev/db_pass", "secret1"),
        ("dev/api_key", "secret2"),
        ("prod/db_pass", "secret3"),
    ]);
    let result = prune_by_prefix(&mut bundle, "dev/", false).unwrap();
    assert_eq!(result.removed_keys.len(), 2);
    assert_eq!(result.kept_keys.len(), 1);
    assert!(!bundle.secrets.contains_key("dev/db_pass"));
    assert!(!bundle.secrets.contains_key("dev/api_key"));
    assert!(bundle.secrets.contains_key("prod/db_pass"));
}

#[test]
fn test_prune_by_prefix_dry_run_does_not_modify() {
    let mut bundle = make_bundle(vec![
        ("dev/db_pass", "secret1"),
        ("prod/db_pass", "secret2"),
    ]);
    let result = prune_by_prefix(&mut bundle, "dev/", true).unwrap();
    assert_eq!(result.removed_keys.len(), 1);
    assert!(result.dry_run);
    // Bundle should be unchanged
    assert!(bundle.secrets.contains_key("dev/db_pass"));
}

#[test]
fn test_prune_empty_values_removes_blank() {
    let mut bundle = make_bundle(vec![
        ("key1", ""),
        ("key2", "   "),
        ("key3", "real_value"),
    ]);
    let result = prune_empty_values(&mut bundle, false).unwrap();
    assert_eq!(result.removed_keys.len(), 2);
    assert_eq!(result.kept_keys.len(), 1);
    assert!(!bundle.secrets.contains_key("key1"));
    assert!(!bundle.secrets.contains_key("key2"));
    assert!(bundle.secrets.contains_key("key3"));
}

#[test]
fn test_prune_empty_values_dry_run() {
    let mut bundle = make_bundle(vec![
        ("empty_key", ""),
        ("full_key", "value"),
    ]);
    let result = prune_empty_values(&mut bundle, true).unwrap();
    assert!(result.dry_run);
    assert_eq!(result.removed_keys.len(), 1);
    assert!(bundle.secrets.contains_key("empty_key"));
}

#[test]
fn test_prune_by_keys_explicit_list() {
    let mut bundle = make_bundle(vec![
        ("alpha", "1"),
        ("beta", "2"),
        ("gamma", "3"),
    ]);
    let keys_to_remove = vec!["alpha".to_string(), "gamma".to_string()];
    let result = prune_by_keys(&mut bundle, &keys_to_remove, false).unwrap();
    assert_eq!(result.removed_keys.len(), 2);
    assert_eq!(result.kept_keys.len(), 1);
    assert!(!bundle.secrets.contains_key("alpha"));
    assert!(bundle.secrets.contains_key("beta"));
}

#[test]
fn test_prune_summary_format() {
    let mut bundle = make_bundle(vec![("old/key", "val")]);
    let result = prune_by_prefix(&mut bundle, "old/", false).unwrap();
    let summary = result.summary();
    assert!(summary.contains("Removed"));
    assert!(summary.contains("1"));
}

#[test]
fn test_prune_no_match_returns_empty_removed() {
    let mut bundle = make_bundle(vec![("prod/key", "value")]);
    let result = prune_by_prefix(&mut bundle, "dev/", false).unwrap();
    assert_eq!(result.removed_keys.len(), 0);
    assert_eq!(result.kept_keys.len(), 1);
}
