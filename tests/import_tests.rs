use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;
use vaultkey_cli::import::{import_from_toml, import_from_env_file};
use vaultkey_cli::vault::Vault;

fn make_vault() -> Vault {
    Vault::new()
}

#[test]
fn test_import_from_valid_toml() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("import.toml");
    fs::write(&file, r#"
format = "vaultkey"
version = 1

[[secrets]]
key = "DB_PASSWORD"
value = "s3cr3t"
tags = ["database"]
description = "Main DB password"

[[secrets]]
key = "API_TOKEN"
value = "tok_abc123"
"#).unwrap();

    let mut vault = make_vault();
    let count = import_from_toml(&file, &mut vault).unwrap();
    assert_eq!(count, 2);
    assert!(vault.get_bundle("DB_PASSWORD").is_some());
    assert!(vault.get_bundle("API_TOKEN").is_some());
}

#[test]
fn test_import_toml_wrong_format_fails() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("bad.toml");
    fs::write(&file, r#"
format = "other"
[[secrets]]
key = "X"
value = "y"
"#).unwrap();

    let mut vault = make_vault();
    let result = import_from_toml(&file, &mut vault);
    assert!(result.is_err());
}

#[test]
fn test_import_from_env_file() {
    let dir = tempdir().unwrap();
    let file = dir.path().join(".env");
    fs::write(&file, "# comment\nDB_HOST=localhost\nDB_PORT=\"5432\"\n\nSECRET_KEY=abc").unwrap();

    let mut vault = make_vault();
    let count = import_from_env_file(&file, &mut vault).unwrap();
    assert_eq!(count, 3);
    assert!(vault.get_bundle("DB_HOST").is_some());
    assert!(vault.get_bundle("DB_PORT").is_some());
    assert!(vault.get_bundle("SECRET_KEY").is_some());
}

#[test]
fn test_import_env_skips_empty_and_comments() {
    let dir = tempdir().unwrap();
    let file = dir.path().join(".env");
    fs::write(&file, "# this is a comment\n\n   \nONLY_KEY=value").unwrap();

    let mut vault = make_vault();
    let count = import_from_env_file(&file, &mut vault).unwrap();
    assert_eq!(count, 1);
}

#[test]
fn test_import_missing_file_returns_error() {
    let mut vault = make_vault();
    let result = import_from_toml(&PathBuf::from("/nonexistent/file.toml"), &mut vault);
    assert!(result.is_err());
}
