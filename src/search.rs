use crate::bundle::Bundle;
use crate::error::VaultKeyError;

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub term: String,
    pub case_sensitive: bool,
    pub keys_only: bool,
}

impl SearchQuery {
    pub fn new(term: &str) -> Self {
        SearchQuery {
            term: term.to_string(),
            case_sensitive: false,
            keys_only: false,
        }
    }

    pub fn case_sensitive(mut self, val: bool) -> Self {
        self.case_sensitive = val;
        self
    }

    pub fn keys_only(mut self, val: bool) -> Self {
        self.keys_only = val;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub key: String,
    pub value: Option<String>,
    pub matched_in_key: bool,
    pub matched_in_value: bool,
}

impl SearchResult {
    /// Returns a short human-readable summary of where the term matched.
    pub fn match_location(&self) -> &'static str {
        match (self.matched_in_key, self.matched_in_value) {
            (true, true) => "key and value",
            (true, false) => "key",
            (false, true) => "value",
            (false, false) => "nowhere",
        }
    }
}

pub fn search_bundle(bundle: &Bundle, query: &SearchQuery) -> Result<Vec<SearchResult>, VaultKeyError> {
    let mut results = Vec::new();

    for (k, v) in bundle.secrets.iter() {
        let (key_cmp, term_cmp, val_cmp) = if query.case_sensitive {
            (k.clone(), query.term.clone(), v.clone())
        } else {
            (k.to_lowercase(), query.term.to_lowercase(), v.to_lowercase())
        };

        let matched_in_key = key_cmp.contains(&term_cmp);
        let matched_in_value = !query.keys_only && val_cmp.contains(&term_cmp);

        if matched_in_key || matched_in_value {
            results.push(SearchResult {
                key: k.clone(),
                value: if query.keys_only { None } else { Some(v.clone()) },
                matched_in_key,
                matched_in_value,
            });
        }
    }

    results.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(results)
}
