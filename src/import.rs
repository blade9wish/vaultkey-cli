use std::path::Path;
use std::fs;
use serde::Deserialize;
use crate::error::VaultKeyError;
use crate::bundle::Bundle;
use crate::vault::Vault;

#[derive(Debug, Deserialize)]
pub struct ImportManifest {
    pub format: String,
    pub version: Option<u32>,
    pub secrets: Vec<ImportEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ImportEntry {
    pub key: String,
    pub value: String,
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
}

pub fn import_from_toml(path: &Path, vault: &mut Vault) -> Result<usize, VaultKeyError> {
    let content = fs::read_to_string(path)
        .map_err(|e| VaultKeyError::Io(e))?;
    let manifest: ImportManifest = toml::from_str(&content)
        .map_err(|e| VaultKeyError::Config(e.to_string()))?;

    if manifest.format != "vaultkey" {
        return Err(VaultKeyError::Config(format!(
            "Unsupported import format: {}",
            manifest.format
        )));
    }

    let mut count = 0;
    for entry in manifest.secrets {
        let mut bundle = Bundle::new(entry.key.clone(), entry.value);
        if let Some(tags) = entry.tags {
            bundle.tags = tags;
        }
        if let Some(desc) = entry.description {
            bundle.description = Some(desc);
        }
        vault.add_bundle(bundle)?;
        count += 1;
    }

    Ok(count)
}

pub fn import_from_env_file(path: &Path, vault: &mut Vault) -> Result<usize, VaultKeyError> {
    let content = fs::read_to_string(path)
        .map_err(|e| VaultKeyError::Io(e))?;

    let mut count = 0;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().trim_matches('"').to_string();
            let bundle = Bundle::new(key, value);
            vault.add_bundle(bundle)?;
            count += 1;
        }
    }

    Ok(count)
}
