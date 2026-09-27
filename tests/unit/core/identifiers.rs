use conduit::core::ids::{PaneId, SessionId, TabId, WindowId, WorkspaceId};

#[test]
fn window_ids_are_unique() {
    let first = WindowId::new();
    let second = WindowId::new();

    assert_ne!(first, second);
}

#[test]
fn tab_ids_are_unique() {
    let first = TabId::new();
    let second = TabId::new();

    assert_ne!(first, second);
}

#[test]
fn pane_ids_are_unique() {
    let first = PaneId::new();
    let second = PaneId::new();

    assert_ne!(first, second);
}

#[test]
fn workspace_ids_are_unique() {
    let first = WorkspaceId::new();
    let second = WorkspaceId::new();

    assert_ne!(first, second);
}

#[test]
fn session_ids_are_unique() {
    let first = SessionId::new();
    let second = SessionId::new();

    assert_ne!(first, second);
}

#[test]
fn identifiers_are_copyable() {
    let window = WindowId::new();
    let copied = window;

    assert_eq!(window, copied);
}
