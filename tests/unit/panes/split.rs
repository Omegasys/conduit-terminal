use conduit::panes::split::{SplitDirection, SplitRatio};

#[test]
fn split_direction_has_horizontal_and_vertical() {
    assert_ne!(
        SplitDirection::Horizontal,
        SplitDirection::Vertical
    );
}

#[test]
fn split_ratio_accepts_valid_ratio() {
    let ratio = SplitRatio::new(0.5)
        .expect("0.5 should be a valid split ratio");

    assert_eq!(ratio.value(), 0.5);
}

#[test]
fn split_ratio_rejects_zero() {
    assert!(
        SplitRatio::new(0.0).is_err()
    );
}

#[test]
fn split_ratio_rejects_one() {
    assert!(
        SplitRatio::new(1.0).is_err()
    );
}

#[test]
fn split_ratio_supports_adjustment() {
    let mut ratio = SplitRatio::new(0.5)
        .expect("ratio should be valid");

    ratio.set(0.25)
        .expect("new ratio should be valid");

    assert_eq!(ratio.value(), 0.25);
}
