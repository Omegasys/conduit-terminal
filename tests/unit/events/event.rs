use conduit::events::{Event, EventKind};

#[test]
fn event_can_be_created() {
    let event = Event::new(EventKind::ApplicationStarted);

    assert_eq!(event.kind(), &EventKind::ApplicationStarted);
}

#[test]
fn events_have_unique_ids() {
    let first = Event::new(EventKind::ApplicationStarted);
    let second = Event::new(EventKind::ApplicationStarted);

    assert_ne!(first.id(), second.id());
}

#[test]
fn event_timestamp_is_available() {
    let event = Event::new(EventKind::ApplicationStarted);

    assert!(event.timestamp().elapsed().is_ok());
}

#[test]
fn event_kind_is_preserved() {
    let event = Event::new(EventKind::ConfigurationChanged);

    assert_eq!(event.kind(), &EventKind::ConfigurationChanged);
}
