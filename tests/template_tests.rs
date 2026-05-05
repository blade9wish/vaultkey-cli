use std::collections::HashMap;
use vaultkey_cli::template::TemplateRenderer;

fn sample_secrets() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("DB_HOST".to_string(), "localhost".to_string());
    m.insert("DB_PORT".to_string(), "5432".to_string());
    m.insert("APP_SECRET".to_string(), "s3cr3t!".to_string());
    m
}

#[test]
fn test_render_simple_substitution() {
    let renderer = TemplateRenderer::default();
    let tmpl = "host={{{DB_HOST}}} port={{{DB_PORT}}}";
    let result = renderer.render(tmpl, &sample_secrets()).unwrap();
    assert_eq!(result, "host=localhost port=5432");
}

#[test]
fn test_render_multiple_occurrences() {
    let renderer = TemplateRenderer::default();
    let tmpl = "{{{DB_HOST}}}:{{{DB_PORT}}}/{{{DB_HOST}}}";
    let result = renderer.render(tmpl, &sample_secrets()).unwrap();
    assert_eq!(result, "localhost:5432/localhost");
}

#[test]
fn test_render_missing_key_returns_error() {
    let renderer = TemplateRenderer::default();
    let tmpl = "value={{{MISSING_KEY}}}";
    let err = renderer.render(tmpl, &sample_secrets()).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("MISSING_KEY"), "Expected key name in error: {msg}");
}

#[test]
fn test_render_no_placeholders() {
    let renderer = TemplateRenderer::default();
    let tmpl = "no placeholders here";
    let result = renderer.render(tmpl, &sample_secrets()).unwrap();
    assert_eq!(result, "no placeholders here");
}

#[test]
fn test_render_whitespace_trimmed_key() {
    let renderer = TemplateRenderer::default();
    let tmpl = "{{{ APP_SECRET }}}";
    let result = renderer.render(tmpl, &sample_secrets()).unwrap();
    assert_eq!(result, "s3cr3t!");
}

#[test]
fn test_extract_keys_basic() {
    let renderer = TemplateRenderer::default();
    let tmpl = "connect {{{DB_HOST}}}:{{{DB_PORT}}} secret={{{APP_SECRET}}}";
    let keys = renderer.extract_keys(tmpl);
    assert_eq!(keys, vec!["DB_HOST", "DB_PORT", "APP_SECRET"]);
}

#[test]
fn test_extract_keys_empty_template() {
    let renderer = TemplateRenderer::default();
    let keys = renderer.extract_keys("no keys here");
    assert!(keys.is_empty());
}

#[test]
fn test_custom_delimiters() {
    let renderer = TemplateRenderer::new("${{", "}}");
    let mut secrets = HashMap::new();
    secrets.insert("NAME".to_string(), "world".to_string());
    let result = renderer.render("Hello ${{NAME}}!", &secrets).unwrap();
    assert_eq!(result, "Hello world!");
}

#[test]
fn test_extract_keys_deduplication_not_applied() {
    // extract_keys returns keys in order, including duplicates
    let renderer = TemplateRenderer::default();
    let tmpl = "{{{A}}} {{{B}}} {{{A}}}";
    let keys = renderer.extract_keys(tmpl);
    assert_eq!(keys, vec!["A", "B", "A"]);
}
