use conduit::windows::geometry::{
    Position,
    Size,
    Rect,
};

#[test]
fn position_stores_coordinates() {
    let position = Position::new(100, 200);

    assert_eq!(position.x(), 100);
    assert_eq!(position.y(), 200);
}

#[test]
fn size_stores_dimensions() {
    let size = Size::new(1280, 720);

    assert_eq!(size.width(), 1280);
    assert_eq!(size.height(), 720);
}

#[test]
fn rectangle_contains_point() {
    let rect = Rect::new(
        Position::new(0, 0),
        Size::new(100, 100),
    );

    assert!(rect.contains(Position::new(50, 50)));
    assert!(!rect.contains(Position::new(200, 200)));
}

#[test]
fn rectangle_can_be_resized() {
    let mut rect = Rect::new(
        Position::new(0, 0),
        Size::new(100, 100),
    );

    rect.set_size(Size::new(500, 300));

    assert_eq!(rect.width(), 500);
    assert_eq!(rect.height(), 300);
}
