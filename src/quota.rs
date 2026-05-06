use crate::error::VaultError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaPolicy {
    pub max_secrets: Option<usize>,
    pub max_bundles: Option<usize>,
    pub max_secret_size_bytes: Option<usize>,
    pub max_total_size_bytes: Option<usize>,
}

impl Default for QuotaPolicy {
    fn default() -> Self {
        Self {
            max_secrets: Some(500),
            max_bundles: Some(50),
            max_secret_size_bytes: Some(4096),
            max_total_size_bytes: Some(1_048_576),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuotaUsage {
    pub secret_count: usize,
    pub bundle_count: usize,
    pub total_size_bytes: usize,
    pub per_bundle: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct QuotaViolation {
    pub field: String,
    pub current: usize,
    pub limit: usize,
}

impl std::fmt::Display for QuotaViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quota exceeded for '{}': current={}, limit={}",
            self.field, self.current, self.limit
        )
    }
}

pub fn check_quota(
    policy: &QuotaPolicy,
    usage: &QuotaUsage,
) -> Result<(), VaultError> {
    let mut violations: Vec<String> = Vec::new();

    if let Some(max) = policy.max_secrets {
        if usage.secret_count >= max {
            violations.push(format!("max_secrets: {}/{}", usage.secret_count, max));
        }
    }
    if let Some(max) = policy.max_bundles {
        if usage.bundle_count >= max {
            violations.push(format!("max_bundles: {}/{}", usage.bundle_count, max));
        }
    }
    if let Some(max) = policy.max_total_size_bytes {
        if usage.total_size_bytes >= max {
            violations.push(format!("max_total_size_bytes: {}/{}", usage.total_size_bytes, max));
        }
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(VaultError::Generic(format!(
            "Quota violation(s): {}",
            violations.join("; ")
        )))
    }
}

pub fn check_secret_size(
    policy: &QuotaPolicy,
    size_bytes: usize,
) -> Result<(), VaultError> {
    if let Some(max) = policy.max_secret_size_bytes {
        if size_bytes > max {
            return Err(VaultError::Generic(format!(
                "Secret size {} bytes exceeds limit of {} bytes",
                size_bytes, max
            )));
        }
    }
    Ok(())
}
