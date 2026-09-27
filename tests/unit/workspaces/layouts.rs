use conduit::workspaces::layouts::{
    Layout,
    LayoutKind,
};

#[test]
fn default_layout_is_valid() {
    let layout = Layout::default();

    assert_eq!(layout.kind(), LayoutKind::Single);
}

#[test]
fn split_layout_is_supported() {
    let layout = Layout::new(LayoutKind::Split);

    assert_eq!(layout.kind(), LayoutKind::Split);
}

#[test]
fn grid_layout_is_supported() {
    let layout = Layout::new(LayoutKind::Grid);

    assert_eq!(layout.kind(), LayoutKind::Grid);
}

#[test]
fn layout_can_store_pane_count() {
    let mut layout = Layout::new(LayoutKind::Grid);

    layout.set_pane_count(4);

    assert_eq!(layout.pane_count(), 4);
}
