use conduit::config_engine::loader::ConfigLoader;
use std::fs;

#[test]
fn loader_can_load_configuration_file() {
    let directory = tempfile::tempdir().expect("temporary directory should exist");
    let path = directory.path().join("config.toml");

    fs::write(
        &path,
        r#"
        shell = "bash"
        scrollback = 10000
        "#,
    )
    .expect("configuration should be written");

    let loader = ConfigLoader::new();
    let config = loader.load(&path).expect("configuration should load");

    assert_eq!(config.get("shell"), Some("bash"));
}

#[test]
fn loader_reports_missing_file() {
    let loader = ConfigLoader::new();

    let result = loader.load(std::path::Path::new("/definitely/missing/config.toml"));

    assert!(result.is_err());
}

#[test]
fn loader_can_reload_configuration() {
    let directory = tempfile::tempdir().expect("temporary directory should exist");
    let path = directory.path().join("config.toml");

    fs::write(&path, r#"shell = "bash""#).expect("write should succeed");

    let loader = ConfigLoader::new();

    let first = loader.load(&path).expect("first load should succeed");
    assert_eq!(first.get("shell"), Some("bash"));

    fs::write(&path, r#"shell = "zsh""#).expect("write should succeed");

    let second = loader.load(&path).expect("second load should succeed");
    assert_eq!(second.get("shell"), Some("zsh"));
}
