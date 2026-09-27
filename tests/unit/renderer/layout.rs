use conduit::renderer::layout::RenderLayout;

#[test]
fn layout_can_be_created() {
    let layout = RenderLayout::new(80, 24);

    assert_eq!(layout.columns(), 80);
    assert_eq!(layout.rows(), 24);
}

#[test]
fn layout_calculates_cell_count() {
    let layout = RenderLayout::new(80, 24);

    assert_eq!(layout.cell_count(), 80 * 24);
}

#[test]
fn layout_can_resize() {
    let mut layout = RenderLayout::new(80, 24);

    layout.resize(120, 40);

    assert_eq!(layout.columns(), 120);
    assert_eq!(layout.rows(), 40);
}

#[test]
fn layout_rejects_zero_dimensions() {
    assert!(
        RenderLayout::new_checked(0, 24).is_err()
    );

    assert!(
        RenderLayout::new_checked(80, 0).is_err()
    );
}
