use conduit::resources::manifest::ResourceManifest;

#[test]
fn manifest_can_be_created() {
    let manifest = ResourceManifest::new(
        "cyberpunk",
        "theme",
        "1.0.0",
    );

    assert_eq!(manifest.name(), "cyberpunk");
    assert_eq!(manifest.resource_type(), "theme");
    assert_eq!(manifest.version(), "1.0.0");
}

#[test]
fn manifest_can_validate_required_fields() {
    let manifest = ResourceManifest::new(
        "cyberpunk",
        "theme",
        "1.0.0",
    );

    assert!(manifest.validate().is_ok());
}

#[test]
fn manifest_rejects_empty_name() {
    let manifest = ResourceManifest::new(
        "",
        "theme",
        "1.0.0",
    );

    assert!(manifest.validate().is_err());
}
