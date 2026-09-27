use conduit::config_engine::workspaces::WorkspaceConfigManager;

#[test]
fn workspace_manager_can_register_workspace() {
    let mut manager = WorkspaceConfigManager::new();

    manager
        .register("development", r#"profile = "development""#)
        .expect("workspace should register");

    assert!(manager.get("development").is_some());
}

#[test]
fn workspace_manager_can_find_workspace() {
    let mut manager = WorkspaceConfigManager::new();

    manager
        .register("server", r#"profile = "hardened""#)
        .expect("workspace should register");

    let workspace = manager
        .get("server")
        .expect("workspace should exist");

    assert!(workspace.contains("hardened"));
}

#[test]
fn workspace_manager_returns_none_for_missing_workspace() {
    let manager = WorkspaceConfigManager::new();

    assert!(manager.get("missing").is_none());
}
