//! CLI command handlers for keyring subcommands.

use crate::error::VaultKeyError;
use crate::keyring::KeyEntry;
use crate::keyring_store::KeyringStore;

/// Add a new key alias to the keyring file at `store_path`.
pub fn cmd_add_key(
    store: &KeyringStore,
    alias: &str,
    fingerprint: &str,
    email: Option<&str>,
    description: Option<&str>,
) -> Result<(), VaultKeyError> {
    let mut keyring = store.load()?;
    let mut entry = KeyEntry::new(alias, fingerprint);
    if let Some(e) = email {
        entry = entry.with_email(e);
    }
    if let Some(d) = description {
        entry = entry.with_description(d);
    }
    keyring.add(entry)?;
    store.save(&keyring)?;
    println!("Key '{}' added to keyring.", alias);
    Ok(())
}

/// Remove a key alias from the keyring.
pub fn cmd_remove_key(store: &KeyringStore, alias: &str) -> Result<(), VaultKeyError> {
    let mut keyring = store.load()?;
    keyring.remove(alias)?;
    store.save(&keyring)?;
    println!("Key '{}' removed from keyring.", alias);
    Ok(())
}

/// List all keys in the keyring.
pub fn cmd_list_keys(store: &KeyringStore) -> Result<(), VaultKeyError> {
    let keyring = store.load()?;
    if keyring.is_empty() {
        println!("No keys in keyring.");
        return Ok(());
    }
    println!("{:<20} {:<52} {}", "ALIAS", "FINGERPRINT", "EMAIL");
    println!("{}", "-".repeat(80));
    for entry in keyring.list() {
        println!(
            "{:<20} {:<52} {}",
            entry.alias,
            entry.fingerprint,
            entry.email.as_deref().unwrap_or("-")
        );
    }
    Ok(())
}

/// Look up a key by alias and print its details.
pub fn cmd_show_key(store: &KeyringStore, alias: &str) -> Result<(), VaultKeyError> {
    let keyring = store.load()?;
    match keyring.get(alias) {
        Some(entry) => {
            println!("Alias:       {}", entry.alias);
            println!("Fingerprint: {}", entry.fingerprint);
            if let Some(ref e) = entry.email {
                println!("Email:       {}", e);
            }
            if let Some(ref d) = entry.description {
                println!("Description: {}", d);
            }
            Ok(())
        }
        None => Err(VaultKeyError::Config(format!(
            "Key alias '{}' not found",
            alias
        ))),
    }
}

/// Rename an existing key alias in the keyring.
///
/// The fingerprint and all other fields are preserved; only the alias changes.
pub fn cmd_rename_key(
    store: &KeyringStore,
    old_alias: &str,
    new_alias: &str,
) -> Result<(), VaultKeyError> {
    let mut keyring = store.load()?;
    let mut entry = keyring
        .get(old_alias)
        .cloned()
        .ok_or_else(|| VaultKeyError::Config(format!("Key alias '{}' not found", old_alias)))?;
    keyring.remove(old_alias)?;
    entry.alias = new_alias.to_string();
    keyring.add(entry)?;
    store.save(&keyring)?;
    println!("Key '{}' renamed to '{}'.", old_alias, new_alias);
    Ok(())
}
