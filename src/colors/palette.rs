use std::collections::BTreeMap;

use super::color::Color;

#[derive(Clone, Debug)]
pub struct PaletteEntry {
    name: String,
    color: Color,
}

impl PaletteEntry {
    pub fn new(name: impl Into<String>, color: Color) -> Self {
        Self {
            name: name.into(),
            color,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

#[derive(Clone, Debug, Default)]
pub struct ColorPalette {
    entries: BTreeMap<String, PaletteEntry>,
}

impl ColorPalette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: impl Into<String>, color: Color) {
        let name = name.into();

        self.entries
            .insert(name.clone(), PaletteEntry::new(name, color));
    }

    pub fn get(&self, name: &str) -> Option<&PaletteEntry> {
        self.entries.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut PaletteEntry> {
        self.entries.get_mut(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<PaletteEntry> {
        self.entries.remove(name)
    }

    pub fn entries(&self) -> impl Iterator<Item = &PaletteEntry> {
        self.entries.values()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
