use crate::error::VaultError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NotifyEvent {
    SecretAccessed(String),
    SecretRotated(String),
    SecretExpired(String),
    BundleUnlocked(String),
    PolicyViolation(String),
    QuotaExceeded(String),
}

impl fmt::Display for NotifyEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotifyEvent::SecretAccessed(k) => write!(f, "[ACCESS] Secret accessed: {}", k),
            NotifyEvent::SecretRotated(k) => write!(f, "[ROTATE] Secret rotated: {}", k),
            NotifyEvent::SecretExpired(k) => write!(f, "[EXPIRE] Secret expired: {}", k),
            NotifyEvent::BundleUnlocked(b) => write!(f, "[UNLOCK] Bundle unlocked: {}", b),
            NotifyEvent::PolicyViolation(p) => write!(f, "[POLICY] Policy violation: {}", p),
            NotifyEvent::QuotaExceeded(q) => write!(f, "[QUOTA] Quota exceeded: {}", q),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyRule {
    pub event: String,
    pub channel: NotifyChannel,
    pub filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotifyChannel {
    Stdout,
    File(String),
    Webhook(String),
}

pub fn dispatch(event: &NotifyEvent, rules: &[NotifyRule]) -> Result<(), VaultError> {
    for rule in rules {
        if matches_rule(event, rule) {
            deliver(event, &rule.channel)?;
        }
    }
    Ok(())
}

fn matches_rule(event: &NotifyEvent, rule: &NotifyRule) -> bool {
    let event_tag = match event {
        NotifyEvent::SecretAccessed(_) => "access",
        NotifyEvent::SecretRotated(_) => "rotate",
        NotifyEvent::SecretExpired(_) => "expire",
        NotifyEvent::BundleUnlocked(_) => "unlock",
        NotifyEvent::PolicyViolation(_) => "policy",
        NotifyEvent::QuotaExceeded(_) => "quota",
    };
    rule.event == "*" || rule.event == event_tag
}

fn deliver(event: &NotifyEvent, channel: &NotifyChannel) -> Result<(), VaultError> {
    match channel {
        NotifyChannel::Stdout => {
            println!("{}", event);
            Ok(())
        }
        NotifyChannel::File(path) => {
            use std::fs::OpenOptions;
            use std::io::Write;
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| VaultError::Io(e))?;
            writeln!(file, "{}", event).map_err(|e| VaultError::Io(e))?;
            Ok(())
        }
        NotifyChannel::Webhook(url) => {
            eprintln!("[notify] webhook dispatch to {} (not implemented)", url);
            Ok(())
        }
    }
}
