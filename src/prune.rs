use crate::error::VaultError;
use crate::bundle::Bundle;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PruneResult {
    pub removed_keys: Vec<String>,
    pub kept_keys: Vec<String>,
    pub dry_run: bool,
}

impl PruneResult {
    pub fn summary(&self) -> String {
        if self.dry_run {
            format!(
                "[dry-run] Would remove {} key(s), keep {} key(s)",
                self.removed_keys.len(),
                self.kept_keys.len()
            )
        } else {
            format!(
                "Removed {} key(s), kept {} key(s)",
                self.removed_keys.len(),
                self.kept_keys.len()
            )
        }
    }
}

/// Prune keys from a bundle that match a given prefix or tag pattern.
pub fn prune_by_prefix(
    bundle: &mut Bundle,
    prefix: &str,
    dry_run: bool,
) -> Result<PruneResult, VaultError> {
    let all_keys: Vec<String> = bundle.secrets.keys().cloned().collect();
    let mut removed = Vec::new();
    let mut kept = Vec::new();

    for key in &all_keys {
        if key.starts_with(prefix) {
            removed.push(key.clone());
        } else {
            kept.push(key.clone());
        }
    }

    if !dry_run {
        for key in &removed {
            bundle.secrets.remove(key);
        }
    }

    Ok(PruneResult {
        removed_keys: removed,
        kept_keys: kept,
        dry_run,
    })
}

/// Prune keys whose values are empty or whitespace-only.
pub fn prune_empty_values(
    bundle: &mut Bundle,
    dry_run: bool,
) -> Result<PruneResult, VaultError> {
    let all_keys: Vec<String> = bundle.secrets.keys().cloned().collect();
    let mut removed = Vec::new();
    let mut kept = Vec::new();

    for key in &all_keys {
        let val = bundle.secrets.get(key).map(|v| v.trim().is_empty()).unwrap_or(true);
        if val {
            removed.push(key.clone());
        } else {
            kept.push(key.clone());
        }
    }

    if !dry_run {
        for key in &removed {
            bundle.secrets.remove(key);
        }
    }

    Ok(PruneResult {
        removed_keys: removed,
        kept_keys: kept,
        dry_run,
    })
}

/// Prune keys by an explicit list.
pub fn prune_by_keys(
    bundle: &mut Bundle,
    keys: &[String],
    dry_run: bool,
) -> Result<PruneResult, VaultError> {
    let mut removed = Vec::new();
    let mut kept = Vec::new();

    for key in bundle.secrets.keys() {
        if keys.contains(key) {
            removed.push(key.clone());
        } else {
            kept.push(key.clone());
        }
    }

    if !dry_run {
        for key in &removed {
            bundle.secrets.remove(key);
        }
    }

    Ok(PruneResult {
        removed_keys: removed,
        kept_keys: kept,
        dry_run,
    })
}
