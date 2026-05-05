use crate::access::{AccessPolicy, AccessRule, Permission, parse_permissions};
use crate::access_store::AccessStore;
use crate::error::VaultError;

pub fn cmd_grant(
    store: &AccessStore,
    identity: &str,
    bundle_pattern: &str,
    perms: &[&str],
) -> Result<(), VaultError> {
    let mut policy = store.load()?;
    let permissions = parse_permissions(perms)?;
    policy.add_rule(AccessRule {
        identity: identity.to_string(),
        permissions,
        bundle_pattern: bundle_pattern.to_string(),
    });
    store.save(&policy)?;
    println!("Granted [{}] on '{}' to '{}'", perms.join(", "), bundle_pattern, identity);
    Ok(())
}

pub fn cmd_revoke(
    store: &AccessStore,
    identity: &str,
    bundle_pattern: &str,
) -> Result<(), VaultError> {
    let mut policy = store.load()?;
    if policy.remove_rule(identity, bundle_pattern) {
        store.save(&policy)?;
        println!("Revoked access for '{}' on '{}'", identity, bundle_pattern);
    } else {
        println!("No matching rule found for '{}' on '{}'", identity, bundle_pattern);
    }
    Ok(())
}

pub fn cmd_check(
    store: &AccessStore,
    identity: &str,
    bundle: &str,
    perm: &str,
) -> Result<(), VaultError> {
    let policy = store.load()?;
    let permission = parse_permissions(&[perm])?.remove(0);
    if policy.check(identity, bundle, &permission) {
        println!("ALLOWED: '{}' has '{}' on '{}'", identity, perm, bundle);
    } else {
        println!("DENIED: '{}' lacks '{}' on '{}'", identity, perm, bundle);
    }
    Ok(())
}

pub fn cmd_list(store: &AccessStore, identity: Option<&str>) -> Result<(), VaultError> {
    let policy = store.load()?;
    let rules: Vec<_> = match identity {
        Some(id) => policy.list_for_identity(id).into_iter().cloned().collect(),
        None => policy.rules.clone(),
    };
    if rules.is_empty() {
        println!("No access rules defined.");
    } else {
        for rule in &rules {
            let perms: Vec<String> = rule.permissions.iter().map(|p| format!("{:?}", p).to_lowercase()).collect();
            println!("  {} -> {} [{}]", rule.identity, rule.bundle_pattern, perms.join(", "));
        }
    }
    Ok(())
}
