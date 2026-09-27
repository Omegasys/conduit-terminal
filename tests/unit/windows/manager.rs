use conduit::windows::{
    WindowManager,
    WindowBuilder,
};

#[test]
fn window_manager_starts_empty() {
    let manager = WindowManager::new();

    assert_eq!(manager.len(), 0);
}

#[test]
fn window_manager_can_add_window() {
    let mut manager = WindowManager::new();

    let window = WindowBuilder::new()
        .title("Test")
        .build()
        .expect("window should be created");

    let id = window.id();

    manager
        .add(window)
        .expect("window should be added");

    assert!(manager.get(id).is_some());
    assert_eq!(manager.len(), 1);
}

#[test]
fn window_manager_can_remove_window() {
    let mut manager = WindowManager::new();

    let window = WindowBuilder::new()
        .build()
        .expect("window should be created");

    let id = window.id();

    manager.add(window).expect("window should be added");
    manager
        .remove(id)
        .expect("window should be removed");

    assert!(manager.get(id).is_none());
    assert_eq!(manager.len(), 0);
}

#[test]
fn window_manager_can_focus_window() {
    let mut manager = WindowManager::new();

    let window = WindowBuilder::new()
        .build()
        .expect("window should be created");

    let id = window.id();

    manager.add(window).expect("window should be added");
    manager
        .focus(id)
        .expect("window should be focused");

    assert_eq!(manager.focused_window(), Some(id));
}

#[test]
fn only_one_window_is_focused() {
    let mut manager = WindowManager::new();

    let first = WindowBuilder::new()
        .build()
        .expect("first window should be created");

    let second = WindowBuilder::new()
        .build()
        .expect("second window should be created");

    let first_id = first.id();
    let second_id = second.id();

    manager.add(first).expect("first should be added");
    manager.add(second).expect("second should be added");

    manager
        .focus(first_id)
        .expect("first should be focused");

    manager
        .focus(second_id)
        .expect("second should be focused");

    assert_eq!(manager.focused_window(), Some(second_id));
    assert!(!manager.get(first_id).unwrap().is_focused());
    assert!(manager.get(second_id).unwrap().is_focused());
}
