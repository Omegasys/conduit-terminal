#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterSet {
    Ascii,
    DecSpecialGraphics,
    Uk,
    Dutch,
    Finnish,
    French,
    FrenchCanadian,
    German,
    Italian,
    Swiss,
    Swedish,
    NorwegianDanish,
    Spanish,
    Japanese,
    Custom(u8),
}

impl CharacterSet {
    pub fn from_designator(value: u8) -> Self {
        match value {
            b'B' => Self::Ascii,
            b'0' => Self::DecSpecialGraphics,
            b'A' => Self::Uk,
            b'4' => Self::Dutch,
            b'C' | b'5' => Self::Finnish,
            b'R' => Self::French,
            b'Q' => Self::FrenchCanadian,
            b'K' => Self::German,
            b'Y' => Self::Italian,
            b'=' => Self::Swiss,
            b'H' => Self::Swedish,
            b'E' | b'6' => Self::NorwegianDanish,
            b'Z' => Self::Spanish,
            b'J' => Self::Japanese,
            other => Self::Custom(other),
        }
    }

    pub fn map_byte(self, byte: u8) -> char {
        if self == Self::DecSpecialGraphics {
            return match byte {
                b'j' => '┘',
                b'k' => '┐',
                b'l' => '┌',
                b'm' => '└',
                b'n' => '┼',
                b'q' => '─',
                b'x' => '│',
                b't' => '├',
                b'u' => '┤',
                b'v' => '┴',
                b'w' => '┬',
                b'~' => '·',
                _ => byte as char,
            };
        }

        byte as char
    }
}

#[derive(Debug, Clone)]
pub struct CharacterSetState {
    g0: CharacterSet,
    g1: CharacterSet,
    g2: CharacterSet,
    g3: CharacterSet,
    active: u8,
}

impl Default for CharacterSetState {
    fn default() -> Self {
        Self {
            g0: CharacterSet::Ascii,
            g1: CharacterSet::Ascii,
            g2: CharacterSet::Ascii,
            g3: CharacterSet::Ascii,
            active: 0,
        }
    }
}

impl CharacterSetState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn designate(&mut self, slot: u8, set: CharacterSet) {
        match slot {
            0 => self.g0 = set,
            1 => self.g1 = set,
            2 => self.g2 = set,
            3 => self.g3 = set,
            _ => {}
        }
    }

    pub fn shift_in(&mut self) {
        self.active = 0;
    }

    pub fn shift_out(&mut self) {
        self.active = 1;
    }

    pub fn select(&mut self, slot: u8) {
        if slot <= 3 {
            self.active = slot;
        }
    }

    pub fn active_set(&self) -> CharacterSet {
        match self.active {
            0 => self.g0,
            1 => self.g1,
            2 => self.g2,
            3 => self.g3,
            _ => CharacterSet::Ascii,
        }
    }

    pub fn map(&self, byte: u8) -> char {
        self.active_set().map_byte(byte)
    }
}
