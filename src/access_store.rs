use crate::access::AccessPolicy;
use crate::error::VaultError;
use std::fs;
use std::path::{Path, PathBuf};

pub struct AccessStore {
    path: PathBuf,
}

impl AccessStore {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn load(&self) -> Result<AccessPolicy, VaultError> {
        if !self.path.exists() {
            return Ok(AccessPolicy::new());
        }
        let content = fs::read_to_string(&self.path)
            .map_err(|e| VaultError::Io(e.to_string()))?;
        toml::from_str(&content)
            .map_err(|e| VaultError::Parse(e.to_string()))
    }

    pub fn save(&self, policy: &AccessPolicy) -> Result<(), VaultError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| VaultError::Io(e.to_string()))?;
        }
        let content = toml::to_string_pretty(policy)
            .map_err(|e| VaultError::Serialize(e.to_string()))?;
        fs::write(&self.path, content)
            .map_err(|e| VaultError::Io(e.to_string()))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
