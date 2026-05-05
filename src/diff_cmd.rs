use crate::diff::{diff_bundles, DiffResult};
use crate::error::VaultKeyError;
use crate::vault::Vault;

pub struct DiffCmd {
    pub vault_a: String,
    pub vault_b: String,
    pub show_values: bool,
    pub output_format: DiffOutputFormat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffOutputFormat {
    Text,
    Json,
}

impl DiffCmd {
    pub fn run(&self) -> Result<(), VaultKeyError> {
        let vault_a = Vault::load(&self.vault_a)?;
        let vault_b = Vault::load(&self.vault_b)?;

        let results = diff_bundles(vault_a.secrets(), vault_b.secrets());

        if results.is_empty() {
            println!("No differences found between '{}' and '{}'.", self.vault_a, self.vault_b);
            return Ok(());
        }

        match self.output_format {
            DiffOutputFormat::Text => self.print_text(&results),
            DiffOutputFormat::Json => self.print_json(&results)?,
        }

        Ok(())
    }

    fn print_text(&self, results: &[DiffResult]) {
        println!("Diff: '{}' vs '{}'\n", self.vault_a, self.vault_b);
        for entry in results {
            match entry {
                DiffResult::Added(key, val) => {
                    if self.show_values {
                        println!("  [+] {} = {}", key, val);
                    } else {
                        println!("  [+] {}", key);
                    }
                }
                DiffResult::Removed(key, val) => {
                    if self.show_values {
                        println!("  [-] {} = {}", key, val);
                    } else {
                        println!("  [-] {}", key);
                    }
                }
                DiffResult::Changed(key, old, new) => {
                    if self.show_values {
                        println!("  [~] {} : {} -> {}", key, old, new);
                    } else {
                        println!("  [~] {}", key);
                    }
                }
            }
        }
    }

    fn print_json(&self, results: &[DiffResult]) -> Result<(), VaultKeyError> {
        let json = serde_json::to_string_pretty(results)
            .map_err(|e| VaultKeyError::SerializationError(e.to_string()))?;
        println!("{}", json);
        Ok(())
    }
}
