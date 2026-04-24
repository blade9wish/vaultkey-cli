use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::error::VaultError;

#[test]
fn test_bundle_creation() {
    let bundle = Bundle::new("my-secrets", Some("Test bundle".to_string()));
    assert_eq!(bundle.name, "my-secrets");
    assert_eq!(bundle.description, Some("Test bundle".to_string()));
    assert!(bundle.secrets.is_empty());
}

#[test]
fn test_add_and_get_secret() {
    let mut bundle = Bundle::new("test", None);
    bundle.add_secret("API_KEY", "super-secret-value");
    let val = bundle.get_secret("API_KEY").unwrap();
    assert_eq!(val, "super-secret-value");
}

#[test]
fn test_get_missing_secret_returns_error() {
    let bundle = Bundle::new("test", None);
    let result = bundle.get_secret("MISSING_KEY");
    assert!(matches!(result, Err(VaultError::NotFound(_))));
}

#[test]
fn test_remove_secret() {
    let mut bundle = Bundle::new("test", None);
    bundle.add_secret("DB_PASS", "password123");
    assert!(bundle.remove_secret("DB_PASS").is_ok());
    assert!(bundle.secrets.is_empty());
}

#[test]
fn test_remove_missing_secret_returns_error() {
    let mut bundle = Bundle::new("test", None);
    let result = bundle.remove_secret("NONEXISTENT");
    assert!(matches!(result, Err(VaultError::NotFound(_))));
}

#[test]
fn test_toml_roundtrip() {
    let mut bundle = Bundle::new("roundtrip-test", Some("Roundtrip bundle".to_string()));
    bundle.add_secret("KEY_ONE", "value_one");
    bundle.add_secret("KEY_TWO", "value_two");

    let toml_str = bundle.to_toml().expect("serialization failed");
    let restored = Bundle::from_toml(&toml_str).expect("deserialization failed");

    assert_eq!(restored.name, bundle.name);
    assert_eq!(restored.description, bundle.description);
    assert_eq!(restored.secrets.get("KEY_ONE"), Some(&"value_one".to_string()));
    assert_eq!(restored.secrets.get("KEY_TWO"), Some(&"value_two".to_string()));
}

#[test]
fn test_updated_at_changes_on_add() {
    let mut bundle = Bundle::new("time-test", None);
    let initial_updated = bundle.updated_at.clone();
    std::thread::sleep(std::time::Duration::from_millis(10));
    bundle.add_secret("NEW_KEY", "new_value");
    assert!(bundle.updated_at >= initial_updated);
}
