use conduit::tabs::state::TabState;

#[test]
fn tab_state_starts_active() {
    assert_eq!(TabState::default(), TabState::Active);
}

#[test]
fn tab_state_supports_active_state() {
    assert_eq!(TabState::Active, TabState::Active);
}

#[test]
fn tab_state_supports_background_state() {
    assert_ne!(TabState::Background, TabState::Active);
}

#[test]
fn tab_state_supports_closing_state() {
    assert_ne!(TabState::Closing, TabState::Active);
}
