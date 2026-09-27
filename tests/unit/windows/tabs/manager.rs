use conduit::tabs::{
    TabBuilder,
    TabManager,
};

#[test]
fn tab_manager_starts_empty() {
    let manager = TabManager::new();

    assert_eq!(manager.len(), 0);
}

#[test]
fn tab_manager_can_add_tab() {
    let mut manager = TabManager::new();

    let tab = TabBuilder::new()
        .title("Terminal")
        .build()
        .expect("tab should be created");

    let id = tab.id();

    manager
        .add(tab)
        .expect("tab should be added");

    assert!(manager.get(id).is_some());
    assert_eq!(manager.len(), 1);
}

#[test]
fn tab_manager_can_remove_tab() {
    let mut manager = TabManager::new();

    let tab = TabBuilder::new()
        .build()
        .expect("tab should be created");

    let id = tab.id();

    manager.add(tab).expect("tab should be added");

    manager
        .remove(id)
        .expect("tab should be removed");

    assert!(manager.get(id).is_none());
    assert_eq!(manager.len(), 0);
}

#[test]
fn tab_manager_can_focus_tab() {
    let mut manager = TabManager::new();

    let tab = TabBuilder::new()
        .title("Focused")
        .build()
        .expect("tab should be created");

    let id = tab.id();

    manager.add(tab).expect("tab should be added");

    manager
        .focus(id)
        .expect("tab should be focused");

    assert_eq!(manager.focused_tab(), Some(id));
}

#[test]
fn only_one_tab_is_focused() {
    let mut manager = TabManager::new();

    let first = TabBuilder::new()
        .title("First")
        .build()
        .expect("first tab should be created");

    let second = TabBuilder::new()
        .title("Second")
        .build()
        .expect("second tab should be created");

    let first_id = first.id();
    let second_id = second.id();

    manager.add(first).expect("first tab should be added");
    manager.add(second).expect("second tab should be added");

    manager
        .focus(first_id)
        .expect("first tab should be focused");

    manager
        .focus(second_id)
        .expect("second tab should be focused");

    assert_eq!(manager.focused_tab(), Some(second_id));
    assert!(!manager.get(first_id).unwrap().is_focused());
    assert!(manager.get(second_id).unwrap().is_focused());
}

#[test]
fn tab_order_is_preserved() {
    let mut manager = TabManager::new();

    let first = TabBuilder::new()
        .title("First")
        .build()
        .expect("first tab should be created");

    let second = TabBuilder::new()
        .title("Second")
        .build()
        .expect("second tab should be created");

    let first_id = first.id();
    let second_id = second.id();

    manager.add(first).expect("first tab should be added");
    manager.add(second).expect("second tab should be added");

    let ids = manager.ids();

    assert_eq!(ids, vec![first_id, second_id]);
}
