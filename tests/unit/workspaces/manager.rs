use conduit::workspaces::{
    WorkspaceBuilder,
    WorkspaceManager,
};

#[test]
fn workspace_manager_starts_empty() {
    let manager = WorkspaceManager::new();

    assert_eq!(manager.len(), 0);
}

#[test]
fn manager_can_add_workspace() {
    let mut manager = WorkspaceManager::new();

    let workspace = WorkspaceBuilder::new()
        .name("Development")
        .build()
        .expect("workspace should be created");

    let id = workspace.id();

    manager
        .add(workspace)
        .expect("workspace should be added");

    assert!(manager.get(id).is_some());
}

#[test]
fn manager_can_remove_workspace() {
    let mut manager = WorkspaceManager::new();

    let workspace = WorkspaceBuilder::new()
        .name("Development")
        .build()
        .expect("workspace should be created");

    let id = workspace.id();

    manager
        .add(workspace)
        .expect("workspace should be added");

    manager
        .remove(id)
        .expect("workspace should be removed");

    assert!(manager.get(id).is_none());
}

#[test]
fn manager_can_switch_workspace() {
    let mut manager = WorkspaceManager::new();

    let first = WorkspaceBuilder::new()
        .name("Development")
        .build()
        .expect("workspace should be created");

    let second = WorkspaceBuilder::new()
        .name("Server")
        .build()
        .expect("workspace should be created");

    let first_id = first.id();
    let second_id = second.id();

    manager.add(first).expect("first should be added");
    manager.add(second).expect("second should be added");

    manager
        .switch_to(first_id)
        .expect("first workspace should be selected");

    manager
        .switch_to(second_id)
        .expect("second workspace should be selected");

    assert_eq!(manager.active_workspace(), Some(second_id));
}
