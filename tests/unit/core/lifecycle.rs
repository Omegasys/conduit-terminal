use conduit::core::lifecycle::{Lifecycle, LifecycleState};

#[test]
fn lifecycle_starts_new() {
    let lifecycle = Lifecycle::new();

    assert_eq!(lifecycle.state(), LifecycleState::Created);
}

#[test]
fn lifecycle_can_initialize() {
    let mut lifecycle = Lifecycle::new();

    lifecycle.initialize().expect("initialization should succeed");

    assert_eq!(lifecycle.state(), LifecycleState::Initialized);
}

#[test]
fn lifecycle_can_start_after_initialization() {
    let mut lifecycle = Lifecycle::new();

    lifecycle.initialize().expect("initialization should succeed");
    lifecycle.start().expect("start should succeed");

    assert_eq!(lifecycle.state(), LifecycleState::Running);
}

#[test]
fn lifecycle_can_stop() {
    let mut lifecycle = Lifecycle::new();

    lifecycle.initialize().expect("initialization should succeed");
    lifecycle.start().expect("start should succeed");
    lifecycle.stop().expect("stop should succeed");

    assert_eq!(lifecycle.state(), LifecycleState::Stopped);
}

#[test]
fn lifecycle_rejects_invalid_start_transition() {
    let mut lifecycle = Lifecycle::new();

    let result = lifecycle.start();

    assert!(result.is_err());
}
