use std::path::{Path, PathBuf};
use crate::audit::{AuditEntry, AuditLog, AuditAction, make_entry};
use crate::error::VaultError;

/// A higher-level wrapper that records audit events for vault operations.
pub struct AuditStore {
    log: AuditLog,
}

impl AuditStore {
    pub fn new(path: impl AsRef<Path>) -> Self {
        AuditStore {
            log: AuditLog::new(path),
        }
    }

    pub fn log_bundle_create(&self, name: &str, success: bool) -> Result<(), VaultError> {
        self.log.record(make_entry(
            AuditAction::BundleCreate,
            name,
            success,
            None,
        ))
    }

    pub fn log_secret_add(&self, bundle: &str, key: &str, success: bool) -> Result<(), VaultError> {
        self.log.record(make_entry(
            AuditAction::SecretAdd,
            bundle,
            success,
            Some(format!("key={}", key)),
        ))
    }

    pub fn log_secret_get(&self, bundle: &str, key: &str, success: bool) -> Result<(), VaultError> {
        self.log.record(make_entry(
            AuditAction::SecretGet,
            bundle,
            success,
            Some(format!("key={}", key)),
        ))
    }

    pub fn log_secret_remove(&self, bundle: &str, key: &str, success: bool) -> Result<(), VaultError> {
        self.log.record(make_entry(
            AuditAction::SecretRemove,
            bundle,
            success,
            Some(format!("key={}", key)),
        ))
    }

    pub fn log_keyring_unlock(&self, name: &str, success: bool) -> Result<(), VaultError> {
        self.log.record(make_entry(
            AuditAction::KeyringUnlock,
            name,
            success,
            None,
        ))
    }

    pub fn entries(&self) -> Result<Vec<AuditEntry>, VaultError> {
        self.log.read_entries()
    }

    pub fn recent(&self, n: usize) -> Result<Vec<AuditEntry>, VaultError> {
        let mut all = self.log.read_entries()?;
        all.reverse();
        all.truncate(n);
        Ok(all)
    }
}
