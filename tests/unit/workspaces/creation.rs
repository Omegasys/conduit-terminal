use conduit::workspaces::create::WorkspaceBuilder;

#[test]
fn builder_creates_workspace() {
    let workspace = WorkspaceBuilder::new()
        .name("Development")
        .build()
        .expect("workspace should be created");

    assert_eq!(workspace.name(), "Development");
}

#[test]
fn builder_sets_profile() {
    let workspace = WorkspaceBuilder::new()
        .name("Development")
        .profile("development")
        .build()
        .expect("workspace should be created");

    assert_eq!(workspace.profile(), Some("development"));
}

#[test]
fn builder_sets_theme() {
    let workspace = WorkspaceBuilder::new()
        .name("Development")
        .theme("cyberpunk")
        .build()
        .expect("workspace should be created");

    assert_eq!(workspace.theme(), Some("cyberpunk"));
}

#[test]
fn builder_can_enable_restore() {
    let workspace = WorkspaceBuilder::new()
        .name("Development")
        .restore(true)
        .build()
        .expect("workspace should be created");

    assert!(workspace.restore_enabled());
}
