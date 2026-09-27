use conduit::panes::geometry::{PaneRect, Position, Size};

#[test]
fn position_stores_coordinates() {
    let position = Position::new(100, 200);

    assert_eq!(position.x(), 100);
    assert_eq!(position.y(), 200);
}

#[test]
fn size_stores_dimensions() {
    let size = Size::new(800, 600);

    assert_eq!(size.width(), 800);
    assert_eq!(size.height(), 600);
}

#[test]
fn pane_rect_stores_position_and_size() {
    let rect = PaneRect::new(
        Position::new(10, 20),
        Size::new(800, 600),
    );

    assert_eq!(rect.x(), 10);
    assert_eq!(rect.y(), 20);
    assert_eq!(rect.width(), 800);
    assert_eq!(rect.height(), 600);
}

#[test]
fn pane_rect_contains_point() {
    let rect = PaneRect::new(
        Position::new(0, 0),
        Size::new(100, 100),
    );

    assert!(rect.contains(50, 50));
    assert!(!rect.contains(200, 200));
}

#[test]
fn pane_rect_can_be_resized() {
    let mut rect = PaneRect::new(
        Position::new(0, 0),
        Size::new(100, 100),
    );

    rect.resize(Size::new(500, 300));

    assert_eq!(rect.width(), 500);
    assert_eq!(rect.height(), 300);
}
