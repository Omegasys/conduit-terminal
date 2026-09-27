use conduit::core::error::CoreError;

#[test]
fn core_error_has_display_text() {
    let error = CoreError::InvalidState("test state".to_string());

    let message = error.to_string();

    assert!(!message.is_empty());
    assert!(message.contains("test state"));
}

#[test]
fn core_error_can_represent_missing_resource() {
    let error = CoreError::ResourceNotFound("example".to_string());

    assert!(error.to_string().contains("example"));
}

#[test]
fn core_errors_are_debuggable() {
    let error = CoreError::InvalidState("debug".to_string());

    let debug = format!("{error:?}");

    assert!(!debug.is_empty());
}
