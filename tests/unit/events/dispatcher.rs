use conduit::events::{
    Event,
    EventDispatcher,
    EventKind,
};

#[test]
fn dispatcher_can_be_created() {
    let dispatcher = EventDispatcher::new();

    assert_eq!(dispatcher.handler_count(), 0);
}

#[test]
fn dispatcher_can_register_handler() {
    let mut dispatcher = EventDispatcher::new();

    dispatcher
        .register(EventKind::TabCreated, |_| {})
        .expect("handler should register");

    assert_eq!(dispatcher.handler_count(), 1);
}

#[test]
fn dispatcher_dispatches_matching_events() {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    let mut dispatcher = EventDispatcher::new();
    let called = Arc::new(AtomicBool::new(false));

    let called_clone = Arc::clone(&called);

    dispatcher
        .register(EventKind::TabCreated, move |_| {
            called_clone.store(true, Ordering::SeqCst);
        })
        .expect("handler should register");

    dispatcher
        .dispatch(Event::new(EventKind::TabCreated))
        .expect("dispatch should succeed");

    assert!(called.load(Ordering::SeqCst));
}

#[test]
fn dispatcher_ignores_unregistered_event_kinds() {
    let mut dispatcher = EventDispatcher::new();

    dispatcher
        .register(EventKind::TabCreated, |_| {})
        .expect("handler should register");

    assert!(
        dispatcher
            .dispatch(Event::new(EventKind::PaneCreated))
            .is_ok()
    );
}
