use conduit::resources::theme::{Theme, ThemeColors};

#[test]
fn theme_can_be_created() {
    let theme = Theme::new("test");

    assert_eq!(theme.name(), "test");
}

#[test]
fn theme_has_default_colors() {
    let theme = Theme::new("test");

    let colors = theme.colors();

    assert!(colors.background.is_some());
    assert!(colors.foreground.is_some());
}

#[test]
fn theme_colors_can_be_changed() {
    let mut theme = Theme::new("test");

    let colors = ThemeColors {
        background: Some("#000000".to_string()),
        foreground: Some("#ffffff".to_string()),
        ..Default::default()
    };

    theme.set_colors(colors.clone());

    assert_eq!(theme.colors(), &colors);
}
