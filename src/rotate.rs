use crate::error::VaultKeyError;
use crate::vault::Vault;
use crate::crypto::CryptoBackend;
use crate::audit::{AuditLog, AuditEvent};
use std::path::Path;

/// Rotates the encryption key for a vault bundle by re-encrypting all secrets
/// with a new recipient or passphrase.
pub struct RotateOptions {
    pub vault_path: String,
    pub old_recipient: String,
    pub new_recipient: String,
    pub dry_run: bool,
}

pub fn rotate_vault_key(
    opts: &RotateOptions,
    crypto: &dyn CryptoBackend,
    audit: &mut AuditLog,
) -> Result<usize, VaultKeyError> {
    let vault_path = Path::new(&opts.vault_path);
    if !vault_path.exists() {
        return Err(VaultKeyError::NotFound(format!(
            "Vault not found: {}",
            opts.vault_path
        )));
    }

    let mut vault = Vault::load(&opts.vault_path)?;
    let secret_count = vault.secrets.len();

    if opts.dry_run {
        audit.record(AuditEvent::new(
            "rotate_dry_run",
            &format!("Would rotate {} secrets from {} to {}", secret_count, opts.old_recipient, opts.new_recipient),
        ));
        return Ok(secret_count);
    }

    let mut rotated = 0;
    for (key, ciphertext) in vault.secrets.iter_mut() {
        let plaintext = crypto.decrypt(ciphertext, &opts.old_recipient)?;
        let new_ciphertext = crypto.encrypt(&plaintext, &opts.new_recipient)?;
        *ciphertext = new_ciphertext;
        rotated += 1;
        audit.record(AuditEvent::new(
            "rotate_secret",
            &format!("Rotated secret '{}' to new recipient", key),
        ));
    }

    vault.save(&opts.vault_path)?;

    audit.record(AuditEvent::new(
        "rotate_complete",
        &format!("Rotated {}/{} secrets in vault '{}'", rotated, secret_count, opts.vault_path),
    ));

    Ok(rotated)
}
