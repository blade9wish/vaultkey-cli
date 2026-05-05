use crate::error::VaultError;
use crate::policy::Policy;
use std::fs;
use std::path::{Path, PathBuf};

pub struct PolicyStore {
    path: PathBuf,
}

impl PolicyStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        PolicyStore { path: path.into() }
    }

    pub fn load_all(&self) -> Result<Vec<Policy>, VaultError> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let content = fs::read_to_string(&self.path)
            .map_err(|e| VaultError::Io(e.to_string()))?;
        let policies: Vec<Policy> = serde_json::from_str(&content)
            .map_err(|e| VaultError::Parse(e.to_string()))?;
        Ok(policies)
    }

    pub fn save_all(&self, policies: &[Policy]) -> Result<(), VaultError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| VaultError::Io(e.to_string()))?;
        }
        let content = serde_json::to_string_pretty(policies)
            .map_err(|e| VaultError::Parse(e.to_string()))?;
        fs::write(&self.path, content)
            .map_err(|e| VaultError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn find(&self, name: &str) -> Result<Option<Policy>, VaultError> {
        let policies = self.load_all()?;
        Ok(policies.into_iter().find(|p| p.name == name))
    }

    pub fn upsert(&self, policy: Policy) -> Result<(), VaultError> {
        let mut policies = self.load_all()?;
        if let Some(pos) = policies.iter().position(|p| p.name == policy.name) {
            policies[pos] = policy;
        } else {
            policies.push(policy);
        }
        self.save_all(&policies)
    }

    pub fn remove(&self, name: &str) -> Result<bool, VaultError> {
        let mut policies = self.load_all()?;
        let before = policies.len();
        policies.retain(|p| p.name != name);
        if policies.len() < before {
            self.save_all(&policies)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
