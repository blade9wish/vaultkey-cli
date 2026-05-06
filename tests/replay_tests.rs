//! Tests for the replay protection module.

use vaultkey_cli::replay::ReplayGuard;

#[test]
test_new_guard_starts_empty() {
    let guard = ReplayGuard::new(300);
    assert_eq!(guard.active_count(), 0);
}

#[test]
fn test_register_nonce_succeeds_first_time() {
    let mut guard = ReplayGuard::new(300);
    let result = guard.check_and_register("nonce-abc");
    assert!(result.is_ok());
    assert_eq!(guard.active_count(), 1);
}

#[test]
fn test_replay_detected_on_duplicate_nonce() {
    let mut guard = ReplayGuard::new(300);
    guard.check_and_register("nonce-dup").unwrap();
    let result = guard.check_and_register("nonce-dup");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("Replay detected"));
    assert!(msg.contains("nonce-dup"));
}

#[test]
fn test_different_nonces_both_register() {
    let mut guard = ReplayGuard::new(300);
    guard.check_and_register("alpha").unwrap();
    guard.check_and_register("beta").unwrap();
    assert_eq!(guard.active_count(), 2);
}

#[test]
fn test_purge_expired_removes_old_entries() {
    // Use a zero-second window so all entries are immediately expired.
    let mut guard = ReplayGuard::new(0);
    guard.check_and_register("stale-nonce").unwrap();
    // Sleep briefly to ensure timestamp advances.
    std::thread::sleep(std::time::Duration::from_millis(10));
    guard.purge_expired();
    assert_eq!(guard.active_count(), 0);
}

#[test]
fn test_purge_keeps_fresh_entries() {
    let mut guard = ReplayGuard::new(3600);
    guard.check_and_register("fresh-nonce").unwrap();
    guard.purge_expired();
    assert_eq!(guard.active_count(), 1);
}

#[test]
fn test_register_after_purge_succeeds() {
    let mut guard = ReplayGuard::new(0);
    guard.check_and_register("reuse-after-expiry").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    guard.purge_expired();
    // After expiry, the same nonce should be registerable again.
    let result = guard.check_and_register("reuse-after-expiry");
    assert!(result.is_ok());
}
