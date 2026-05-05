//! Bundle diff module: compare two secret bundles and report differences.

use crate::bundle::Bundle;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum DiffEntry {
    Added(String),
    Removed(String),
    Modified { old: String, new: String },
    Unchanged,
}

#[derive(Debug)]
pub struct BundleDiff {
    pub entries: HashMap<String, DiffEntry>,
}

impl BundleDiff {
    pub fn has_changes(&self) -> bool {
        self.entries.values().any(|e| !matches!(e, DiffEntry::Unchanged))
    }

    pub fn added(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter_map(|(k, v)| matches!(v, DiffEntry::Added(_)).then_some(k.as_str()))
            .collect()
    }

    pub fn removed(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter_map(|(k, v)| matches!(v, DiffEntry::Removed(_)).then_some(k.as_str()))
            .collect()
    }

    pub fn modified(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter_map(|(k, v)| matches!(v, DiffEntry::Modified { .. }).then_some(k.as_str()))
            .collect()
    }
}

/// Compare two bundles and return a diff of their secrets.
pub fn diff_bundles(old: &Bundle, new: &Bundle) -> BundleDiff {
    let mut entries: HashMap<String, DiffEntry> = HashMap::new();

    for (key, old_val) in &old.secrets {
        match new.secrets.get(key) {
            Some(new_val) if new_val == old_val => {
                entries.insert(key.clone(), DiffEntry::Unchanged);
            }
            Some(new_val) => {
                entries.insert(
                    key.clone(),
                    DiffEntry::Modified {
                        old: old_val.clone(),
                        new: new_val.clone(),
                    },
                );
            }
            None => {
                entries.insert(key.clone(), DiffEntry::Removed(old_val.clone()));
            }
        }
    }

    for (key, new_val) in &new.secrets {
        if !old.secrets.contains_key(key) {
            entries.insert(key.clone(), DiffEntry::Added(new_val.clone()));
        }
    }

    BundleDiff { entries }
}
