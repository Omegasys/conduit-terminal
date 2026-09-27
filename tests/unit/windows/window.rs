use conduit::windows::window::{Window, WindowId};

#[test]
fn window_has_unique_id() {
    let first = Window::new();
    let second = Window::new();

    assert_ne!(first.id(), second.id());
}

#[test]
fn window_starts_unfocused() {
    let window = Window::new();

    assert!(!window.is_focused());
}

#[test]
fn window_can_gain_focus() {
    let mut window = Window::new();

    window.focus();

    assert!(window.is_focused());
}

#[test]
fn window_can_lose_focus() {
    let mut window = Window::new();

    window.focus();
    window.unfocus();

    assert!(!window.is_focused());
}

#[test]
fn window_id_can_be_retrieved() {
    let window = Window::new();
    let id: WindowId = window.id();

    assert_eq!(id, window.id());
}
