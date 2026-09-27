use conduit::config_engine::defaults::default_configuration;

#[test]
fn default_configuration_exists() {
    let config = default_configuration();

    assert!(!config.is_empty());
}

#[test]
fn default_configuration_has_terminal_settings() {
    let config = default_configuration();

    assert!(config.contains("terminal"));
}

#[test]
fn default_configuration_has_security_settings() {
    let config = default_configuration();

    assert!(config.contains("security"));
}

#[test]
fn default_configuration_has_ui_settings() {
    let config = default_configuration();

    assert!(config.contains("ui"));
}
