use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::VaultError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: AuditAction,
    pub target: String,
    pub user: Option<String>,
    pub success: bool,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    BundleCreate,
    BundleOpen,
    BundleDelete,
    SecretAdd,
    SecretGet,
    SecretRemove,
    KeyringUnlock,
    KeyringLock,
    ConfigLoad,
}

pub struct AuditLog {
    path: PathBuf,
}

impl AuditLog {
    pub fn new(path: impl AsRef<Path>) -> Self {
        AuditLog {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn record(&self, entry: AuditEntry) -> Result<(), VaultError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| VaultError::Io(e))?;

        let line = serde_json::to_string(&entry)
            .map_err(|e| VaultError::Serialization(e.to_string()))?;

        writeln!(file, "{}", line).map_err(|e| VaultError::Io(e))?;
        Ok(())
    }

    pub fn read_entries(&self) -> Result<Vec<AuditEntry>, VaultError> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let content = std::fs::read_to_string(&self.path)
            .map_err(|e| VaultError::Io(e))?;

        let entries = content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str::<AuditEntry>(l)
                .map_err(|e| VaultError::Serialization(e.to_string())))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }
}

pub fn make_entry(
    action: AuditAction,
    target: impl Into<String>,
    success: bool,
    detail: Option<String>,
) -> AuditEntry {
    AuditEntry {
        timestamp: Utc::now(),
        action,
        target: target.into(),
        user: std::env::var("USER").ok(),
        success,
        detail,
    }
}
