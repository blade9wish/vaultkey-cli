use vaultkey_cli::notify::{
    dispatch, NotifyChannel, NotifyEvent, NotifyRule,
};
use vaultkey_cli::notify_store::NotifyStore;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_notify_event_display() {
    assert_eq!(
        NotifyEvent::SecretAccessed("db_pass".into()).to_string(),
        "[ACCESS] Secret accessed: db_pass"
    );
    assert_eq!(
        NotifyEvent::SecretRotated("api_key".into()).to_string(),
        "[ROTATE] Secret rotated: api_key"
    );
    assert_eq!(
        NotifyEvent::PolicyViolation("min_len".into()).to_string(),
        "[POLICY] Policy violation: min_len"
    );
}

#[test]
fn test_dispatch_stdout_rule() {
    let rules = vec![NotifyRule {
        event: "access".to_string(),
        channel: NotifyChannel::Stdout,
        filter: None,
    }];
    let event = NotifyEvent::SecretAccessed("my_key".into());
    assert!(dispatch(&event, &rules).is_ok());
}

#[test]
fn test_dispatch_wildcard_matches_all() {
    let rules = vec![NotifyRule {
        event: "*".to_string(),
        channel: NotifyChannel::Stdout,
        filter: None,
    }];
    let events = vec![
        NotifyEvent::SecretAccessed("k".into()),
        NotifyEvent::SecretRotated("k".into()),
        NotifyEvent::SecretExpired("k".into()),
    ];
    for ev in &events {
        assert!(dispatch(ev, &rules).is_ok());
    }
}

#[test]
fn test_dispatch_file_channel() {
    let dir = tempdir().unwrap();
    let log_path = dir.path().join("notify.log");
    let rules = vec![NotifyRule {
        event: "rotate".to_string(),
        channel: NotifyChannel::File(log_path.to_str().unwrap().to_string()),
        filter: None,
    }];
    let event = NotifyEvent::SecretRotated("token".into());
    dispatch(&event, &rules).unwrap();
    let content = fs::read_to_string(&log_path).unwrap();
    assert!(content.contains("[ROTATE] Secret rotated: token"));
}

#[test]
fn test_dispatch_no_matching_rules() {
    let rules = vec![NotifyRule {
        event: "expire".to_string(),
        channel: NotifyChannel::Stdout,
        filter: None,
    }];
    let event = NotifyEvent::SecretAccessed("k".into());
    assert!(dispatch(&event, &rules).is_ok());
}

#[test]
fn test_notify_store_add_and_list() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("notify.toml");
    let mut store = NotifyStore::load(&path).unwrap();
    store.add_rule("access", NotifyChannel::Stdout, None);
    store.add_rule("rotate", NotifyChannel::File("/tmp/r.log".into()), None);
    store.save(&path).unwrap();

    let loaded = NotifyStore::load(&path).unwrap();
    assert_eq!(loaded.list().len(), 2);
    assert_eq!(loaded.list()[0].event, "access");
}

#[test]
fn test_notify_store_remove_rules() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("notify.toml");
    let mut store = NotifyStore::default();
    store.add_rule("access", NotifyChannel::Stdout, None);
    store.add_rule("access", NotifyChannel::File("/tmp/a.log".into()), None);
    store.add_rule("rotate", NotifyChannel::Stdout, None);
    let removed = store.remove_rules_for_event("access");
    assert_eq!(removed, 2);
    assert_eq!(store.list().len(), 1);
    store.save(&path).unwrap();
}

#[test]
fn test_notify_store_default_empty() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("nonexistent.toml");
    let store = NotifyStore::load(&path).unwrap();
    assert!(store.list().is_empty());
}
