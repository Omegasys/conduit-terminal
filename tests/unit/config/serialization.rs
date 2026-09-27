use conduit::config_engine::serialization::{
    deserialize_configuration,
    serialize_configuration,
};

#[test]
fn serialization_round_trip_preserves_configuration() {
    let original = r#"
        [terminal]
        scrollback = 10000
        cursor_style = "block"
    "#;

    let value = deserialize_configuration(original)
        .expect("deserialization should succeed");

    let serialized = serialize_configuration(&value)
        .expect("serialization should succeed");

    let restored = deserialize_configuration(&serialized)
        .expect("second deserialization should succeed");

    assert_eq!(value, restored);
}

#[test]
fn serialization_produces_non_empty_output() {
    let value = deserialize_configuration(r#"shell = "bash""#)
        .expect("deserialization should succeed");

    let serialized =
        serialize_configuration(&value).expect("serialization should succeed");

    assert!(!serialized.is_empty());
}
