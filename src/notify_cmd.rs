use crate::error::VaultError;
use crate::notify::{NotifyChannel, NotifyEvent};
use crate::notify_store::NotifyStore;
use std::path::PathBuf;

fn store_path() -> PathBuf {
    PathBuf::from(".vaultkey/notify.toml")
}

pub fn cmd_notify_add(
    event: &str,
    channel_type: &str,
    target: Option<&str>,
) -> Result<(), VaultError> {
    let mut store = NotifyStore::load(&store_path())?;
    let channel = parse_channel(channel_type, target)?;
    store.add_rule(event, channel, None);
    store.save(&store_path())?;
    println!("Notification rule added for event '{}'", event);
    Ok(())
}

pub fn cmd_notify_remove(event: &str) -> Result<(), VaultError> {
    let mut store = NotifyStore::load(&store_path())?;
    let removed = store.remove_rules_for_event(event);
    store.save(&store_path())?;
    println!("Removed {} rule(s) for event '{}'", removed, event);
    Ok(())
}

pub fn cmd_notify_list() -> Result<(), VaultError> {
    let store = NotifyStore::load(&store_path())?;
    if store.list().is_empty() {
        println!("No notification rules configured.");
        return Ok(());
    }
    println!("{:<12} {:<10} {}", "EVENT", "CHANNEL", "TARGET");
    println!("{}", "-".repeat(50));
    for rule in store.list() {
        let (ch, tgt) = channel_display(&rule.channel);
        println!("{:<12} {:<10} {}", rule.event, ch, tgt);
    }
    Ok(())
}

pub fn cmd_notify_test(event_type: &str, key: &str) -> Result<(), VaultError> {
    let store = NotifyStore::load(&store_path())?;
    let event = build_test_event(event_type, key)?;
    crate::notify::dispatch(&event, store.list())?;
    println!("Test notification dispatched.");
    Ok(())
}

fn parse_channel(kind: &str, target: Option<&str>) -> Result<NotifyChannel, VaultError> {
    match kind {
        "stdout" => Ok(NotifyChannel::Stdout),
        "file" => {
            let path = target.ok_or_else(|| VaultError::Config("file target required".into()))?;
            Ok(NotifyChannel::File(path.to_string()))
        }
        "webhook" => {
            let url = target.ok_or_else(|| VaultError::Config("webhook url required".into()))?;
            Ok(NotifyChannel::Webhook(url.to_string()))
        }
        other => Err(VaultError::Config(format!("unknown channel type: {}", other))),
    }
}

fn channel_display(ch: &NotifyChannel) -> (&'static str, String) {
    match ch {
        NotifyChannel::Stdout => ("stdout", String::new()),
        NotifyChannel::File(p) => ("file", p.clone()),
        NotifyChannel::Webhook(u) => ("webhook", u.clone()),
    }
}

fn build_test_event(kind: &str, key: &str) -> Result<NotifyEvent, VaultError> {
    match kind {
        "access" => Ok(NotifyEvent::SecretAccessed(key.to_string())),
        "rotate" => Ok(NotifyEvent::SecretRotated(key.to_string())),
        "expire" => Ok(NotifyEvent::SecretExpired(key.to_string())),
        "unlock" => Ok(NotifyEvent::BundleUnlocked(key.to_string())),
        "policy" => Ok(NotifyEvent::PolicyViolation(key.to_string())),
        "quota" => Ok(NotifyEvent::QuotaExceeded(key.to_string())),
        other => Err(VaultError::Config(format!("unknown event type: {}", other))),
    }
}
