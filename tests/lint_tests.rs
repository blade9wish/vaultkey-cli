use vaultkey_cli::bundle::Bundle;
use vaultkey_cli::lint::{has_errors, lint_bundle, LintLevel};

fn make_bundle(pairs: &[(&str, &str)]) -> Bundle {
    let mut b = Bundle::new();
    for (k, v) in pairs {
        b.insert(k.to_string(), v.to_string());
    }
    b
}

#[test]
fn test_clean_bundle_no_diagnostics() {
    let bundle = make_bundle(&[("API_KEY", "supersecret123"), ("DB_PASS", "hunter2")]);
    let diags = lint_bundle(&bundle);
    assert!(diags.is_empty(), "Expected no diagnostics for a clean bundle");
}

#[test]
fn test_empty_value_produces_warning() {
    let bundle = make_bundle(&[("EMPTY_SECRET", "")]);
    let diags = lint_bundle(&bundle);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].level, LintLevel::Warning);
    assert!(diags[0].message.contains("empty"));
}

#[test]
fn test_placeholder_value_produces_warning() {
    for placeholder in &["TODO", "todo", "changeme", "FIXME", "placeholder"] {
        let bundle = make_bundle(&[("MY_KEY", placeholder)]);
        let diags = lint_bundle(&bundle);
        assert!(
            diags.iter().any(|d| d.level == LintLevel::Warning),
            "Expected warning for placeholder value '{}'",
            placeholder
        );
    }
}

#[test]
fn test_invalid_key_characters_produce_error() {
    let bundle = make_bundle(&[("bad key!", "value"), ("also bad/key", "value2")]);
    let diags = lint_bundle(&bundle);
    let errors: Vec<_> = diags.iter().filter(|d| d.level == LintLevel::Error).collect();
    assert_eq!(errors.len(), 2);
    assert!(has_errors(&diags));
}

#[test]
fn test_long_key_produces_warning() {
    let long_key = "A".repeat(65);
    let bundle = make_bundle(&[(&long_key, "some_value")]);
    let diags = lint_bundle(&bundle);
    assert!(diags.iter().any(|d| d.level == LintLevel::Warning && d.message.contains("64")));
}

#[test]
fn test_has_errors_false_for_warnings_only() {
    let bundle = make_bundle(&[("KEY", "")]);
    let diags = lint_bundle(&bundle);
    assert!(!has_errors(&diags), "Warnings should not count as errors");
}

#[test]
fn test_valid_key_formats_accepted() {
    let bundle = make_bundle(&[
        ("snake_case_key", "val1"),
        ("kebab-case-key", "val2"),
        ("MixedCase123", "val3"),
    ]);
    let diags = lint_bundle(&bundle);
    assert!(!has_errors(&diags));
}
