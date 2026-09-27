use conduit::config_engine::diff::ConfigDiff;

#[test]
fn identical_configurations_have_no_changes() {
    let diff = ConfigDiff::between(
        r#"scrollback = 10000"#,
        r#"scrollback = 10000"#,
    )
    .expect("diff should succeed");

    assert!(diff.is_empty());
}

#[test]
fn changed_configuration_is_detected() {
    let diff = ConfigDiff::between(
        r#"scrollback = 10000"#,
        r#"scrollback = 50000"#,
    )
    .expect("diff should succeed");

    assert!(!diff.is_empty());
}

#[test]
fn added_configuration_is_detected() {
    let diff = ConfigDiff::between(
        r#"scrollback = 10000"#,
        r#"
        scrollback = 10000
        shell = "bash"
        "#,
    )
    .expect("diff should succeed");

    assert!(!diff.is_empty());
}
