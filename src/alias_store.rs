use std::path::{Path, PathBuf};
use std::fs;
use crate::alias::AliasMap;
use crate::error::VaultError;

pub struct AliasStore {
    path: PathBuf,
}

impl AliasStore {
    pub fn new(dir: &Path) -> Self {
        Self {
            path: dir.join("aliases.toml"),
        }
    }

    pub fn load(&self) -> Result<AliasMap, VaultError> {
        if !self.path.exists() {
            return Ok(AliasMap::new());
        }
        let content = fs::read_to_string(&self.path)
            .map_err(|e| VaultError::Io(e.to_string()))?;
        toml::from_str(&content)
            .map_err(|e| VaultError::Parse(e.to_string()))
    }

    pub fn save(&self, map: &AliasMap) -> Result<(), VaultError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| VaultError::Io(e.to_string()))?;
        }
        let content = toml::to_string_pretty(map)
            .map_err(|e| VaultError::Parse(e.to_string()))?;
        fs::write(&self.path, content)
            .map_err(|e| VaultError::Io(e.to_string()))
    }
}
