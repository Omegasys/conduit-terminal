use conduit::tabs::create::TabBuilder;

#[test]
fn tab_builder_creates_tab() {
    let tab = TabBuilder::new()
        .build()
        .expect("tab should be created");

    assert!(!tab.id().to_string().is_empty());
}

#[test]
fn tab_builder_sets_title() {
    let tab = TabBuilder::new()
        .title("Terminal")
        .build()
        .expect("tab should be created");

    assert_eq!(tab.title(), "Terminal");
}

#[test]
fn tab_builder_can_set_shell() {
    let tab = TabBuilder::new()
        .shell("bash")
        .build()
        .expect("tab should be created");

    assert_eq!(tab.shell(), Some("bash"));
}

#[test]
fn tab_builder_can_set_working_directory() {
    let tab = TabBuilder::new()
        .working_directory("/tmp")
        .build()
        .expect("tab should be created");

    assert_eq!(tab.working_directory(), Some("/tmp"));
}
