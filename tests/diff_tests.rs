use std::collections::HashMap;
vaultkey_cli::bundle::Bundle;
use vaultkey_cli::diff::{diff_bundles, DiffEntry};

fn make_bundle(secrets: &[(&str, &str)]) -> Bundle {
    Bundle {
        name: "test".to_string(),
        secrets: secrets
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    }
}

#[test]
fn test_diff_no_changes() {
    let old = make_bundle(&[("DB_PASS", "secret123"), ("API_KEY", "abc")]);
    let new = make_bundle(&[("DB_PASS", "secret123"), ("API_KEY", "abc")]);
    let diff = diff_bundles(&old, &new);
    assert!(!diff.has_changes());
    assert!(diff.added().is_empty());
    assert!(diff.removed().is_empty());
    assert!(diff.modified().is_empty());
}

#[test]
fn test_diff_added_key() {
    let old = make_bundle(&[("DB_PASS", "secret123")]);
    let new = make_bundle(&[("DB_PASS", "secret123"), ("NEW_KEY", "newval")]);
    let diff = diff_bundles(&old, &new);
    assert!(diff.has_changes());
    assert!(diff.added().contains(&"NEW_KEY"));
    assert!(diff.removed().is_empty());
    assert!(diff.modified().is_empty());
}

#[test]
fn test_diff_removed_key() {
    let old = make_bundle(&[("DB_PASS", "secret123"), ("OLD_KEY", "oldval")]);
    let new = make_bundle(&[("DB_PASS", "secret123")]);
    let diff = diff_bundles(&old, &new);
    assert!(diff.has_changes());
    assert!(diff.removed().contains(&"OLD_KEY"));
    assert!(diff.added().is_empty());
}

#[test]
fn test_diff_modified_key() {
    let old = make_bundle(&[("DB_PASS", "oldpass")]);
    let new = make_bundle(&[("DB_PASS", "newpass")]);
    let diff = diff_bundles(&old, &new);
    assert!(diff.has_changes());
    assert!(diff.modified().contains(&"DB_PASS"));
    match diff.entries.get("DB_PASS") {
        Some(DiffEntry::Modified { old, new }) => {
            assert_eq!(old, "oldpass");
            assert_eq!(new, "newpass");
        }
        _ => panic!("Expected Modified entry for DB_PASS"),
    }
}

#[test]
fn test_diff_mixed_changes() {
    let old = make_bundle(&[("A", "1"), ("B", "2"), ("C", "3")]);
    let new = make_bundle(&[("A", "1"), ("B", "changed"), ("D", "4")]);
    let diff = diff_bundles(&old, &new);
    assert!(diff.has_changes());
    assert!(diff.added().contains(&"D"));
    assert!(diff.removed().contains(&"C"));
    assert!(diff.modified().contains(&"B"));
    assert!(!diff.modified().contains(&"A"));
}
