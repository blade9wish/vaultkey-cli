//! Secret key resolution with alias and environment variable fallback.
//!
//! Provides a unified lookup that checks aliases, then the vault bundle,
//! then optionally falls back to an environment variable of the same name.

use crate::alias::AliasMap;
use crate::bundle::Bundle;
use crate::error::VaultError;

/// Resolution order used when looking up a key.
#[derive(Debug, Clone, PartialEq)]
pub enum ResolveSource {
    Alias,
    Bundle,
    Environment,
}

/// The result of a successful key resolution.
#[derive(Debug, Clone)]
pub struct Resolved {
    pub key: String,
    pub value: String,
    pub source: ResolveSource,
}

/// Resolve `key` against the given alias map, bundle, and optional env fallback.
///
/// Resolution order:
/// 1. If `key` exists in `aliases`, rewrite to the canonical key and look it up in `bundle`.
/// 2. Look up `key` directly in `bundle`.
/// 3. If `env_fallback` is `true`, look up `key` in the process environment.
pub fn resolve(
    key: &str,
    bundle: &Bundle,
    aliases: &AliasMap,
    env_fallback: bool,
) -> Result<Resolved, VaultError> {
    // Step 1 – alias rewrite
    if let Some(canonical) = aliases.get(key) {
        if let Some(value) = bundle.get(canonical) {
            return Ok(Resolved {
                key: canonical.to_string(),
                value: value.to_string(),
                source: ResolveSource::Alias,
            });
        }
    }

    // Step 2 – direct bundle lookup
    if let Some(value) = bundle.get(key) {
        return Ok(Resolved {
            key: key.to_string(),
            value: value.to_string(),
            source: ResolveSource::Bundle,
        });
    }

    // Step 3 – environment variable fallback
    if env_fallback {
        if let Ok(value) = std::env::var(key) {
            return Ok(Resolved {
                key: key.to_string(),
                value,
                source: ResolveSource::Environment,
            });
        }
    }

    Err(VaultError::NotFound(format!(
        "key '{}' could not be resolved",
        key
    )))
}
