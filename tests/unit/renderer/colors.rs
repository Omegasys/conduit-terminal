use conduit::renderer::colors::Color;

#[test]
fn rgb_color_can_be_created() {
    let color = Color::rgb(255, 128, 64);

    assert_eq!(color.red(), 255);
    assert_eq!(color.green(), 128);
    assert_eq!(color.blue(), 64);
}

#[test]
fn color_can_be_created_from_hex() {
    let color = Color::from_hex("#ff8040")
        .expect("valid hexadecimal color should parse");

    assert_eq!(color.red(), 255);
    assert_eq!(color.green(), 128);
    assert_eq!(color.blue(), 64);
}

#[test]
fn invalid_hex_color_is_rejected() {
    assert!(
        Color::from_hex("not-a-color").is_err()
    );
}

#[test]
fn transparent_color_is_supported() {
    let color = Color::rgba(255, 255, 255, 0);

    assert_eq!(color.alpha(), 0);
}
