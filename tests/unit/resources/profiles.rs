use conduit::resources::profile::Profile;

#[test]
fn profile_can_be_created() {
    let profile = Profile::new("default");

    assert_eq!(profile.name(), "default");
}

#[test]
fn profile_has_terminal_configuration() {
    let profile = Profile::new("development");

    assert!(profile.terminal().is_some());
}

#[test]
fn profile_can_set_theme() {
    let mut profile = Profile::new("development");

    profile.set_theme("cyberpunk");

    assert_eq!(profile.theme(), Some("cyberpunk"));
}

#[test]
fn profile_can_set_scrollback() {
    let mut profile = Profile::new("development");

    profile.set_scrollback(50_000);

    assert_eq!(profile.scrollback(), Some(50_000));
}
