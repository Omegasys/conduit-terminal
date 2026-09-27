use conduit::resources::paths::ResourcePaths;

#[test]
fn resource_paths_can_be_created() {
    let paths = ResourcePaths::new();

    assert!(!paths.resource_root().as_os_str().is_empty());
}

#[test]
fn theme_path_is_under_resource_root() {
    let paths = ResourcePaths::new();

    let theme_root = paths.theme_root();

    assert!(theme_root.starts_with(paths.resource_root()));
}

#[test]
fn profile_path_is_under_resource_root() {
    let paths = ResourcePaths::new();

    let profile_root = paths.profile_root();

    assert!(profile_root.starts_with(paths.resource_root()));
}

#[test]
fn workspace_path_is_under_resource_root() {
    let paths = ResourcePaths::new();

    let workspace_root = paths.workspace_root();

    assert!(workspace_root.starts_with(paths.resource_root()));
}
