use vaultkey_cli::alias::AliasMap;
use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::resolve::{resolve, ResolveSource};

fn make_bundle(pairs: &[(&str, &str)]) -> Bundle {
    let mut b = Bundle::new();
    for (k, v) in pairs {
        b.insert(k.to_string(), v.to_string());
    }
    b
}

fn make_aliases(pairs: &[(&str, &str)]) -> AliasMap {
    let mut m = AliasMap::new();
    for (alias, canonical) in pairs {
        m.insert(alias.to_string(), canonical.to_string());
    }
    m
}

#[test]
fn test_direct_bundle_lookup() {
    let bundle = make_bundle(&[("DB_PASS", "s3cr3t")]);
    let aliases = make_aliases(&[]);
    let result = resolve("DB_PASS", &bundle, &aliases, false).unwrap();
    assert_eq!(result.value, "s3cr3t");
    assert_eq!(result.source, ResolveSource::Bundle);
    assert_eq!(result.key, "DB_PASS");
}

#[test]
fn test_alias_resolution() {
    let bundle = make_bundle(&[("DATABASE_PASSWORD", "hunter2")]);
    let aliases = make_aliases(&[("db_pass", "DATABASE_PASSWORD")]);
    let result = resolve("db_pass", &bundle, &aliases, false).unwrap();
    assert_eq!(result.value, "hunter2");
    assert_eq!(result.source, ResolveSource::Alias);
    assert_eq!(result.key, "DATABASE_PASSWORD");
}

#[test]
fn test_alias_missing_canonical_falls_through_to_bundle() {
    // Alias points to a key not in bundle; direct key IS in bundle.
    let bundle = make_bundle(&[("db_pass", "direct_value")]);
    let aliases = make_aliases(&[("db_pass", "NONEXISTENT_CANONICAL")]);
    let result = resolve("db_pass", &bundle, &aliases, false).unwrap();
    assert_eq!(result.value, "direct_value");
    assert_eq!(result.source, ResolveSource::Bundle);
}

#[test]
fn test_env_fallback_used_when_not_in_bundle() {
    std::env::set_var("VAULTKEY_TEST_RESOLVE_ENV", "env_value_42");
    let bundle = make_bundle(&[]);
    let aliases = make_aliases(&[]);
    let result = resolve("VAULTKEY_TEST_RESOLVE_ENV", &bundle, &aliases, true).unwrap();
    assert_eq!(result.value, "env_value_42");
    assert_eq!(result.source, ResolveSource::Environment);
    std::env::remove_var("VAULTKEY_TEST_RESOLVE_ENV");
}

#[test]
fn test_env_fallback_disabled_returns_error() {
    std::env::set_var("VAULTKEY_TEST_NO_FALLBACK", "should_not_see_this");
    let bundle = make_bundle(&[]);
    let aliases = make_aliases(&[]);
    let result = resolve("VAULTKEY_TEST_NO_FALLBACK", &bundle, &aliases, false);
    assert!(result.is_err());
    std::env::remove_var("VAULTKEY_TEST_NO_FALLBACK");
}

#[test]
fn test_key_not_found_anywhere_returns_error() {
    let bundle = make_bundle(&[]);
    let aliases = make_aliases(&[]);
    let result = resolve("TOTALLY_MISSING", &bundle, &aliases, false);
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(msg.contains("TOTALLY_MISSING"));
}

#[test]
fn test_bundle_takes_priority_over_env() {
    std::env::set_var("VAULTKEY_PRIORITY_TEST", "env_value");
    let bundle = make_bundle(&[("VAULTKEY_PRIORITY_TEST", "bundle_value")]);
    let aliases = make_aliases(&[]);
    let result = resolve("VAULTKEY_PRIORITY_TEST", &bundle, &aliases, true).unwrap();
    assert_eq!(result.value, "bundle_value");
    assert_eq!(result.source, ResolveSource::Bundle);
    std::env::remove_var("VAULTKEY_PRIORITY_TEST");
}
