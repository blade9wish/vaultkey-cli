use std::collections::HashMap;
use vaultkey_cli::diff::{diff_bundles, DiffResult};

fn make_map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
test_diff_added_keys() {
    let a = make_map(&[("key1", "val1")]);
    let b = make_map(&[("key1", "val1"), ("key2", "val2")]);
    let results = diff_bundles(&a, &b);
    assert_eq!(results.len(), 1);
    assert!(matches!(&results[0], DiffResult::Added(k, _) if k == "key2"));
}

#[test]
fn test_diff_removed_keys() {
    let a = make_map(&[("key1", "val1"), ("key2", "val2")]);
    let b = make_map(&[("key1", "val1")]);
    let results = diff_bundles(&a, &b);
    assert_eq!(results.len(), 1);
    assert!(matches!(&results[0], DiffResult::Removed(k, _) if k == "key2"));
}

#[test]
fn test_diff_changed_keys() {
    let a = make_map(&[("key1", "old_val")]);
    let b = make_map(&[("key1", "new_val")]);
    let results = diff_bundles(&a, &b);
    assert_eq!(results.len(), 1);
    assert!(matches!(&results[0], DiffResult::Changed(k, old, new)
        if k == "key1" && old == "old_val" && new == "new_val"));
}

#[test]
fn test_diff_no_changes() {
    let a = make_map(&[("key1", "val1"), ("key2", "val2")]);
    let b = make_map(&[("key1", "val1"), ("key2", "val2")]);
    let results = diff_bundles(&a, &b);
    assert!(results.is_empty());
}

#[test]
fn test_diff_mixed_changes() {
    let a = make_map(&[("key1", "val1"), ("key2", "old"), ("key3", "val3")]);
    let b = make_map(&[("key1", "val1"), ("key2", "new"), ("key4", "val4")]);
    let results = diff_bundles(&a, &b);
    assert_eq!(results.len(), 3);

    let added: Vec<_> = results.iter().filter(|r| matches!(r, DiffResult::Added(_, _))).collect();
    let removed: Vec<_> = results.iter().filter(|r| matches!(r, DiffResult::Removed(_, _))).collect();
    let changed: Vec<_> = results.iter().filter(|r| matches!(r, DiffResult::Changed(_, _, _))).collect();

    assert_eq!(added.len(), 1);
    assert_eq!(removed.len(), 1);
    assert_eq!(changed.len(), 1);
}

#[test]
fn test_diff_empty_vaults() {
    let a: HashMap<String, String> = HashMap::new();
    let b: HashMap<String, String> = HashMap::new();
    let results = diff_bundles(&a, &b);
    assert!(results.is_empty());
}
