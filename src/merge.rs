use crate::bundle::Bundle;
use crate::error::VaultKeyError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum MergeStrategy {
    KeepSource,
    KeepTarget,
    FailOnConflict,
}

impl Default for MergeStrategy {
    fn default() -> Self {
        MergeStrategy::FailOnConflict
    }
}

#[derive(Debug, Default)]
pub struct MergeResult {
    pub merged: Bundle,
    pub conflicts: Vec<String>,
    pub added: Vec<String>,
    pub overwritten: Vec<String>,
}

pub fn merge_bundles(
    target: &Bundle,
    source: &Bundle,
    strategy: &MergeStrategy,
) -> Result<MergeResult, VaultKeyError> {
    let mut merged_secrets: HashMap<String, String> = target.secrets.clone();
    let mut conflicts = Vec::new();
    let mut added = Vec::new();
    let mut overwritten = Vec::new();

    for (key, value) in &source.secrets {
        if merged_secrets.contains_key(key) {
            match strategy {
                MergeStrategy::FailOnConflict => {
                    conflicts.push(key.clone());
                }
                MergeStrategy::KeepSource => {
                    merged_secrets.insert(key.clone(), value.clone());
                    overwritten.push(key.clone());
                }
                MergeStrategy::KeepTarget => {
                    // keep existing, do nothing
                }
            }
        } else {
            merged_secrets.insert(key.clone(), value.clone());
            added.push(key.clone());
        }
    }

    if !conflicts.is_empty() {
        return Err(VaultKeyError::Config(format!(
            "Merge conflicts on keys: {}",
            conflicts.join(", ")
        )));
    }

    let mut merged_bundle = target.clone();
    merged_bundle.secrets = merged_secrets;

    Ok(MergeResult {
        merged: merged_bundle,
        conflicts,
        added,
        overwritten,
    })
}
