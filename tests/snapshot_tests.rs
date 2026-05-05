use std::collections::HashMap;
use tempfile::tempdir;
use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::snapshot::{
    load_snapshots, save_snapshot, Snapshot, SnapshotDiff,
};

fn make_bundle(entries: Vec<(&str, &str)>) -> Bundle {
    let mut map = HashMap::new();
    for (k, v) in entries {
        map.insert(k.to_string(), v.to_string());
    }
    Bundle { entries: map }
}

#[test]
fn test_snapshot_creation() {
    let bundle = make_bundle(vec![("key1", "val1"), ("key2", "val2")]);
    let snap = Snapshot::new(&bundle, Some("initial".to_string()));
    assert_eq!(snap.label, Some("initial".to_string()));
    assert_eq!(snap.entries.get("key1"), Some(&"val1".to_string()));
    assert_eq!(snap.entries.get("key2"), Some(&"val2".to_string()));
}

#[test]
fn test_snapshot_no_label() {
    let bundle = make_bundle(vec![("x", "y")]);
    let snap = Snapshot::new(&bundle, None);
    assert!(snap.label.is_none());
}

#[test]
fn test_save_and_load_snapshot() {
    let dir = tempdir().unwrap();
    let bundle = make_bundle(vec![("a", "1")]);
    let snap = Snapshot::new(&bundle, Some("test".to_string()));
    let snap_id = snap.id.clone();

    save_snapshot(&snap, dir.path()).unwrap();
    let loaded = load_snapshots(dir.path()).unwrap();

    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].id, snap_id);
    assert_eq!(loaded[0].label, Some("test".to_string()));
}

#[test]
fn test_load_empty_dir() {
    let dir = tempdir().unwrap();
    let snaps = load_snapshots(dir.path()).unwrap();
    assert!(snaps.is_empty());
}

#[test]
fn test_load_nonexistent_dir() {
    let path = std::path::PathBuf::from("/tmp/vaultkey_nonexistent_snap_dir_xyz");
    let snaps = load_snapshots(&path).unwrap();
    assert!(snaps.is_empty());
}

#[test]
fn test_diff_added_key() {
    let old_bundle = make_bundle(vec![("a", "1")]);
    let new_bundle = make_bundle(vec![("a", "1"), ("b", "2")]);
    let old_snap = Snapshot::new(&old_bundle, None);
    let new_snap = Snapshot::new(&new_bundle, None);
    let diffs = new_snap.diff_from(&old_snap);
    assert!(matches!(diffs.get("b"), Some(SnapshotDiff::Added(_))));
}

#[test]
fn test_diff_removed_key() {
    let old_bundle = make_bundle(vec![("a", "1"), ("b", "2")]);
    let new_bundle = make_bundle(vec![("a", "1")]);
    let old_snap = Snapshot::new(&old_bundle, None);
    let new_snap = Snapshot::new(&new_bundle, None);
    let diffs = new_snap.diff_from(&old_snap);
    assert!(matches!(diffs.get("b"), Some(SnapshotDiff::Removed)));
}

#[test]
fn test_diff_modified_key() {
    let old_bundle = make_bundle(vec![("a", "old_val")]);
    let new_bundle = make_bundle(vec![("a", "new_val")]);
    let old_snap = Snapshot::new(&old_bundle, None);
    let new_snap = Snapshot::new(&new_bundle, None);
    let diffs = new_snap.diff_from(&old_snap);
    assert!(matches!(diffs.get("a"), Some(SnapshotDiff::Modified(_, _))));
}

#[test]
fn test_diff_no_changes() {
    let bundle = make_bundle(vec![("a", "1"), ("b", "2")]);
    let snap1 = Snapshot::new(&bundle, None);
    let snap2 = Snapshot::new(&bundle, None);
    let diffs = snap1.diff_from(&snap2);
    assert!(diffs.is_empty());
}
