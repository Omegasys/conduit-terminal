use conduit::renderer::cells::{Cell, CellAttributes};

#[test]
fn empty_cell_is_created() {
    let cell = Cell::default();

    assert_eq!(cell.character(), ' ');
}

#[test]
fn cell_can_store_character() {
    let mut cell = Cell::default();

    cell.set_character('A');

    assert_eq!(cell.character(), 'A');
}

#[test]
fn cell_can_store_attributes() {
    let mut cell = Cell::default();

    let attributes = CellAttributes::default();

    cell.set_attributes(attributes.clone());

    assert_eq!(cell.attributes(), &attributes);
}

#[test]
fn cell_can_be_reset() {
    let mut cell = Cell::default();

    cell.set_character('X');
    cell.reset();

    assert_eq!(cell.character(), ' ');
}
