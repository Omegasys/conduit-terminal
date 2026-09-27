use conduit::windows::state::WindowState;

#[test]
fn window_state_starts_normal() {
    assert_eq!(WindowState::default(), WindowState::Normal);
}

#[test]
fn window_state_supports_minimized() {
    assert_ne!(WindowState::Minimized, WindowState::Normal);
}

#[test]
fn window_state_supports_maximized() {
    assert_ne!(WindowState::Maximized, WindowState::Normal);
}

#[test]
fn window_state_supports_fullscreen() {
    assert_ne!(WindowState::Fullscreen, WindowState::Normal);
}
