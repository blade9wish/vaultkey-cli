use crate::pin::PinStore;
use crate::error::VaultError;

/// Handle the `pin add <key> [--label <label>]` command.
pub fn cmd_pin_add(
    store: &mut PinStore,
    key: &str,
    label: Option<String>,
) -> Result<(), VaultError> {
    store.pin(key, label.clone())?;
    match label {
        Some(l) => println!("Pinned '{}' with label '{}'", key, l),
        None => println!("Pinned '{}'", key),
    }
    Ok(())
}

/// Handle the `pin remove <key>` command.
pub fn cmd_pin_remove(store: &mut PinStore, key: &str) -> Result<(), VaultError> {
    store.unpin(key)?;
    println!("Unpinned '{}'", key);
    Ok(())
}

/// Handle the `pin list` command.
pub fn cmd_pin_list(store: &PinStore) {
    let pins = store.list();
    if pins.is_empty() {
        println!("No pinned secrets.");
        return;
    }
    println!("{:<30} {:<20} {}", "Key", "Label", "Pinned At (unix)");
    println!("{}", "-".repeat(65));
    for p in pins {
        println!(
            "{:<30} {:<20} {}",
            p.key,
            p.label.as_deref().unwrap_or("-"),
            p.pinned_at
        );
    }
}

/// Handle the `pin check <key>` command.
pub fn cmd_pin_check(store: &PinStore, key: &str) {
    if store.is_pinned(key) {
        println!("'{}' is pinned.", key);
    } else {
        println!("'{}' is not pinned.", key);
    }
}
