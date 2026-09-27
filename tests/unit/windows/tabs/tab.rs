use conduit::tabs::tab::Tab;

#[test]
fn tab_has_unique_id() {
    let first = Tab::new();
    let second = Tab::new();

    assert_ne!(first.id(), second.id());
}

#[test]
fn tab_starts_with_default_title() {
    let tab = Tab::new();

    assert!(!tab.title().is_empty());
}

#[test]
fn tab_can_change_title() {
    let mut tab = Tab::new();

    tab.set_title("Development");

    assert_eq!(tab.title(), "Development");
}

#[test]
fn tab_starts_unfocused() {
    let tab = Tab::new();

    assert!(!tab.is_focused());
}

#[test]
fn tab_can_be_focused() {
    let mut tab = Tab::new();

    tab.focus();

    assert!(tab.is_focused());
}
