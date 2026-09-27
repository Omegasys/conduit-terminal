use conduit::workspaces::restore::WorkspaceRestorer;

#[test]
fn workspace_can_be_restored() {
    let restorer = WorkspaceRestorer::new();

    let data = r#"
        name = "Development"
        profile = "development"
        theme = "cyberpunk"
    "#;

    let workspace = restorer
        .restore(data)
        .expect("workspace should be restored");

    assert_eq!(workspace.name(), "Development");
}

#[test]
fn restored_workspace_preserves_profile() {
    let restorer = WorkspaceRestorer::new();

    let data = r#"
        name = "Development"
        profile = "development"
    "#;

    let workspace = restorer
        .restore(data)
        .expect("workspace should be restored");

    assert_eq!(workspace.profile(), Some("development"));
}

#[test]
fn invalid_workspace_data_is_rejected() {
    let restorer = WorkspaceRestorer::new();

    let result = restorer.restore("this is not valid workspace data");

    assert!(result.is_err());
}
