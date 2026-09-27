use conduit::events::{Event, EventFilter, EventKind};

#[test]
fn event_filter_can_match_event_kind() {
    let filter = EventFilter::kind(EventKind::TabCreated);

    let matching = Event::new(EventKind::TabCreated);
    let non_matching = Event::new(EventKind::PaneCreated);

    assert!(filter.matches(&matching));
    assert!(!filter.matches(&non_matching));
}

#[test]
fn event_filter_can_match_multiple_kinds() {
    let filter = EventFilter::any_of([
        EventKind::TabCreated,
        EventKind::TabClosed,
    ]);

    assert!(filter.matches(&Event::new(EventKind::TabCreated)));
    assert!(filter.matches(&Event::new(EventKind::TabClosed)));
    assert!(!filter.matches(&Event::new(EventKind::PaneCreated)));
}

#[test]
fn empty_filter_matches_all_events() {
    let filter = EventFilter::all();

    assert!(filter.matches(&Event::new(EventKind::TabCreated)));
    assert!(filter.matches(&Event::new(EventKind::PaneCreated)));
}
