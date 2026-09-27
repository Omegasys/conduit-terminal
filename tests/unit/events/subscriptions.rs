use conduit::events::{EventBus, EventKind};

#[test]
fn subscriptions_receive_matching_events() {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    let bus = EventBus::new();
    let count = Arc::new(AtomicUsize::new(0));

    let count_clone = Arc::clone(&count);

    bus.subscribe(move |event| {
        if event.kind() == &EventKind::TabCreated {
            count_clone.fetch_add(1, Ordering::SeqCst);
        }
    })
    .expect("subscription should succeed");

    bus.publish(EventKind::TabCreated.into())
        .expect("event should publish");

    bus.publish(EventKind::PaneCreated.into())
        .expect("event should publish");

    bus.publish(EventKind::TabCreated.into())
        .expect("event should publish");

    assert_eq!(count.load(Ordering::SeqCst), 2);
}

#[test]
fn multiple_subscribers_can_receive_same_event() {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    let bus = EventBus::new();
    let count = Arc::new(AtomicUsize::new(0));

    let first = Arc::clone(&count);
    let second = Arc::clone(&count);

    bus.subscribe(move |_| {
        first.fetch_add(1, Ordering::SeqCst);
    })
    .expect("first subscription should succeed");

    bus.subscribe(move |_| {
        second.fetch_add(1, Ordering::SeqCst);
    })
    .expect("second subscription should succeed");

    bus.publish(EventKind::TabCreated.into())
        .expect("event should publish");

    assert_eq!(count.load(Ordering::SeqCst), 2);
}
