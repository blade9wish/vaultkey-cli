use std::collections::HashMap;
use std::fs;
use tempfile::tempdir;

#[cfg(test)]
mod rotate_tests {
    use super::*;
    use vaultkey_cli::rotate::{rotate_vault_key, RotateOptions};
    use vaultkey_cli::audit::AuditLog;
    use vaultkey_cli::vault::Vault;
    use vaultkey_cli::crypto::MockCryptoBackend;

    fn make_vault_with_secrets(path: &str, secrets: HashMap<String, String>) {
        let mut vault = Vault::new(path);
        for (k, v) in secrets {
            vault.secrets.insert(k, v);
        }
        vault.save(path).expect("Failed to save vault");
    }

    #[test]
    fn test_rotate_dry_run_returns_count() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path().join("test.vault").to_str().unwrap().to_string();
        let audit_path = dir.path().join("audit.log").to_str().unwrap().to_string();

        let mut secrets = HashMap::new();
        secrets.insert("api_key".to_string(), "encrypted_val_1".to_string());
        secrets.insert("db_pass".to_string(), "encrypted_val_2".to_string());
        make_vault_with_secrets(&vault_path, secrets);

        let mut audit = AuditLog::open(&audit_path).unwrap();
        let crypto = MockCryptoBackend::new();

        let opts = RotateOptions {
            vault_path: vault_path.clone(),
            old_recipient: "old@example.com".to_string(),
            new_recipient: "new@example.com".to_string(),
            dry_run: true,
        };

        let result = rotate_vault_key(&opts, &crypto, &mut audit);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_rotate_nonexistent_vault_errors() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path().join("missing.vault").to_str().unwrap().to_string();
        let audit_path = dir.path().join("audit.log").to_str().unwrap().to_string();

        let mut audit = AuditLog::open(&audit_path).unwrap();
        let crypto = MockCryptoBackend::new();

        let opts = RotateOptions {
            vault_path,
            old_recipient: "old@example.com".to_string(),
            new_recipient: "new@example.com".to_string(),
            dry_run: false,
        };

        let result = rotate_vault_key(&opts, &crypto, &mut audit);
        assert!(result.is_err());
    }

    #[test]
    fn test_rotate_updates_all_secrets() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path().join("rotate.vault").to_str().unwrap().to_string();
        let audit_path = dir.path().join("audit.log").to_str().unwrap().to_string();

        let mut secrets = HashMap::new();
        secrets.insert("token".to_string(), "enc::oldtoken".to_string());
        make_vault_with_secrets(&vault_path, secrets);

        let mut audit = AuditLog::open(&audit_path).unwrap();
        let crypto = MockCryptoBackend::new();

        let opts = RotateOptions {
            vault_path: vault_path.clone(),
            old_recipient: "old@example.com".to_string(),
            new_recipient: "new@example.com".to_string(),
            dry_run: false,
        };

        let result = rotate_vault_key(&opts, &crypto, &mut audit);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let updated = Vault::load(&vault_path).unwrap();
        let token_val = updated.secrets.get("token").unwrap();
        assert!(token_val.contains("new@example.com") || !token_val.contains("old"));
    }
}
