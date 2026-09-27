use conduit::config_engine::validator::ConfigValidator;

#[test]
fn validator_accepts_valid_configuration() {
    let validator = ConfigValidator::new();

    let config = r#"
        [terminal]
        scrollback = 10000
    "#;

    assert!(validator.validate(config).is_ok());
}

#[test]
fn validator_rejects_invalid_configuration() {
    let validator = ConfigValidator::new();

    let config = r#"
        [terminal
        scrollback = 10000
    "#;

    assert!(validator.validate(config).is_err());
}

#[test]
fn validator_detects_invalid_values() {
    let validator = ConfigValidator::new();

    let config = r#"
        [terminal]
        scrollback = -100
    "#;

    assert!(validator.validate(config).is_err());
}
