use conduit::events::{Event, EventBus, EventKind};

#[test]
fn event_bus_can_be_created() {
    let bus = EventBus::new();

    assert_eq!(bus.subscriber_count(), 0);
}

#[test]
fn event_bus_can_publish_event() {
    let bus = EventBus::new();

    let event = Event::new(EventKind::ApplicationStarted);

    assert!(bus.publish(event).is_ok());
}

#[test]
fn event_bus_can_subscribe() {
    let bus = EventBus::new();

    let subscription = bus.subscribe(|_| {});

    assert!(subscription.is_ok());
    assert_eq!(bus.subscriber_count(), 1);
}

#[test]
fn event_bus_can_remove_subscription() {
    let bus = EventBus::new();

    let subscription = bus
        .subscribe(|_| {})
        .expect("subscription should succeed");

    assert_eq!(bus.subscriber_count(), 1);

    bus.unsubscribe(subscription)
        .expect("unsubscribe should succeed");

    assert_eq!(bus.subscriber_count(), 0);
}
