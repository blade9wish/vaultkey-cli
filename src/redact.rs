use crate::bundle::Bundle;
use crate::error::VaultError;
use std::collections::HashSet;

/// Patterns that indicate a value should be redacted in output
const SENSITIVE_PATTERNS: &[&str] = &[
    "password", "passwd", "secret", "token", "api_key", "apikey",
    "private_key", "auth", "credential", "cert", "passphrase",
];

/// Result of a redaction operation
#[derive(Debug, Clone)]
pub struct RedactedBundle {
    pub name: String,
    pub entries: Vec<RedactedEntry>,
    pub redacted_count: usize,
}

#[derive(Debug, Clone)]
pub struct RedactedEntry {
    pub key: String,
    pub value: String,
    pub was_redacted: bool,
}

/// Determine if a key name suggests sensitive content
pub fn is_sensitive_key(key: &str) -> bool {
    let lower = key.to_lowercase();
    SENSITIVE_PATTERNS.iter().any(|p| lower.contains(p))
}

/// Redact sensitive values in a bundle for safe display
pub fn redact_bundle(
    bundle: &Bundle,
    force_keys: &HashSet<String>,
    show_all: bool,
) -> Result<RedactedBundle, VaultError> {
    let mut entries = Vec::new();
    let mut redacted_count = 0;

    for (key, value) in &bundle.secrets {
        let should_redact = !show_all && (is_sensitive_key(key) || force_keys.contains(key));
        let display_value = if should_redact {
            redacted_count += 1;
            "[REDACTED]".to_string()
        } else {
            value.clone()
        };
        entries.push(RedactedEntry {
            key: key.clone(),
            value: display_value,
            was_redacted: should_redact,
        });
    }

    entries.sort_by(|a, b| a.key.cmp(&b.key));

    Ok(RedactedBundle {
        name: bundle.name.clone(),
        entries,
        redacted_count,
    })
}

/// Format a redacted bundle for terminal output
pub fn format_redacted(rb: &RedactedBundle) -> String {
    let mut lines = vec![format!("Bundle: {}", rb.name)];
    for entry in &rb.entries {
        let marker = if entry.was_redacted { "*" } else { " " };
        lines.push(format!("  [{}] {} = {}", marker, entry.key, entry.value));
    }
    if rb.redacted_count > 0 {
        lines.push(format!("  ({} value(s) redacted)", rb.redacted_count));
    }
    lines.join("\n")
}
