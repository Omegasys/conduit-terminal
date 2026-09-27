use conduit::renderer::glyphs::Glyph;

#[test]
fn glyph_can_be_created() {
    let glyph = Glyph::new('A');

    assert_eq!(glyph.character(), 'A');
}

#[test]
fn glyph_can_store_width() {
    let glyph = Glyph::new('A');

    assert!(glyph.width() >= 1);
}

#[test]
fn unicode_glyph_is_supported() {
    let glyph = Glyph::new('λ');

    assert_eq!(glyph.character(), 'λ');
}

#[test]
fn emoji_glyph_is_supported() {
    let glyph = Glyph::new('🚀');

    assert_eq!(glyph.character(), '🚀');
}
