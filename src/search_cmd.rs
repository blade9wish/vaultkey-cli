use crate::bundle::Bundle;
use crate::error::VaultKeyError;
use crate::search::{search_bundle, SearchQuery};

pub struct SearchCmd {
    pub vault_path: String,
    pub term: String,
    pub case_sensitive: bool,
    pub keys_only: bool,
}

impl SearchCmd {
    pub fn new(vault_path: &str, term: &str) -> Self {
        SearchCmd {
            vault_path: vault_path.to_string(),
            term: term.to_string(),
            case_sensitive: false,
            keys_only: false,
        }
    }

    pub fn run(&self) -> Result<(), VaultKeyError> {
        let bundle = Bundle::load(&self.vault_path)?;

        let query = SearchQuery::new(&self.term)
            .case_sensitive(self.case_sensitive)
            .keys_only(self.keys_only);

        let results = search_bundle(&bundle, &query)?;

        if results.is_empty() {
            println!("No secrets found matching '{}'", self.term);
            return Ok(());
        }

        println!("Found {} result(s) for '{}':", results.len(), self.term);
        println!("{:-<40}", "");

        for result in &results {
            let mut tags = Vec::new();
            if result.matched_in_key { tags.push("key"); }
            if result.matched_in_value { tags.push("value"); }
            let tag_str = tags.join(", ");

            match &result.value {
                Some(v) => println!("  {} = {} [matched in: {}]", result.key, v, tag_str),
                None    => println!("  {} [matched in: {}]", result.key, tag_str),
            }
        }

        Ok(())
    }
}
