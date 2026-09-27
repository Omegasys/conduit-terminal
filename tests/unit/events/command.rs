use conduit::events::{
    CommandEvent,
    Event,
};

#[test]
fn command_event_can_be_created() {
    let command = CommandEvent::new("ls -la");

    assert_eq!(command.command(), "ls -la");
}

#[test]
fn command_event_can_be_converted_to_event() {
    let command = CommandEvent::new("pwd");
    let event: Event = command.into();

    assert!(event.is_command());
}

#[test]
fn command_event_preserves_command_text() {
    let command = CommandEvent::new("cargo test");

    assert_eq!(command.command(), "cargo test");
}
