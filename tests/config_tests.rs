use std::io::Write;
use tempfile::NamedTempFile;
use vaultkey_cli::config::VaultConfig;

fn write_config(content: &str) -> NamedTempFile {
    let mut f = NamedTempFile::new().expect("tmp file");
    write!(f, "{}", content).expect("write");
    f
}

#[test]
fn test_valid_config_loads() {
    let toml = r#"
[vault]
name = "my-vault"
version = 1

[gpg]
key_id = "ABCD1234"
armor = true

[[secrets]]
name = "db_password"
file = "secrets/db.gpg"
"#;
    let f = write_config(toml);
    let cfg = VaultConfig::load(f.path()).expect("should load");
    assert_eq!(cfg.vault.name, "my-vault");
    assert_eq!(cfg.gpg.key_id, "ABCD1234");
    assert_eq!(cfg.secrets.len(), 1);
    assert_eq!(cfg.secrets[0].name, "db_password");
}

#[test]
fn test_default_gpg_home() {
    let toml = r#"
[vault]
name = "test"
version = 1

[gpg]
key_id = "KEY123"
armor = false
"#;
    let f = write_config(toml);
    let cfg = VaultConfig::load(f.path()).expect("should load");
    assert_eq!(cfg.gpg.gpg_home, "~/.gnupg");
}

#[test]
fn test_empty_vault_name_fails_validation() {
    let toml = r#"
[vault]
name = ""
version = 1

[gpg]
key_id = "KEY123"
armor = false
"#;
    let f = write_config(toml);
    let result = VaultConfig::load(f.path());
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("vault.name"));
}

#[test]
fn test_missing_file_returns_error() {
    let result = VaultConfig::load("/nonexistent/path/vaultkey.toml");
    assert!(result.is_err());
}
