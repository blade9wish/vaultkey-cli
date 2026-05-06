use std::collections::{HashMap, HashSet};
use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::redact::{
    format_redacted, is_sensitive_key, redact_bundle,
};

fn make_bundle(secrets: Vec<(&str, &str)>) -> Bundle {
    Bundle {
        name: "test-bundle".to_string(),
        secrets: secrets
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<_, _>>(),
    }
}

#[test]
fn test_is_sensitive_key_detects_password() {
    assert!(is_sensitive_key("db_password"));
    assert!(is_sensitive_key("PASSWORD"));
    assert!(is_sensitive_key("user_passwd"));
}

#[test]
fn test_is_sensitive_key_detects_token() {
    assert!(is_sensitive_key("api_token"));
    assert!(is_sensitive_key("AUTH_TOKEN"));
    assert!(is_sensitive_key("github_token"));
}

#[test]
fn test_is_sensitive_key_allows_safe_keys() {
    assert!(!is_sensitive_key("username"));
    assert!(!is_sensitive_key("host"));
    assert!(!is_sensitive_key("port"));
    assert!(!is_sensitive_key("database"));
}

#[test]
fn test_redact_bundle_hides_sensitive_values() {
    let bundle = make_bundle(vec![
        ("db_password", "supersecret123"),
        ("host", "localhost"),
    ]);
    let rb = redact_bundle(&bundle, &HashSet::new(), false).unwrap();
    let password_entry = rb.entries.iter().find(|e| e.key == "db_password").unwrap();
    let host_entry = rb.entries.iter().find(|e| e.key == "host").unwrap();
    assert_eq!(password_entry.value, "[REDACTED]");
    assert!(password_entry.was_redacted);
    assert_eq!(host_entry.value, "localhost");
    assert!(!host_entry.was_redacted);
    assert_eq!(rb.redacted_count, 1);
}

#[test]
fn test_redact_bundle_show_all_skips_redaction() {
    let bundle = make_bundle(vec![("api_key", "abc123"), ("region", "us-east-1")]);
    let rb = redact_bundle(&bundle, &HashSet::new(), true).unwrap();
    assert!(rb.entries.iter().all(|e| !e.was_redacted));
    assert_eq!(rb.redacted_count, 0);
}

#[test]
fn test_redact_bundle_force_keys() {
    let bundle = make_bundle(vec![("my_custom_field", "hidden_value")]);
    let mut force = HashSet::new();
    force.insert("my_custom_field".to_string());
    let rb = redact_bundle(&bundle, &force, false).unwrap();
    let entry = rb.entries.iter().find(|e| e.key == "my_custom_field").unwrap();
    assert_eq!(entry.value, "[REDACTED]");
    assert_eq!(rb.redacted_count, 1);
}

#[test]
fn test_format_redacted_output_contains_bundle_name() {
    let bundle = make_bundle(vec![("host", "localhost")]);
    let rb = redact_bundle(&bundle, &HashSet::new(), false).unwrap();
    let output = format_redacted(&rb);
    assert!(output.contains("test-bundle"));
    assert!(output.contains("host"));
    assert!(output.contains("localhost"));
}

#[test]
fn test_format_redacted_shows_redacted_count() {
    let bundle = make_bundle(vec![("secret_key", "val")]);
    let rb = redact_bundle(&bundle, &HashSet::new(), false).unwrap();
    let output = format_redacted(&rb);
    assert!(output.contains("1 value(s) redacted"));
}
