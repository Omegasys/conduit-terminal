use super::rgba::Rgba;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnsiColorDepth {
    Basic8,
    Standard16,
    Indexed256,
    TrueColor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AnsiColor {
    index: u8,
}

impl AnsiColor {
    pub fn new(index: u8) -> Self {
        Self {
            index: index.min(255),
        }
    }

    pub fn index(&self) -> u8 {
        self.index
    }

    pub fn to_rgba(&self) -> Rgba {
        match self.index {
            0 => Rgba::rgb(0, 0, 0),
            1 => Rgba::rgb(128, 0, 0),
            2 => Rgba::rgb(0, 128, 0),
            3 => Rgba::rgb(128, 128, 0),
            4 => Rgba::rgb(0, 0, 128),
            5 => Rgba::rgb(128, 0, 128),
            6 => Rgba::rgb(0, 128, 128),
            7 => Rgba::rgb(192, 192, 192),
            8 => Rgba::rgb(128, 128, 128),
            9 => Rgba::rgb(255, 0, 0),
            10 => Rgba::rgb(0, 255, 0),
            11 => Rgba::rgb(255, 255, 0),
            12 => Rgba::rgb(0, 0, 255),
            13 => Rgba::rgb(255, 0, 255),
            14 => Rgba::rgb(0, 255, 255),
            15 => Rgba::rgb(255, 255, 255),
            16..=231 => Self::cube_color(self.index),
            232..=255 => Self::gray_color(self.index),
        }
    }

    fn cube_color(index: u8) -> Rgba {
        let index = index - 16;

        let r = index / 36;
        let g = (index % 36) / 6;
        let b = index % 6;

        let convert = |value: u8| -> u8 {
            if value == 0 {
                0
            } else {
                55 + value * 40
            }
        };

        Rgba::rgb(convert(r), convert(g), convert(b))
    }

    fn gray_color(index: u8) -> Rgba {
        let value = 8 + (index - 232) * 10;
        Rgba::rgb(value, value, value)
    }

    pub fn depth_supports(depth: AnsiColorDepth, required: AnsiColorDepth) -> bool {
        let rank = |value| match value {
            AnsiColorDepth::Basic8 => 0,
            AnsiColorDepth::Standard16 => 1,
            AnsiColorDepth::Indexed256 => 2,
            AnsiColorDepth::TrueColor => 3,
        };

        rank(depth) >= rank(required)
    }
}
