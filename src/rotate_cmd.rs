use crate::rotate::{rotate_vault_key, RotateOptions};
use crate::crypto::GpgCryptoBackend;
use crate::audit::AuditLog;
use crate::error::VaultKeyError;

pub struct RotateCmdArgs {
    pub vault: String,
    pub old_recipient: String,
    pub new_recipient: String,
    pub dry_run: bool,
    pub audit_path: Option<String>,
}

pub fn run_rotate_cmd(args: RotateCmdArgs) -> Result<(), VaultKeyError> {
    let audit_path = args
        .audit_path
        .clone()
        .unwrap_or_else(|| "audit.log".to_string());

    let mut audit = AuditLog::open(&audit_path)?;
    let crypto = GpgCryptoBackend::new();

    let opts = RotateOptions {
        vault_path: args.vault.clone(),
        old_recipient: args.old_recipient.clone(),
        new_recipient: args.new_recipient.clone(),
        dry_run: args.dry_run,
    };

    let count = rotate_vault_key(&opts, &crypto, &mut audit)?;

    if args.dry_run {
        println!(
            "[dry-run] Would rotate {} secret(s) in '{}' from '{}' to '{}'",
            count, args.vault, args.old_recipient, args.new_recipient
        );
    } else {
        println!(
            "Rotated {} secret(s) in '{}' to new recipient '{}'",
            count, args.vault, args.new_recipient
        );
    }

    audit.flush()?;
    Ok(())
}
