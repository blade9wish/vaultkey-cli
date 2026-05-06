use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::merge::{merge_bundles, MergeStrategy};
use std::collections::HashMap;

fn make_bundle(secrets: &[(&str, &str)]) -> Bundle {
    let mut b = Bundle::default();
    for (k, v) in secrets {
        b.secrets.insert(k.to_string(), v.to_string());
    }
    b
}

#[test]
fn test_merge_no_conflicts() {
    let target = make_bundle(&[("key1", "val1")]);
    let source = make_bundle(&[("key2", "val2")]);
    let result = merge_bundles(&target, &source, &MergeStrategy::FailOnConflict).unwrap();
    assert_eq!(result.merged.secrets.get("key1").unwrap(), "val1");
    assert_eq!(result.merged.secrets.get("key2").unwrap(), "val2");
    assert_eq!(result.added, vec!["key2".to_string()]);
    assert!(result.overwritten.is_empty());
}

#[test]
fn test_merge_conflict_fail_strategy() {
    let target = make_bundle(&[("shared", "from_target")]);
    let source = make_bundle(&[("shared", "from_source")]);
    let result = merge_bundles(&target, &source, &MergeStrategy::FailOnConflict);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("shared"));
}

#[test]
fn test_merge_keep_source_strategy() {
    let target = make_bundle(&[("shared", "from_target"), ("only_target", "t")]);
    let source = make_bundle(&[("shared", "from_source"), ("only_source", "s")]);
    let result = merge_bundles(&target, &source, &MergeStrategy::KeepSource).unwrap();
    assert_eq!(result.merged.secrets.get("shared").unwrap(), "from_source");
    assert_eq!(result.merged.secrets.get("only_target").unwrap(), "t");
    assert_eq!(result.merged.secrets.get("only_source").unwrap(), "s");
    assert!(result.overwritten.contains(&"shared".to_string()));
}

#[test]
fn test_merge_keep_target_strategy() {
    let target = make_bundle(&[("shared", "from_target")]);
    let source = make_bundle(&[("shared", "from_source"), ("new_key", "new_val")]);
    let result = merge_bundles(&target, &source, &MergeStrategy::KeepTarget).unwrap();
    assert_eq!(result.merged.secrets.get("shared").unwrap(), "from_target");
    assert_eq!(result.merged.secrets.get("new_key").unwrap(), "new_val");
    assert!(result.overwritten.is_empty());
    assert!(result.added.contains(&"new_key".to_string()));
}

#[test]
fn test_merge_empty_source() {
    let target = make_bundle(&[("key1", "val1")]);
    let source = make_bundle(&[]);
    let result = merge_bundles(&target, &source, &MergeStrategy::FailOnConflict).unwrap();
    assert_eq!(result.merged.secrets.len(), 1);
    assert!(result.added.is_empty());
}
