use conduit::workspaces::{
    WorkspaceBuilder,
    save::WorkspaceSaver,
};

#[test]
fn workspace_can_be_saved() {
    let workspace = WorkspaceBuilder::new()
        .name("Development")
        .profile("development")
        .build()
        .expect("workspace should be created");

    let saver = WorkspaceSaver::new();

    let data = saver
        .save(&workspace)
        .expect("workspace should be saved");

    assert!(!data.is_empty());
}

#[test]
fn saved_workspace_contains_name() {
    let workspace = WorkspaceBuilder::new()
        .name("Development")
        .build()
        .expect("workspace should be created");

    let saver = WorkspaceSaver::new();

    let data = saver
        .save(&workspace)
        .expect("workspace should be saved");

    assert!(data.contains("Development"));
}
