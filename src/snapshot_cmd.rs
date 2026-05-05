use std::path::PathBuf;
use crate::error::VaultError;
use crate::bundle::Bundle;
use crate::snapshot::{Snapshot, load_snapshots, save_snapshot};

pub fn cmd_snapshot_create(
    bundle: &Bundle,
    label: Option<String>,
    snapshot_dir: &PathBuf,
) -> Result<(), VaultError> {
    let snap = Snapshot::new(bundle, label.clone());
    save_snapshot(&snap, snapshot_dir)?;
    println!(
        "Snapshot '{}' created{}",
        snap.id,
        label.map(|l| format!(" [{}]", l)).unwrap_or_default()
    );
    Ok(())
}

pub fn cmd_snapshot_list(snapshot_dir: &PathBuf) -> Result<(), VaultError> {
    let snapshots = load_snapshots(snapshot_dir)?;
    if snapshots.is_empty() {
        println!("No snapshots found.");
        return Ok(());
    }
    println!("{:<20} {:<25} {}", "ID", "Created At", "Label");
    println!("{}", "-".repeat(60));
    for snap in &snapshots {
        println!(
            "{:<20} {:<25} {}",
            snap.id,
            snap.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
            snap.label.as_deref().unwrap_or("-")
        );
    }
    Ok(())
}

pub fn cmd_snapshot_diff(
    snapshot_dir: &PathBuf,
    id_a: &str,
    id_b: &str,
) -> Result<(), VaultError> {
    let snapshots = load_snapshots(snapshot_dir)?;
    let find = |id: &str| {
        snapshots
            .iter()
            .find(|s| s.id == id)
            .cloned()
            .ok_or_else(|| VaultError::NotFound(format!("Snapshot '{}' not found", id)))
    };
    let snap_a = find(id_a)?;
    let snap_b = find(id_b)?;
    let diffs = snap_a.diff_from(&snap_b);
    if diffs.is_empty() {
        println!("No differences between snapshots.");
        return Ok(());
    }
    println!("Diff: {} → {}", id_b, id_a);
    println!("{}", "-".repeat(40));
    let mut keys: Vec<_> = diffs.keys().collect();
    keys.sort();
    for key in keys {
        match &diffs[key] {
            crate::snapshot::SnapshotDiff::Added(v) => println!("+ {}: {}", key, v),
            crate::snapshot::SnapshotDiff::Removed => println!("- {}: <removed>", key),
            crate::snapshot::SnapshotDiff::Modified(old, new) => {
                println!("~ {}: {} → {}", key, old, new)
            }
        }
    }
    Ok(())
}
