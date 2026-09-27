use conduit::tabs::title::{
    TabTitle,
    TabTitleSource,
};

#[test]
fn static_title_is_preserved() {
    let title = TabTitle::new(
        "Development",
        TabTitleSource::Static,
    );

    assert_eq!(title.text(), "Development");
    assert_eq!(title.source(), TabTitleSource::Static);
}

#[test]
fn shell_title_is_supported() {
    let title = TabTitle::new(
        "bash",
        TabTitleSource::Shell,
    );

    assert_eq!(title.text(), "bash");
    assert_eq!(title.source(), TabTitleSource::Shell);
}

#[test]
fn directory_title_is_supported() {
    let title = TabTitle::new(
        "~/Projects",
        TabTitleSource::Directory,
    );

    assert_eq!(title.text(), "~/Projects");
    assert_eq!(title.source(), TabTitleSource::Directory);
}

#[test]
fn title_can_be_updated() {
    let mut title = TabTitle::new(
        "Old",
        TabTitleSource::Static,
    );

    title.set_text("New");

    assert_eq!(title.text(), "New");
}
