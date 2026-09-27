use conduit::renderer::Renderer;

#[test]
fn renderer_can_be_created() {
    let renderer = Renderer::new();

    assert!(!renderer.is_initialized());
}

#[test]
fn renderer_can_initialize() {
    let mut renderer = Renderer::new();

    renderer
        .initialize()
        .expect("renderer should initialize");

    assert!(renderer.is_initialized());
}

#[test]
fn renderer_can_be_shutdown() {
    let mut renderer = Renderer::new();

    renderer
        .initialize()
        .expect("renderer should initialize");

    renderer
        .shutdown()
        .expect("renderer should shut down");

    assert!(!renderer.is_initialized());
}
