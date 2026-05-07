//! Verification module for validating secret bundle integrity and consistency.

use crate::bundle::Bundle;
use crate::error::VaultError;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum VerifyIssue {
    DuplicateKey(String),
    EmptyValue(String),
    InvalidKeyFormat(String),
    MissingRequiredKey(String),
}

impl std::fmt::Display for VerifyIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyIssue::DuplicateKey(k) => write!(f, "Duplicate key: '{}'", k),
            VerifyIssue::EmptyValue(k) => write!(f, "Empty value for key: '{}'", k),
            VerifyIssue::InvalidKeyFormat(k) => write!(f, "Invalid key format: '{}'", k),
            VerifyIssue::MissingRequiredKey(k) => write!(f, "Missing required key: '{}'", k),
        }
    }
}

#[derive(Debug, Default)]
pub struct VerifyResult {
    pub issues: Vec<VerifyIssue>,
}

impl VerifyResult {
    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn add(&mut self, issue: VerifyIssue) {
        self.issues.push(issue);
    }
}

fn is_valid_key(key: &str) -> bool {
    !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
}

pub fn verify_bundle(bundle: &Bundle, required_keys: &[&str]) -> Result<VerifyResult, VaultError> {
    let mut result = VerifyResult::default();
    let mut seen_keys: HashSet<String> = HashSet::new();

    for (key, value) in &bundle.secrets {
        if !seen_keys.insert(key.clone()) {
            result.add(VerifyIssue::DuplicateKey(key.clone()));
        }
        if !is_valid_key(key) {
            result.add(VerifyIssue::InvalidKeyFormat(key.clone()));
        }
        if value.trim().is_empty() {
            result.add(VerifyIssue::EmptyValue(key.clone()));
        }
    }

    for req in required_keys {
        if !bundle.secrets.contains_key(*req) {
            result.add(VerifyIssue::MissingRequiredKey(req.to_string()));
        }
    }

    Ok(result)
}
