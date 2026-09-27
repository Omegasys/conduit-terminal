use conduit::windows::create::WindowBuilder;

#[test]
fn builder_can_create_window() {
    let window = WindowBuilder::new()
        .build()
        .expect("window should be created");

    assert!(!window.id().to_string().is_empty());
}

#[test]
fn builder_can_set_title() {
    let window = WindowBuilder::new()
        .title("Conduit")
        .build()
        .expect("window should be created");

    assert_eq!(window.title(), "Conduit");
}

#[test]
fn builder_can_set_initial_size() {
    let window = WindowBuilder::new()
        .size(1280, 720)
        .build()
        .expect("window should be created");

    assert_eq!(window.width(), 1280);
    assert_eq!(window.height(), 720);
}

#[test]
fn builder_rejects_zero_dimensions() {
    let result = WindowBuilder::new()
        .size(0, 720)
        .build();

    assert!(result.is_err());
}
