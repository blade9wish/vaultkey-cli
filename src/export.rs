//! Export module for vaultkey-cli.
//!
//! Provides functionality to export secret bundles to various formats
//! such as plain JSON, environment variable files (.env), and encrypted archives.

use crate::bundle::Bundle;
use crate::error::VaultError;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Supported export formats.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    /// Export as a JSON file
    Json,
    /// Export as a dotenv (.env) file
    DotEnv,
    /// Export as a TOML file
    Toml,
}

impl ExportFormat {
    /// Parse a format string into an ExportFormat variant.
    pub fn from_str(s: &str) -> Result<Self, VaultError> {
        match s.to_lowercase().as_str() {
            "json" => Ok(ExportFormat::Json),
            "dotenv" | "env" => Ok(ExportFormat::DotEnv),
            "toml" => Ok(ExportFormat::Toml),
            other => Err(VaultError::InvalidFormat(format!(
                "Unsupported export format: '{}'. Use json, dotenv, or toml.",
                other
            ))),
        }
    }

    /// Returns the typical file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Json => "json",
            ExportFormat::DotEnv => "env",
            ExportFormat::Toml => "toml",
        }
    }
}

/// Export a bundle's secrets to the specified format and write to a file.
///
/// # Arguments
/// * `bundle` - The bundle whose secrets will be exported
/// * `format` - The target export format
/// * `output_path` - Destination file path
///
/// # Returns
/// `Ok(())` on success, or a `VaultError` on failure.
pub fn export_bundle(
    bundle: &Bundle,
    format: &ExportFormat,
    output_path: &Path,
) -> Result<(), VaultError> {
    let secrets: &HashMap<String, String> = bundle.secrets();

    let content = match format {
        ExportFormat::Json => serialize_json(secrets)?,
        ExportFormat::DotEnv => serialize_dotenv(secrets),
        ExportFormat::Toml => serialize_toml(secrets)?,
    };

    fs::write(output_path, content).map_err(|e| {
        VaultError::Io(format!(
            "Failed to write export to '{}': {}",
            output_path.display(),
            e
        ))
    })?;

    Ok(())
}

/// Serialize secrets map to a JSON string.
fn serialize_json(secrets: &HashMap<String, String>) -> Result<String, VaultError> {
    serde_json::to_string_pretty(secrets)
        .map_err(|e| VaultError::Serialization(format!("JSON serialization error: {}", e)))
}

/// Serialize secrets map to dotenv format (KEY=value per line).
fn serialize_dotenv(secrets: &HashMap<String, String>) -> String {
    let mut lines: Vec<String> = secrets
        .iter()
        .map(|(k, v)| {
            // Quote values that contain spaces or special characters
            if v.contains(' ') || v.contains('"') || v.contains('\n') {
                format!("{}=\"{}\"", k, v.replace('"', "\\\""))
            } else {
                format!("{}={}", k, v)
            }
        })
        .collect();
    lines.sort(); // deterministic output
    lines.join("\n") + "\n"
}

/// Serialize secrets map to TOML format.
fn serialize_toml(secrets: &HashMap<String, String>) -> Result<String, VaultError> {
    toml::to_string(secrets)
        .map_err(|e| VaultError::Serialization(format!("TOML serialization error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_secrets() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("API_KEY".to_string(), "abc123".to_string());
        m.insert("DB_PASS".to_string(), "s3cr3t".to_string());
        m
    }

    #[test]
    fn test_format_from_str() {
        assert_eq!(ExportFormat::from_str("json").unwrap(), ExportFormat::Json);
        assert_eq!(ExportFormat::from_str("dotenv").unwrap(), ExportFormat::DotEnv);
        assert_eq!(ExportFormat::from_str("env").unwrap(), ExportFormat::DotEnv);
        assert_eq!(ExportFormat::from_str("toml").unwrap(), ExportFormat::Toml);
        assert!(ExportFormat::from_str("xml").is_err());
    }

    #[test]
    fn test_serialize_dotenv() {
        let secrets = sample_secrets();
        let output = serialize_dotenv(&secrets);
        assert!(output.contains("API_KEY=abc123"));
        assert!(output.contains("DB_PASS=s3cr3t"));
    }

    #[test]
    fn test_serialize_json() {
        let secrets = sample_secrets();
        let output = serialize_json(&secrets).unwrap();
        assert!(output.contains("API_KEY"));
        assert!(output.contains("abc123"));
    }

    #[test]
    fn test_serialize_toml() {
        let secrets = sample_secrets();
        let output = serialize_toml(&secrets).unwrap();
        assert!(output.contains("API_KEY"));
    }

    #[test]
    fn test_dotenv_quotes_values_with_spaces() {
        let mut secrets = HashMap::new();
        secrets.insert("NOTE".to_string(), "hello world".to_string());
        let output = serialize_dotenv(&secrets);
        assert!(output.contains("NOTE=\"hello world\""));
    }
}
