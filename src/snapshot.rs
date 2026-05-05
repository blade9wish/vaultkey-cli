use std::collections::HashMap;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::error::VaultError;
use crate::bundle::Bundle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub entries: HashMap<String, String>,
}

impl Snapshot {
    pub fn new(bundle: &Bundle, label: Option<String>) -> Self {
        let id = format!("{}", Utc::now().timestamp());
        Snapshot {
            id,
            label,
            created_at: Utc::now(),
            entries: bundle.entries.clone(),
        }
    }

    pub fn diff_from(&self, other: &Snapshot) -> HashMap<String, SnapshotDiff> {
        let mut changes = HashMap::new();

        for (key, val) in &self.entries {
            match other.entries.get(key) {
                None => {
                    changes.insert(key.clone(), SnapshotDiff::Added(val.clone()));
                }
                Some(old_val) if old_val != val => {
                    changes.insert(
                        key.clone(),
                        SnapshotDiff::Modified(old_val.clone(), val.clone()),
                    );
                }
                _ => {}
            }
        }

        for key in other.entries.keys() {
            if !self.entries.contains_key(key) {
                changes.insert(key.clone(), SnapshotDiff::Removed);
            }
        }

        changes
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotDiff {
    Added(String),
    Removed,
    Modified(String, String),
}

pub fn save_snapshot(snapshot: &Snapshot, dir: &Path) -> Result<(), VaultError> {
    std::fs::create_dir_all(dir)?;
    let filename = format!("{}.json", snapshot.id);
    let path = dir.join(filename);
    let data = serde_json::to_string_pretty(snapshot)
        .map_err(|e| VaultError::SerializationError(e.to_string()))?;
    std::fs::write(&path, data)?;
    Ok(())
}

pub fn load_snapshots(dir: &Path) -> Result<Vec<Snapshot>, VaultError> {
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut snapshots = vec![];
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if entry.path().extension().and_then(|e| e.to_str()) == Some("json") {
            let data = std::fs::read_to_string(entry.path())?;
            let snap: Snapshot = serde_json::from_str(&data)
                .map_err(|e| VaultError::SerializationError(e.to_string()))?;
            snapshots.push(snap);
        }
    }
    snapshots.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    Ok(snapshots)
}
