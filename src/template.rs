use std::collections::HashMap;
use crate::error::VaultError;

/// Renders a template string by substituting `{{KEY}}` placeholders
/// with values from the provided secrets map.
#[derive(Debug, Clone)]
pub struct TemplateRenderer {
    pub open_delim: String,
    pub close_delim: String,
}

impl Default for TemplateRenderer {
    fn default() -> Self {
        Self {
            open_delim: "{{{".to_string(),
            close_delim: "}}}".to_string(),
        }
    }
}

impl TemplateRenderer {
    pub fn new(open_delim: &str, close_delim: &str) -> Self {
        Self {
            open_delim: open_delim.to_string(),
            close_delim: close_delim.to_string(),
        }
    }

    /// Render a template string, replacing all delimited keys with their values.
    /// Returns an error if a referenced key is not found in the secrets map.
    pub fn render(
        &self,
        template: &str,
        secrets: &HashMap<String, String>,
    ) -> Result<String, VaultError> {
        let mut output = template.to_string();
        let mut start = 0;

        while let Some(open_pos) = output[start..].find(&self.open_delim) {
            let abs_open = start + open_pos;
            let search_from = abs_open + self.open_delim.len();
            if let Some(close_pos) = output[search_from..].find(&self.close_delim) {
                let abs_close = search_from + close_pos;
                let key = &output[search_from..abs_close].trim().to_string();
                let value = secrets.get(key.as_str()).ok_or_else(|| {
                    VaultError::Generic(format!("Template key not found in secrets: '{key}'"))
                })?;
                let full_placeholder = format!(
                    "{}{}{}",
                    self.open_delim,
                    &output[search_from..abs_close],
                    self.close_delim
                );
                output = output.replacen(&full_placeholder, value, 1);
                // Don't advance start; replacement may be shorter/longer
            } else {
                break;
            }
        }

        Ok(output)
    }

    /// Extract all placeholder keys referenced in a template string.
    pub fn extract_keys(&self, template: &str) -> Vec<String> {
        let mut keys = Vec::new();
        let mut search = template;
        while let Some(open_pos) = search.find(&self.open_delim) {
            let after_open = &search[open_pos + self.open_delim.len()..];
            if let Some(close_pos) = after_open.find(&self.close_delim) {
                let key = after_open[..close_pos].trim().to_string();
                if !key.is_empty() {
                    keys.push(key);
                }
                search = &after_open[close_pos + self.close_delim.len()..];
            } else {
                break;
            }
        }
        keys
    }
}
