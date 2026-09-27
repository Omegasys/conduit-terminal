use conduit::events::EventPriority;

#[test]
fn event_priorities_have_expected_order() {
    assert!(EventPriority::Critical > EventPriority::High);
    assert!(EventPriority::High > EventPriority::Normal);
    assert!(EventPriority::Normal > EventPriority::Low);
}

#[test]
fn normal_is_default_priority() {
    assert_eq!(EventPriority::default(), EventPriority::Normal);
}
