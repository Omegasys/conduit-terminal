use conduit::resources::validation::ResourceValidator;

#[test]
fn validator_accepts_valid_theme() {
    let validator = ResourceValidator::new();

    let resource = r#"
        name = "nord"
        type = "theme"
        version = "1.0.0"
    "#;

    assert!(validator.validate(resource).is_ok());
}

#[test]
fn validator_rejects_missing_name() {
    let validator = ResourceValidator::new();

    let resource = r#"
        type = "theme"
        version = "1.0.0"
    "#;

    assert!(validator.validate(resource).is_err());
}

#[test]
fn validator_rejects_unknown_resource_type() {
    let validator = ResourceValidator::new();

    let resource = r#"
        name = "test"
        type = "something_invalid"
        version = "1.0.0"
    "#;

    assert!(validator.validate(resource).is_err());
}
