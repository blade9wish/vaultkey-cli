use crate::error::VaultError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Delete,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRule {
    pub identity: String,
    pub permissions: Vec<Permission>,
    pub bundle_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessPolicy {
    pub rules: Vec<AccessRule>,
}

impl AccessPolicy {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: AccessRule) {
        self.rules.retain(|r| r.identity != rule.identity || r.bundle_pattern != rule.bundle_pattern);
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, identity: &str, bundle_pattern: &str) -> bool {
        let before = self.rules.len();
        self.rules.retain(|r| !(r.identity == identity && r.bundle_pattern == bundle_pattern));
        self.rules.len() < before
    }

    pub fn check(&self, identity: &str, bundle: &str, permission: &Permission) -> bool {
        self.rules.iter().any(|rule| {
            rule.identity == identity
                && bundle_matches(&rule.bundle_pattern, bundle)
                && rule.permissions.contains(permission)
        })
    }

    pub fn list_for_identity(&self, identity: &str) -> Vec<&AccessRule> {
        self.rules.iter().filter(|r| r.identity == identity).collect()
    }

    pub fn summary(&self) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for rule in &self.rules {
            map.entry(rule.identity.clone())
                .or_default()
                .push(rule.bundle_pattern.clone());
        }
        map
    }
}

fn bundle_matches(pattern: &str, bundle: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix('*') {
        return bundle.starts_with(prefix);
    }
    pattern == bundle
}

pub fn parse_permissions(raw: &[&str]) -> Result<Vec<Permission>, VaultError> {
    raw.iter()
        .map(|s| match *s {
            "read" => Ok(Permission::Read),
            "write" => Ok(Permission::Write),
            "delete" => Ok(Permission::Delete),
            "admin" => Ok(Permission::Admin),
            other => Err(VaultError::InvalidInput(format!("Unknown permission: {}", other))),
        })
        .collect()
}
