use conduit::core::state::{ApplicationState, ApplicationStatus};

#[test]
fn application_state_starts_in_expected_state() {
    let state = ApplicationState::default();

    assert_eq!(state.status(), ApplicationStatus::Initializing);
}

#[test]
fn application_state_can_transition_to_running() {
    let mut state = ApplicationState::default();

    state.set_status(ApplicationStatus::Running);

    assert_eq!(state.status(), ApplicationStatus::Running);
}

#[test]
fn application_state_can_transition_to_stopping() {
    let mut state = ApplicationState::default();

    state.set_status(ApplicationStatus::Stopping);

    assert_eq!(state.status(), ApplicationStatus::Stopping);
}

#[test]
fn application_state_can_transition_to_stopped() {
    let mut state = ApplicationState::default();

    state.set_status(ApplicationStatus::Stopped);

    assert_eq!(state.status(), ApplicationStatus::Stopped);
}
