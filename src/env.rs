//! Environment variable injection for secret bundles.
//!
//! Allows exporting secrets from a vault bundle as environment variables,
//! either by printing shell export statements or by writing a `.env` file.

use crate::bundle::Bundle;
use crate::error::VaultError;
use std::collections::HashMap;
use std::path::Path;

/// Format for environment variable output.
#[derive(Debug, Clone, PartialEq)]
pub enum EnvFormat {
    /// `export KEY=VALUE` shell syntax
    Shell,
    /// `KEY=VALUE` dotenv syntax
    Dotenv,
}

impl Default for EnvFormat {
    fn default() -> Self {
        EnvFormat::Shell
    }
}

/// Render secrets from a bundle as environment variable lines.
pub fn render_env(bundle: &Bundle, format: &EnvFormat) -> Vec<String> {
    let mut lines = Vec::new();
    let mut keys: Vec<&String> = bundle.secrets.keys().collect();
    keys.sort();
    for key in keys {
        let value = &bundle.secrets[key];
        let sanitized_key = key.to_uppercase().replace('-', "_").replace('.', "_");
        let line = match format {
            EnvFormat::Shell => format!("export {}={}", sanitized_key, shell_escape(value)),
            EnvFormat::Dotenv => format!("{}={}", sanitized_key, value),
        };
        lines.push(line);
    }
    lines
}

/// Write env output to a file path.
pub fn write_env_file(
    bundle: &Bundle,
    format: &EnvFormat,
    path: &Path,
) -> Result<(), VaultError> {
    let lines = render_env(bundle, format);
    let content = lines.join("\n") + "\n";
    std::fs::write(path, content)
        .map_err(|e| VaultError::Io(e))
}

/// Build a HashMap of env-style key/value pairs from a bundle.
pub fn to_env_map(bundle: &Bundle) -> HashMap<String, String> {
    bundle
        .secrets
        .iter()
        .map(|(k, v)| {
            let key = k.to_uppercase().replace('-', "_").replace('.', "_");
            (key, v.clone())
        })
        .collect()
}

/// Minimally escape a value for shell export usage.
fn shell_escape(value: &str) -> String {
    if value.contains(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | '$' | '`' | '\\')) {
        format!("'{}'", value.replace('\'', "'\\''" ))
    } else {
        value.to_string()
    }
}
