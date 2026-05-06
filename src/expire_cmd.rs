use crate::expire::{ExpireEntry, ExpireRegistry};
use crate::error::VaultError;
use chrono::{DateTime, Duration, Utc};

pub fn cmd_expire_set(
    registry: &mut ExpireRegistry,
    key: &str,
    ttl_secs: i64,
    notify_before_secs: Option<i64>,
) -> Result<(), VaultError> {
    if ttl_secs <= 0 {
        return Err(VaultError::InvalidInput("TTL must be positive".to_string()));
    }
    let expires_at = Utc::now() + Duration::seconds(ttl_secs);
    let entry = ExpireEntry::new(key, expires_at, notify_before_secs);
    registry.set(entry);
    println!("Expiry set for '{}': expires in {} seconds", key, ttl_secs);
    Ok(())
}

pub fn cmd_expire_check(
    registry: &ExpireRegistry,
    key: &str,
) -> Result<(), VaultError> {
    let entry = registry.get(key).ok_or_else(|| {
        VaultError::NotFound(format!("No expiry configured for key: {}", key))
    })?;
    if entry.is_expired() {
        println!("[EXPIRED] '{}' expired at {}", key, entry.expires_at);
    } else if entry.is_expiring_soon() {
        println!(
            "[WARN] '{}' expiring soon — {} seconds remaining",
            key,
            entry.seconds_remaining()
        );
    } else {
        println!("[OK] '{}' valid — {} seconds remaining", key, entry.seconds_remaining());
    }
    Ok(())
}

pub fn cmd_expire_list(registry: &ExpireRegistry) {
    if registry.entries.is_empty() {
        println!("No expiry entries registered.");
        return;
    }
    for entry in registry.entries.values() {
        let status = if entry.is_expired() {
            "EXPIRED"
        } else if entry.is_expiring_soon() {
            "SOON"
        } else {
            "OK"
        };
        println!("[{}] {} — expires at {}", status, entry.key, entry.expires_at);
    }
}

pub fn cmd_expire_purge(registry: &mut ExpireRegistry) {
    let removed = registry.purge_expired();
    if removed.is_empty() {
        println!("No expired entries to purge.");
    } else {
        println!("Purged {} expired entries:", removed.len());
        for key in &removed {
            println!("  - {}", key);
        }
    }
}
