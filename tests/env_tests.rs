use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::env::{render_env, to_env_map, write_env_file, EnvFormat};
use std::collections::HashMap;
use tempfile::NamedTempFile;

fn make_bundle(secrets: &[(&str, &str)]) -> Bundle {
    let mut b = Bundle::new("test-env-bundle");
    for (k, v) in secrets {
        b.secrets.insert(k.to_string(), v.to_string());
    }
    b
}

#[test]
fn test_render_shell_format() {
    let bundle = make_bundle(&[("db-password", "secret123"), ("api.key", "abc")]);
    let lines = render_env(&bundle, &EnvFormat::Shell);
    assert!(lines.iter().any(|l| l == "export API_KEY=abc"));
    assert!(lines.iter().any(|l| l == "export DB_PASSWORD=secret123"));
}

#[test]
fn test_render_dotenv_format() {
    let bundle = make_bundle(&[("my-secret", "value1")]);
    let lines = render_env(&bundle, &EnvFormat::Dotenv);
    assert_eq!(lines, vec!["MY_SECRET=value1"]);
}

#[test]
fn test_render_sorted_output() {
    let bundle = make_bundle(&[("zebra", "z"), ("alpha", "a"), ("middle", "m")]);
    let lines = render_env(&bundle, &EnvFormat::Dotenv);
    assert_eq!(lines[0], "ALPHA=a");
    assert_eq!(lines[1], "MIDDLE=m");
    assert_eq!(lines[2], "ZEBRA=z");
}

#[test]
fn test_shell_escape_spaces() {
    let bundle = make_bundle(&[("msg", "hello world")]);
    let lines = render_env(&bundle, &EnvFormat::Shell);
    assert_eq!(lines[0], "export MSG='hello world'");
}

#[test]
fn test_shell_escape_single_quote() {
    let bundle = make_bundle(&[("val", "it's here")]);
    let lines = render_env(&bundle, &EnvFormat::Shell);
    assert!(lines[0].starts_with("export VAL="));
    assert!(lines[0].contains("it"));
}

#[test]
fn test_to_env_map() {
    let bundle = make_bundle(&[("foo-bar", "baz"), ("qux.quux", "corge")]);
    let map = to_env_map(&bundle);
    assert_eq!(map.get("FOO_BAR").map(String::as_str), Some("baz"));
    assert_eq!(map.get("QUX_QUUX").map(String::as_str), Some("corge"));
}

#[test]
fn test_write_env_file_dotenv() {
    let bundle = make_bundle(&[("token", "abc123")]);
    let tmp = NamedTempFile::new().unwrap();
    write_env_file(&bundle, &EnvFormat::Dotenv, tmp.path()).unwrap();
    let content = std::fs::read_to_string(tmp.path()).unwrap();
    assert!(content.contains("TOKEN=abc123"));
}

#[test]
fn test_empty_bundle_renders_empty() {
    let bundle = make_bundle(&[]);
    let lines = render_env(&bundle, &EnvFormat::Shell);
    assert!(lines.is_empty());
}

#[test]
fn test_env_format_default_is_shell() {
    assert_eq!(EnvFormat::default(), EnvFormat::Shell);
}
