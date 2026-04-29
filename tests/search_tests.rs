use std::collections::HashMap;
use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::search::{search_bundle, SearchQuery};

fn make_bundle(secrets: Vec<(&str, &str)>) -> Bundle {
    let mut map = HashMap::new();
    for (k, v) in secrets {
        map.insert(k.to_string(), v.to_string());
    }
    Bundle { name: "test".to_string(), secrets: map }
}

#[test]
fn test_search_by_key() {
    let bundle = make_bundle(vec![
        ("db_password", "secret123"),
        ("api_key", "abc"),
        ("db_host", "localhost"),
    ]);
    let query = SearchQuery::new("db");
    let results = search_bundle(&bundle, &query).unwrap();
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.matched_in_key));
}

#[test]
fn test_search_by_value() {
    let bundle = make_bundle(vec![
        ("token", "my_secret_value"),
        ("host", "localhost"),
    ]);
    let query = SearchQuery::new("secret");
    let results = search_bundle(&bundle, &query).unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].matched_in_value);
    assert_eq!(results[0].key, "token");
}

#[test]
fn test_search_case_insensitive() {
    let bundle = make_bundle(vec![("API_KEY", "Value123")]);
    let query = SearchQuery::new("api_key");
    let results = search_bundle(&bundle, &query).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_case_sensitive_no_match() {
    let bundle = make_bundle(vec![("API_KEY", "Value123")]);
    let query = SearchQuery::new("api_key").case_sensitive(true);
    let results = search_bundle(&bundle, &query).unwrap();
    assert_eq!(results.len(), 0);
}

#[test]
fn test_search_keys_only_skips_value_match() {
    let bundle = make_bundle(vec![
        ("username", "admin_token"),
        ("admin_pass", "hunter2"),
    ]);
    let query = SearchQuery::new("admin").keys_only(true);
    let results = search_bundle(&bundle, &query).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].key, "admin_pass");
    assert!(results[0].value.is_none());
}

#[test]
fn test_search_empty_bundle() {
    let bundle = make_bundle(vec![]);
    let query = SearchQuery::new("anything");
    let results = search_bundle(&bundle, &query).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_search_results_sorted() {
    let bundle = make_bundle(vec![
        ("z_key", "val"),
        ("a_key", "val"),
        ("m_key", "val"),
    ]);
    let query = SearchQuery::new("key");
    let results = search_bundle(&bundle, &query).unwrap();
    let keys: Vec<&str> = results.iter().map(|r| r.key.as_str()).collect();
    assert_eq!(keys, vec!["a_key", "m_key", "z_key"]);
}
