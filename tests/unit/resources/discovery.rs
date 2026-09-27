use conduit::resources::discovery::ResourceDiscovery;
use std::fs;

#[test]
fn discovery_can_find_resource_files() {
    let directory = tempfile::tempdir().expect("temporary directory should exist");

    fs::write(
        directory.path().join("theme.toml"),
        r#"
        name = "test"
        type = "theme"
        "#,
    )
    .expect("resource should be written");

    let discovery = ResourceDiscovery::new();
    let resources = discovery
        .scan(directory.path())
        .expect("resource scan should succeed");

    assert_eq!(resources.len(), 1);
}

#[test]
fn discovery_ignores_unrelated_files() {
    let directory = tempfile::tempdir().expect("temporary directory should exist");

    fs::write(directory.path().join("notes.txt"), "not a resource")
        .expect("file should be written");

    let discovery = ResourceDiscovery::new();
    let resources = discovery
        .scan(directory.path())
        .expect("resource scan should succeed");

    assert!(resources.is_empty());
}
