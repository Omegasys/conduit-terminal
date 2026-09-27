use conduit::panes::pane::Pane;

#[test]
fn pane_has_unique_id() {
    let first = Pane::new();
    let second = Pane::new();

    assert_ne!(first.id(), second.id());
}

#[test]
fn pane_starts_active() {
    let pane = Pane::new();

    assert!(pane.is_active());
}

#[test]
fn pane_can_be_activated() {
    let mut pane = Pane::new();

    pane.deactivate();
    pane.activate();

    assert!(pane.is_active());
}

#[test]
fn pane_can_be_deactivated() {
    let mut pane = Pane::new();

    pane.deactivate();

    assert!(!pane.is_active());
}

#[test]
fn pane_can_store_title() {
    let mut pane = Pane::new();

    pane.set_title("Terminal");

    assert_eq!(pane.title(), "Terminal");
}
