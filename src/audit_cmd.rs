use clap::ArgMatches;
use crate::audit::{AuditLog, AuditAction};
use crate::error::VaultError;
use crate::config::Config;

pub fn handle_audit(matches: &ArgMatches, config: &Config) -> Result<(), VaultError> {
    let log_path = config
        .audit_log_path
        .as_deref()
        .unwrap_or("vault_audit.log");

    let log = AuditLog::new(log_path);

    match matches.subcommand() {
        Some(("list", sub)) => {
            let entries = log.read_entries()?;
            let limit: usize = sub
                .get_one::<String>("limit")
                .and_then(|v| v.parse().ok())
                .unwrap_or(20);

            let action_filter = sub.get_one::<String>("action");

            let filtered: Vec<_> = entries
                .iter()
                .filter(|e| {
                    action_filter.map_or(true, |a| {
                        format!("{:?}", e.action).to_lowercase() == a.to_lowercase()
                    })
                })
                .rev()
                .take(limit)
                .collect();

            if filtered.is_empty() {
                println!("No audit entries found.");
                return Ok(());
            }

            for entry in filtered {
                let status = if entry.success { "OK" } else { "FAIL" };
                let detail = entry.detail.as_deref().unwrap_or("");
                println!(
                    "[{}] [{status}] {:?} -> {} {}",
                    entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    entry.action,
                    entry.target,
                    detail
                );
            }
        }
        Some(("clear", _)) => {
            std::fs::remove_file(log_path)
                .map_err(|e| VaultError::Io(e))?;
            println!("Audit log cleared.");
        }
        _ => {
            println!("Usage: vaultkey audit <list|clear>");
        }
    }

    Ok(())
}

pub fn audit_action_from_str(s: &str) -> Option<AuditAction> {
    match s.to_lowercase().as_str() {
        "bundle_create" => Some(AuditAction::BundleCreate),
        "bundle_open" => Some(AuditAction::BundleOpen),
        "bundle_delete" => Some(AuditAction::BundleDelete),
        "secret_add" => Some(AuditAction::SecretAdd),
        "secret_get" => Some(AuditAction::SecretGet),
        "secret_remove" => Some(AuditAction::SecretRemove),
        "keyring_unlock" => Some(AuditAction::KeyringUnlock),
        "keyring_lock" => Some(AuditAction::KeyringLock),
        "config_load" => Some(AuditAction::ConfigLoad),
        _ => None,
    }
}
