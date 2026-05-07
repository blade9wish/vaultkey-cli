use std::collections::HashMap;
vaultkey_cli::use_bundle;
use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::verify::{verify_bundle, VerifyIssue};

fn make_bundle(pairs: &[(&str, &str)]) -> Bundle {
    let mut secrets = HashMap::new();
    for (k, v) in pairs {
        secrets.insert(k.to_string(), v.to_string());
    }
    Bundle { secrets, ..Default::default() }
}

#[test]
fn test_verify_valid_bundle() {
    let bundle = make_bundle(&[("db_password", "s3cr3t"), ("api_key", "abc123")]);
    let result = verify_bundle(&bundle, &[]).unwrap();
    assert!(result.is_ok());
}

#[test]
fn test_verify_empty_value() {
    let bundle = make_bundle(&[("db_password", ""), ("api_key", "abc123")]);
    let result = verify_bundle(&bundle, &[]).unwrap();
    assert!(!result.is_ok());
    assert!(result
        .issues
        .iter()
        .any(|i| matches!(i, VerifyIssue::EmptyValue(k) if k == "db_password")));
}

#[test]
fn test_verify_invalid_key_format() {
    let bundle = make_bundle(&[("bad key!", "value")]);
    let result = verify_bundle(&bundle, &[]).unwrap();
    assert!(!result.is_ok());
    assert!(result
        .issues
        .iter()
        .any(|i| matches!(i, VerifyIssue::InvalidKeyFormat(_))));
}

#[test]
fn test_verify_missing_required_key() {
    let bundle = make_bundle(&[("api_key", "abc123")]);
    let result = verify_bundle(&bundle, &["db_password", "api_key"]).unwrap();
    assert!(!result.is_ok());
    assert!(result
        .issues
        .iter()
        .any(|i| matches!(i, VerifyIssue::MissingRequiredKey(k) if k == "db_password")));
}

#[test]
fn test_verify_all_required_present() {
    let bundle = make_bundle(&[("db_password", "pass"), ("api_key", "key")]);
    let result = verify_bundle(&bundle, &["db_password", "api_key"]).unwrap();
    assert!(result.is_ok());
}

#[test]
fn test_verify_issue_display() {
    let issue = VerifyIssue::DuplicateKey("my_key".to_string());
    assert_eq!(issue.to_string(), "Duplicate key: 'my_key'");

    let issue = VerifyIssue::EmptyValue("token".to_string());
    assert_eq!(issue.to_string(), "Empty value for key: 'token'");

    let issue = VerifyIssue::MissingRequiredKey("secret".to_string());
    assert_eq!(issue.to_string(), "Missing required key: 'secret'");
}

#[test]
fn test_verify_valid_key_formats() {
    let bundle = make_bundle(&[
        ("valid_key", "v1"),
        ("valid-key", "v2"),
        ("valid.key", "v3"),
        ("KEY123", "v4"),
    ]);
    let result = verify_bundle(&bundle, &[]).unwrap();
    assert!(result.is_ok());
}
