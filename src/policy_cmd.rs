use crate::error::VaultError;
use crate::policy::Policy;
use std::collections::HashSet;

pub fn cmd_policy_check(
    policy: &Policy,
    key: &str,
    value: &str,
    tags: &[String],
) -> Result<(), VaultError> {
    policy.validate_key(key)?;
    policy.validate_secret_length(value)?;
    let tag_set: HashSet<String> = tags.iter().cloned().collect();
    policy.validate_tags(&tag_set)?;
    Ok(())
}

pub fn cmd_policy_show(policy: &Policy) -> String {
    let mut lines = vec![
        format!("Policy: {}", policy.name),
        format!("  Required tags    : {}", policy.required_tags.join(", ")),
        format!("  Forbidden keys   : {}", policy.forbidden_keys.join(", ")),
        format!(
            "  Max secret length: {}",
            policy
                .max_secret_length
                .map(|n| n.to_string())
                .unwrap_or_else(|| "unlimited".to_string())
        ),
        format!(
            "  Allowed prefixes : {}",
            if policy.allowed_key_prefixes.is_empty() {
                "any".to_string()
            } else {
                policy.allowed_key_prefixes.join(", ")
            }
        ),
    ];
    lines.join("\n")
}

pub fn cmd_policy_list(policies: &[Policy]) -> String {
    if policies.is_empty() {
        return "No policies defined.".to_string();
    }
    policies
        .iter()
        .map(|p| format!("- {}", p.name))
        .collect::<Vec<_>>()
        .join("\n")
}
