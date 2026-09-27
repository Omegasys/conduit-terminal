use conduit::panes::create::PaneBuilder;

#[test]
fn pane_builder_creates_pane() {
    let pane = PaneBuilder::new()
        .build()
        .expect("pane should be created");

    assert!(!pane.id().to_string().is_empty());
}

#[test]
fn pane_builder_sets_title() {
    let pane = PaneBuilder::new()
        .title("Shell")
        .build()
        .expect("pane should be created");

    assert_eq!(pane.title(), "Shell");
}

#[test]
fn pane_builder_sets_working_directory() {
    let pane = PaneBuilder::new()
        .working_directory("/tmp")
        .build()
        .expect("pane should be created");

    assert_eq!(pane.working_directory(), Some("/tmp"));
}

#[test]
fn pane_builder_sets_shell() {
    let pane = PaneBuilder::new()
        .shell("bash")
        .build()
        .expect("pane should be created");

    assert_eq!(pane.shell(), Some("bash"));
}
