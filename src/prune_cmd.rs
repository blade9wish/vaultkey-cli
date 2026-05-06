use crate::prune::{prune_by_prefix, prune_empty_values, prune_by_keys};
use crate::bundle::Bundle;
use crate::error::VaultError;

#[derive(Debug)]
pub enum PruneMode {
    Prefix(String),
    Empty,
    Keys(Vec<String>),
}

pub fn run_prune(
    bundle: &mut Bundle,
    mode: PruneMode,
    dry_run: bool,
) -> Result<(), VaultError> {
    let result = match mode {
        PruneMode::Prefix(ref prefix) => {
            prune_by_prefix(bundle, prefix, dry_run)?
        }
        PruneMode::Empty => {
            prune_empty_values(bundle, dry_run)?
        }
        PruneMode::Keys(ref keys) => {
            prune_by_keys(bundle, keys, dry_run)?
        }
    };

    println!("{}", result.summary());

    if !result.removed_keys.is_empty() {
        println!("Keys {}:", if dry_run { "to remove" } else { "removed" });
        for key in &result.removed_keys {
            println!("  - {}", key);
        }
    } else {
        println!("No matching keys found.");
    }

    Ok(())
}
