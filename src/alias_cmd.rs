use std::path::Path;
use crate::alias_store::AliasStore;
use crate::error::VaultError;

pub fn cmd_alias_add(dir: &Path, alias: &str, target: &str) -> Result<(), VaultError> {
    let store = AliasStore::new(dir);
    let mut map = store.load()?;
    map.add(alias, target)?;
    store.save(&map)?;
    println!("Alias '{}' -> '{}' added.", alias, target);
    Ok(())
}

pub fn cmd_alias_remove(dir: &Path, alias: &str) -> Result<(), VaultError> {
    let store = AliasStore::new(dir);
    let mut map = store.load()?;
    map.remove(alias)?;
    store.save(&map)?;
    println!("Alias '{}' removed.", alias);
    Ok(())
}

pub fn cmd_alias_list(dir: &Path) -> Result<(), VaultError> {
    let store = AliasStore::new(dir);
    let map = store.load()?;
    let pairs = map.list();
    if pairs.is_empty() {
        println!("No aliases defined.");
    } else {
        println!("{:<24} {}", "ALIAS", "TARGET");
        println!("{}", "-".repeat(48));
        for (alias, target) in pairs {
            println!("{:<24} {}", alias, target);
        }
    }
    Ok(())
}

pub fn cmd_alias_resolve(dir: &Path, alias: &str) -> Result<(), VaultError> {
    let store = AliasStore::new(dir);
    let map = store.load()?;
    match map.resolve(alias) {
        Some(target) => {
            println!("{}", target);
            Ok(())
        }
        None => Err(VaultError::NotFound(format!("Alias '{}' not found", alias))),
    }
}

pub fn cmd_alias_rename(dir: &Path, old: &str, new_alias: &str) -> Result<(), VaultError> {
    let store = AliasStore::new(dir);
    let mut map = store.load()?;
    map.rename(old, new_alias)?;
    store.save(&map)?;
    println!("Alias '{}' renamed to '{}'.", old, new_alias);
    Ok(())
}
