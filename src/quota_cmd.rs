use crate::quota::{check_quota, QuotaPolicy, QuotaUsage};
use crate::error::VaultError;
use std::collections::HashMap;

pub fn cmd_quota_status(usage: &QuotaUsage, policy: &QuotaPolicy) {
    println!("=== Quota Status ===");
    println!(
        "Secrets : {}/{}",
        usage.secret_count,
        policy.max_secrets.map_or("unlimited".to_string(), |v| v.to_string())
    );
    println!(
        "Bundles : {}/{}",
        usage.bundle_count,
        policy.max_bundles.map_or("unlimited".to_string(), |v| v.to_string())
    );
    println!(
        "Total Size: {} bytes / {}",
        usage.total_size_bytes,
        policy.max_total_size_bytes.map_or("unlimited".to_string(), |v| format!("{} bytes", v))
    );

    if !usage.per_bundle.is_empty() {
        println!("\nPer-bundle secret counts:");
        let mut entries: Vec<_> = usage.per_bundle.iter().collect();
        entries.sort_by_key(|(k, _)| k.as_str());
        for (bundle, count) in entries {
            println!("  {}: {}", bundle, count);
        }
    }

    match check_quota(policy, usage) {
        Ok(_) => println!("\n[OK] All quotas within limits."),
        Err(e) => println!("\n[WARN] {}", e),
    }
}

pub fn cmd_quota_check(usage: &QuotaUsage, policy: &QuotaPolicy) -> Result<(), VaultError> {
    check_quota(policy, usage).map_err(|e| {
        eprintln!("Quota check failed: {}", e);
        e
    })
}

pub fn build_usage_from_bundles(bundles: &HashMap<String, Vec<String>>) -> QuotaUsage {
    let bundle_count = bundles.len();
    let mut secret_count = 0usize;
    let mut total_size_bytes = 0usize;
    let mut per_bundle: HashMap<String, usize> = HashMap::new();

    for (name, secrets) in bundles {
        let count = secrets.len();
        secret_count += count;
        per_bundle.insert(name.clone(), count);
        for s in secrets {
            total_size_bytes += s.len();
        }
    }

    QuotaUsage {
        secret_count,
        bundle_count,
        total_size_bytes,
        per_bundle,
    }
}
