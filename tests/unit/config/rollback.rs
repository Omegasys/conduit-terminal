use conduit::config_engine::rollback::RollbackManager;

#[test]
fn rollback_manager_can_create_checkpoint() {
    let mut manager = RollbackManager::new();

    let checkpoint = manager
        .checkpoint("before-change", r#"scrollback = 10000"#)
        .expect("checkpoint should succeed");

    assert!(!checkpoint.is_empty());
}

#[test]
fn rollback_manager_can_restore_checkpoint() {
    let mut manager = RollbackManager::new();

    let checkpoint = manager
        .checkpoint("before-change", r#"scrollback = 10000"#)
        .expect("checkpoint should succeed");

    let restored = manager
        .restore(&checkpoint)
        .expect("restore should succeed");

    assert_eq!(restored, r#"scrollback = 10000"#);
}

#[test]
fn rollback_manager_tracks_checkpoints() {
    let mut manager = RollbackManager::new();

    manager
        .checkpoint("one", r#"a = 1"#)
        .expect("checkpoint should succeed");

    manager
        .checkpoint("two", r#"a = 2"#)
        .expect("checkpoint should succeed");

    assert_eq!(manager.len(), 2);
}
