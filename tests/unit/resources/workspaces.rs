use conduit::resources::workspace::WorkspaceResource;

#[test]
fn workspace_resource_can_be_created() {
    let workspace = WorkspaceResource::new("development");

    assert_eq!(workspace.name(), "development");
}

#[test]
fn workspace_resource_can_set_profile() {
    let mut workspace = WorkspaceResource::new("development");

    workspace.set_profile("development");

    assert_eq!(workspace.profile(), Some("development"));
}

#[test]
fn workspace_resource_can_set_theme() {
    let mut workspace = WorkspaceResource::new("development");

    workspace.set_theme("cyberpunk");

    assert_eq!(workspace.theme(), Some("cyberpunk"));
}

#[test]
fn workspace_resource_can_enable_restore() {
    let mut workspace = WorkspaceResource::new("development");

    workspace.set_restore(true);

    assert!(workspace.restore_enabled());
}
