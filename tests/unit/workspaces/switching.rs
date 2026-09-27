use conduit::workspaces::{
    WorkspaceBuilder,
    WorkspaceManager,
};

#[test]
fn switching_changes_active_workspace() {
    let mut manager = WorkspaceManager::new();

    let first = WorkspaceBuilder::new()
        .name("First")
        .build()
        .expect("workspace should be created");

    let second = WorkspaceBuilder::new()
        .name("Second")
        .build()
        .expect("workspace should be created");

    let first_id = first.id();
    let second_id = second.id();

    manager.add(first).expect("first should be added");
    manager.add(second).expect("second should be added");

    manager.switch_to(first_id).expect("switch should succeed");
    assert_eq!(manager.active_workspace(), Some(first_id));

    manager.switch_to(second_id).expect("switch should succeed");
    assert_eq!(manager.active_workspace(), Some(second_id));
}

#[test]
fn switching_to_unknown_workspace_fails() {
    let mut manager = WorkspaceManager::new();

    let workspace = WorkspaceBuilder::new()
        .name("Development")
        .build()
        .expect("workspace should be created");

    manager.add(workspace).expect("workspace should be added");

    let result = manager.switch_to(conduit::core::ids::WorkspaceId::new());

    assert!(result.is_err());
}
