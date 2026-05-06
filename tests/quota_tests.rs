use std::collections::HashMap;
use vaultkey_cli::quota::{check_quota, check_secret_size, QuotaPolicy, QuotaUsage};
use vaultkey_cli::quota_cmd::{build_usage_from_bundles, cmd_quota_check};

fn make_usage(secrets: usize, bundles: usize, size: usize) -> QuotaUsage {
    QuotaUsage {
        secret_count: secrets,
        bundle_count: bundles,
        total_size_bytes: size,
        per_bundle: HashMap::new(),
    }
}

#[test]
fn test_quota_within_limits() {
    let policy = QuotaPolicy {
        max_secrets: Some(100),
        max_bundles: Some(10),
        max_secret_size_bytes: Some(1024),
        max_total_size_bytes: Some(10_000),
    };
    let usage = make_usage(50, 5, 4096);
    assert!(check_quota(&policy, &usage).is_ok());
}

#[test]
fn test_quota_exceeds_secrets() {
    let policy = QuotaPolicy {
        max_secrets: Some(10),
        max_bundles: Some(100),
        max_secret_size_bytes: None,
        max_total_size_bytes: None,
    };
    let usage = make_usage(10, 1, 0);
    let result = check_quota(&policy, &usage);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("max_secrets"));
}

#[test]
fn test_quota_exceeds_total_size() {
    let policy = QuotaPolicy {
        max_secrets: None,
        max_bundles: None,
        max_secret_size_bytes: None,
        max_total_size_bytes: Some(1000),
    };
    let usage = make_usage(1, 1, 1000);
    let result = check_quota(&policy, &usage);
    assert!(result.is_err());
}

#[test]
fn test_secret_size_within_limit() {
    let policy = QuotaPolicy {
        max_secret_size_bytes: Some(512),
        ..Default::default()
    };
    assert!(check_secret_size(&policy, 256).is_ok());
}

#[test]
fn test_secret_size_exceeds_limit() {
    let policy = QuotaPolicy {
        max_secret_size_bytes: Some(128),
        ..Default::default()
    };
    let result = check_secret_size(&policy, 256);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exceeds limit"));
}

#[test]
fn test_build_usage_from_bundles() {
    let mut bundles: HashMap<String, Vec<String>> = HashMap::new();
    bundles.insert("alpha".to_string(), vec!["key1=val1".to_string(), "key2=val2".to_string()]);
    bundles.insert("beta".to_string(), vec!["x=y".to_string()]);

    let usage = build_usage_from_bundles(&bundles);
    assert_eq!(usage.bundle_count, 2);
    assert_eq!(usage.secret_count, 3);
    assert!(usage.total_size_bytes > 0);
    assert_eq!(usage.per_bundle["alpha"], 2);
    assert_eq!(usage.per_bundle["beta"], 1);
}

#[test]
fn test_cmd_quota_check_passes() {
    let policy = QuotaPolicy::default();
    let usage = make_usage(10, 2, 500);
    assert!(cmd_quota_check(&usage, &policy).is_ok());
}

#[test]
fn test_default_policy_values() {
    let policy = QuotaPolicy::default();
    assert_eq!(policy.max_secrets, Some(500));
    assert_eq!(policy.max_bundles, Some(50));
    assert_eq!(policy.max_secret_size_bytes, Some(4096));
    assert_eq!(policy.max_total_size_bytes, Some(1_048_576));
}
