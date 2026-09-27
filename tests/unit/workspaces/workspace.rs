use conduit::workspaces::workspace::Workspace;

#[test]
fn workspace_has_unique_id() {
    let first = Workspace::new("Development");
    let second = Workspace::new("Development");

    assert_ne!(first.id(), second.id());
}

#[test]
fn workspace_preserves_name() {
    let workspace = Workspace::new("Development");

    assert_eq!(workspace.name(), "Development");
}

#[test]
fn workspace_starts_empty() {
    let workspace = Workspace::new("Development");

    assert_eq!(workspace.tab_count(), 0);
}

#[test]
fn workspace_can_be_renamed() {
    let mut workspace = Workspace::new("Development");

    workspace.rename("Server");

    assert_eq!(workspace.name(), "Server");
}
