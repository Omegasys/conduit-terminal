use conduit::events::{Event, EventBus, EventKind};

#[test]
fn event_bus_starts_without_subscribers() {
    let bus = EventBus::new();

    assert_eq!(bus.subscriber_count(), 0);
}

#[test]
fn event_bus_accepts_subscribers() {
    let bus = EventBus::new();

    bus.subscribe(|_| {})
        .expect("subscription should succeed");

    assert_eq!(bus.subscriber_count(), 1);
}

#[test]
fn event_bus_delivers_events() {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    let bus = EventBus::new();
    let received = Arc::new(AtomicBool::new(false));

    let received_clone = Arc::clone(&received);

    bus.subscribe(move |event| {
        if event.kind() == &EventKind::ApplicationStarted {
            received_clone.store(true, Ordering::SeqCst);
        }
    })
    .expect("subscription should succeed");

    bus.publish(Event::new(EventKind::ApplicationStarted))
        .expect("event should publish");

    assert!(received.load(Ordering::SeqCst));
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
