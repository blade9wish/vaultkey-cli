use std::path::PathBuf;
use clap::Args;
use crate::error::VaultKeyError;
use crate::vault::Vault;
use crate::import::{import_from_toml, import_from_env_file};

#[derive(Debug, Args)]
pub struct ImportArgs {
    /// Path to the file to import
    #[arg(short, long)]
    pub file: PathBuf,

    /// Import format: toml or env
    #[arg(short = 'F', long, default_value = "toml")]
    pub format: String,

    /// Vault file to import into
    #[arg(short, long)]
    pub vault: Option<PathBuf>,

    /// Dry run — show what would be imported without saving
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

pub fn run_import(args: ImportArgs) -> Result<(), VaultKeyError> {
    let vault_path = args.vault.unwrap_or_else(|| PathBuf::from("vault.toml"));

    let mut vault = if vault_path.exists() {
        Vault::load(&vault_path)?
    } else {
        Vault::new()
    };

    if args.dry_run {
        println!("[dry-run] Reading from: {}", args.file.display());
        println!("[dry-run] Format: {}", args.format);
        println!("[dry-run] No changes will be saved.");
        return Ok(());
    }

    let count = match args.format.as_str() {
        "toml" => import_from_toml(&args.file, &mut vault)?,
        "env" => import_from_env_file(&args.file, &mut vault)?,
        other => {
            return Err(VaultKeyError::Config(format!(
                "Unknown import format: '{}'. Use 'toml' or 'env'.",
                other
            )));
        }
    };

    vault.save(&vault_path)?;

    println!(
        "Successfully imported {} secret(s) into {}",
        count,
        vault_path.display()
    );

    Ok(())
}
