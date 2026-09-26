use crate::terminal_protocols::core::charset::CharacterSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecCharacterSet {
    Ascii,
    DecSpecial,
    UserDefined(u8),
}

impl DecCharacterSet {
    pub fn character_set(self) -> CharacterSet {
        match self {
            Self::Ascii => CharacterSet::Ascii,
            Self::DecSpecial => CharacterSet::DecSpecialGraphics,
            Self::UserDefined(value) => CharacterSet::Custom(value),
        }
    }
}

#[derive(Debug, Default)]
pub struct DecCharacterSetRegistry {
    sets: Vec<DecCharacterSet>,
}

impl DecCharacterSetRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, set: DecCharacterSet) {
        self.sets.push(set);
    }

    pub fn all(&self) -> &[DecCharacterSet] {
        &self.sets
    }

    pub fn clear(&mut self) {
        self.sets.clear();
    }
}
