use conduit::workspaces::state::WorkspaceState;

#[test]
fn workspace_state_defaults_to_active() {
    assert_eq!(WorkspaceState::default(), WorkspaceState::Active);
}

#[test]
fn workspace_state_supports_active() {
    assert_eq!(WorkspaceState::Active, WorkspaceState::Active);
}

#[test]
fn workspace_state_supports_inactive() {
    assert_ne!(WorkspaceState::Inactive, WorkspaceState::Active);
}

#[test]
fn workspace_state_supports_restoring() {
    assert_ne!(WorkspaceState::Restoring, WorkspaceState::Active);
}

#[test]
fn workspace_state_supports_saving() {
    assert_ne!(WorkspaceState::Saving, WorkspaceState::Active);
}
