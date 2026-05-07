use vaultkey_cli::fmt::{render_json, render_plain, render_table, OutputFormat};
use serde::Serialize;

#[test]
fn test_output_format_from_str_valid() {
    assert_eq!(OutputFormat::from_str("table").unwrap(), OutputFormat::Table);
    assert_eq!(OutputFormat::from_str("json").unwrap(), OutputFormat::Json);
    assert_eq!(OutputFormat::from_str("plain").unwrap(), OutputFormat::Plain);
    assert_eq!(OutputFormat::from_str("TABLE").unwrap(), OutputFormat::Table);
    assert_eq!(OutputFormat::from_str("JSON").unwrap(), OutputFormat::Json);
}

#[test]
fn test_output_format_from_str_invalid() {
    let result = OutputFormat::from_str("xml");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("xml"));
}

#[test]
fn test_output_format_default() {
    assert_eq!(OutputFormat::default(), OutputFormat::Table);
}

#[test]
fn test_render_table_basic() {
    let headers = vec!["Key", "Value"];
    let rows = vec![
        vec!["db_pass".to_string(), "secret123".to_string()],
        vec!["api_key".to_string(), "abc".to_string()],
    ];
    let output = render_table(&headers, &rows);
    assert!(output.contains("Key"));
    assert!(output.contains("Value"));
    assert!(output.contains("db_pass"));
    assert!(output.contains("secret123"));
    assert!(output.contains("api_key"));
}

#[test]
fn test_render_table_empty() {
    let headers = vec!["Key", "Value"];
    let rows: Vec<Vec<String>> = vec![];
    let output = render_table(&headers, &rows);
    assert_eq!(output.trim(), "(no entries)");
}

#[test]
fn test_render_table_column_width_padding() {
    let headers = vec!["K", "V"];
    let rows = vec![vec!["a_very_long_key_name".to_string(), "v".to_string()]];
    let output = render_table(&headers, &rows);
    assert!(output.contains("a_very_long_key_name"));
}

#[test]
fn test_render_json_valid() {
    #[derive(Serialize)]
    struct Sample {
        name: String,
        value: u32,
    }
    let s = Sample { name: "token".to_string(), value: 42 };
    let result = render_json(&s).unwrap();
    assert!(result.contains("\"name\""));
    assert!(result.contains("\"token\""));
    assert!(result.contains("42"));
}

#[test]
fn test_render_plain_basic() {
    let pairs = vec![
        ("host".to_string(), "localhost".to_string()),
        ("port".to_string(), "5432".to_string()),
    ];
    let output = render_plain(&pairs);
    assert!(output.contains("host: localhost"));
    assert!(output.contains("port: 5432"));
}

#[test]
fn test_render_plain_empty() {
    let pairs: Vec<(String, String)> = vec![];
    let output = render_plain(&pairs);
    assert_eq!(output, "\n");
}
