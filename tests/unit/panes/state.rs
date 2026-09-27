use conduit::panes::state::PaneState;

#[test]
fn pane_state_defaults_to_active() {
    assert_eq!(PaneState::default(), PaneState::Active);
}

#[test]
fn pane_state_supports_active() {
    assert_eq!(PaneState::Active, PaneState::Active);
}

#[test]
fn pane_state_supports_inactive() {
    assert_ne!(PaneState::Inactive, PaneState::Active);
}

#[test]
fn pane_state_supports_closing() {
    assert_ne!(PaneState::Closing, PaneState::Active);
}
