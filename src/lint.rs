//! Lint checks for secret bundles — validates key naming conventions,
//! detects duplicate keys, and warns about empty or suspicious values.

use crate::bundle::Bundle;

#[derive(Debug, Clone, PartialEq)]
pub enum LintLevel {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct LintDiagnostic {
    pub level: LintLevel,
    pub key: String,
    pub message: String,
}

impl LintDiagnostic {
    pub fn warning(key: impl Into<String>, message: impl Into<String>) -> Self {
        Self { level: LintLevel::Warning, key: key.into(), message: message.into() }
    }

    pub fn error(key: impl Into<String>, message: impl Into<String>) -> Self {
        Self { level: LintLevel::Error, key: key.into(), message: message.into() }
    }

    pub fn is_error(&self) -> bool {
        self.level == LintLevel::Error
    }
}

pub fn lint_bundle(bundle: &Bundle) -> Vec<LintDiagnostic> {
    let mut diags: Vec<LintDiagnostic> = Vec::new();
    let mut seen_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

    for (key, value) in bundle.secrets() {
        // Duplicate key detection
        let lower = key.to_lowercase();
        if !seen_keys.insert(lower.clone()) {
            diags.push(LintDiagnostic::error(key, "Duplicate key detected (case-insensitive)"));
        }

        // Key naming convention: only alphanumeric, underscores, hyphens
        if !key.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            diags.push(LintDiagnostic::error(
                key,
                "Key contains invalid characters; use only [a-zA-Z0-9_-]",
            ));
        }

        // Warn about keys that look like they might be placeholders
        let val_lower = value.to_lowercase();
        if val_lower == "todo"
            || val_lower == "fixme"
            || val_lower == "changeme"
            || val_lower == "placeholder"
        {
            diags.push(LintDiagnostic::warning(
                key,
                format!("Value looks like a placeholder: '{}'", value),
            ));
        }

        // Warn about empty values
        if value.trim().is_empty() {
            diags.push(LintDiagnostic::warning(key, "Value is empty"));
        }

        // Warn if key is excessively long
        if key.len() > 64 {
            diags.push(LintDiagnostic::warning(key, "Key name exceeds 64 characters"));
        }
    }

    diags
}

pub fn has_errors(diags: &[LintDiagnostic]) -> bool {
    diags.iter().any(|d| d.is_error())
}
