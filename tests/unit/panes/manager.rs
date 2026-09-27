use conduit::panes::{
    PaneBuilder,
    PaneManager,
};

#[test]
fn pane_manager_starts_empty() {
    let manager = PaneManager::new();

    assert_eq!(manager.len(), 0);
}

#[test]
fn pane_manager_can_add_pane() {
    let mut manager = PaneManager::new();

    let pane = PaneBuilder::new()
        .title("Terminal")
        .build()
        .expect("pane should be created");

    let id = pane.id();

    manager
        .add(pane)
        .expect("pane should be added");

    assert!(manager.get(id).is_some());
    assert_eq!(manager.len(), 1);
}

#[test]
fn pane_manager_can_remove_pane() {
    let mut manager = PaneManager::new();

    let pane = PaneBuilder::new()
        .build()
        .expect("pane should be created");

    let id = pane.id();

    manager
        .add(pane)
        .expect("pane should be added");

    manager
        .remove(id)
        .expect("pane should be removed");

    assert!(manager.get(id).is_none());
    assert_eq!(manager.len(), 0);
}

#[test]
fn pane_manager_can_focus_pane() {
    let mut manager = PaneManager::new();

    let pane = PaneBuilder::new()
        .build()
        .expect("pane should be created");

    let id = pane.id();

    manager
        .add(pane)
        .expect("pane should be added");

    manager
        .focus(id)
        .expect("pane should be focused");

    assert_eq!(manager.focused_pane(), Some(id));
}

#[test]
fn only_one_pane_is_focused() {
    let mut manager = PaneManager::new();

    let first = PaneBuilder::new()
        .build()
        .expect("first pane should be created");

    let second = PaneBuilder::new()
        .build()
        .expect("second pane should be created");

    let first_id = first.id();
    let second_id = second.id();

    manager.add(first).expect("first pane should be added");
    manager.add(second).expect("second pane should be added");

    manager
        .focus(first_id)
        .expect("first pane should be focused");

    manager
        .focus(second_id)
        .expect("second pane should be focused");

    assert_eq!(manager.focused_pane(), Some(second_id));
    assert!(!manager.get(first_id).unwrap().is_active());
    assert!(manager.get(second_id).unwrap().is_active());
}
