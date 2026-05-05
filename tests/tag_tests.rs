use vaultkey_cli::tag::{Tag, TagMap};

#[test]
fn test_tag_parse_valid() {
    let tag = Tag::parse("env=production").unwrap();
    assert_eq!(tag.key, "env");
    assert_eq!(tag.value, "production");
}

#[test]
fn test_tag_parse_with_spaces() {
    let tag = Tag::parse(" team = backend ").unwrap();
    assert_eq!(tag.key, "team");
    assert_eq!(tag.value, "backend");
}

#[test]
fn test_tag_parse_value_with_equals() {
    let tag = Tag::parse("url=http://example.com?a=1").unwrap();
    assert_eq!(tag.key, "url");
    assert_eq!(tag.value, "http://example.com?a=1");
}

#[test]
fn test_tag_parse_invalid_no_equals() {
    let result = Tag::parse("nodivider");
    assert!(result.is_err());
}

#[test]
fn test_tag_parse_invalid_empty_key() {
    let result = Tag::parse("=value");
    assert!(result.is_err());
}

#[test]
fn test_tag_to_string() {
    let tag = Tag::new("env", "staging");
    assert_eq!(tag.to_string(), "env=staging");
}

#[test]
fn test_tagmap_insert_and_get() {
    let mut map = TagMap::new();
    map.insert(Tag::new("env", "prod"));
    let vals = map.get("env").unwrap();
    assert!(vals.contains(&"prod".to_string()));
}

#[test]
fn test_tagmap_multiple_values_same_key() {
    let mut map = TagMap::new();
    map.insert(Tag::new("env", "prod"));
    map.insert(Tag::new("env", "staging"));
    let vals = map.get("env").unwrap();
    assert_eq!(vals.len(), 2);
}

#[test]
fn test_tagmap_contains() {
    let mut map = TagMap::new();
    map.insert(Tag::new("team", "infra"));
    assert!(map.contains(&Tag::new("team", "infra")));
    assert!(!map.contains(&Tag::new("team", "frontend")));
}

#[test]
fn test_tagmap_remove() {
    let mut map = TagMap::new();
    map.insert(Tag::new("region", "us-east-1"));
    assert!(map.remove("region"));
    assert!(!map.remove("region"));
    assert!(map.get("region").is_none());
}

#[test]
fn test_tagmap_all_tags() {
    let mut map = TagMap::new();
    map.insert(Tag::new("env", "prod"));
    map.insert(Tag::new("team", "backend"));
    let all = map.all_tags();
    assert_eq!(all.len(), 2);
}

#[test]
fn test_tagmap_len_and_is_empty() {
    let mut map = TagMap::new();
    assert!(map.is_empty());
    map.insert(Tag::new("k", "v"));
    assert!(!map.is_empty());
    assert_eq!(map.len(), 1);
}
