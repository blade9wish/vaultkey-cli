use std::collections::HashMap;
use crate::error::VaultKeyError;

/// Represents a tag attached to a secret entry
#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tag {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Tag {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Parse a tag from "key=value" format
    pub fn parse(raw: &str) -> Result<Self, VaultKeyError> {
        let parts: Vec<&str> = raw.splitn(2, '=').collect();
        if parts.len() != 2 || parts[0].is_empty() {
            return Err(VaultKeyError::InvalidInput(format!(
                "Invalid tag format '{}': expected key=value",
                raw
            )));
        }
        Ok(Tag::new(parts[0].trim(), parts[1].trim()))
    }

    pub fn to_string(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

/// Manages a collection of tags for secret entries
#[derive(Debug, Default, Clone)]
pub struct TagMap {
    inner: HashMap<String, Vec<String>>,
}

impl TagMap {
    pub fn new() -> Self {
        TagMap {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, tag: Tag) {
        self.inner
            .entry(tag.key)
            .or_insert_with(Vec::new)
            .push(tag.value);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.inner.get(key)
    }

    pub fn contains(&self, tag: &Tag) -> bool {
        self.inner
            .get(&tag.key)
            .map(|vals| vals.contains(&tag.value))
            .unwrap_or(false)
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.inner.remove(key).is_some()
    }

    pub fn all_tags(&self) -> Vec<Tag> {
        let mut result = Vec::new();
        for (k, vals) in &self.inner {
            for v in vals {
                result.push(Tag::new(k.clone(), v.clone()));
            }
        }
        result
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.values().map(|v| v.len()).sum()
    }
}
