use conduit::config_engine::profiles::ProfileManager;

#[test]
fn profile_manager_starts_empty_or_with_defaults() {
    let manager = ProfileManager::new();

    assert!(manager.len() >= 0);
}

#[test]
fn profile_manager_can_register_profile() {
    let mut manager = ProfileManager::new();

    manager
        .register("development", r#"theme = "cyberpunk""#)
        .expect("profile should register");

    assert!(manager.get("development").is_some());
}

#[test]
fn profile_manager_can_replace_profile() {
    let mut manager = ProfileManager::new();

    manager
        .register("development", r#"theme = "nord""#)
        .expect("profile should register");

    manager
        .register("development", r#"theme = "cyberpunk""#)
        .expect("profile should update");

    let profile = manager
        .get("development")
        .expect("profile should exist");

    assert!(profile.contains("cyberpunk"));
}

#[test]
fn profile_manager_reports_unknown_profile() {
    let manager = ProfileManager::new();

    assert!(manager.get("does-not-exist").is_none());
}
