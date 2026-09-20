use super::{
    ansi::{AnsiColor, AnsiColorDepth},
    color::Color,
    rgba::Rgba,
};

pub struct ColorConversion;

impl ColorConversion {
    pub fn to_rgba(color: Color) -> Rgba {
        match color {
            Color::Rgba(value) => value,
            Color::Ansi(index) | Color::Indexed(index) => AnsiColor::new(index).to_rgba(),
        }
    }

    pub fn to_ansi(color: Rgba, depth: AnsiColorDepth) -> AnsiColor {
        match depth {
            AnsiColorDepth::Basic8 => AnsiColor::new(Self::nearest_ansi(color, 8)),
            AnsiColorDepth::Standard16 => AnsiColor::new(Self::nearest_ansi(color, 16)),
            AnsiColorDepth::Indexed256 => AnsiColor::new(Self::nearest_ansi(color, 256)),
            AnsiColorDepth::TrueColor => {
                AnsiColor::new(Self::nearest_ansi(color, 256))
            }
        }
    }

    fn nearest_ansi(color: Rgba, count: u16) -> u8 {
        let mut best_index = 0;
        let mut best_distance = u32::MAX;

        for index in 0..count {
            let candidate = AnsiColor::new(index as u8).to_rgba();

            let distance = Self::distance(color, candidate);

            if distance < best_distance {
                best_distance = distance;
                best_index = index as u8;
            }
        }

        best_index
    }

    fn distance(a: Rgba, b: Rgba) -> u32 {
        let red = a.red() as i32 - b.red() as i32;
        let green = a.green() as i32 - b.green() as i32;
        let blue = a.blue() as i32 - b.blue() as i32;

        (red * red + green * green + blue * blue) as u32
    }
}

pub type ColorConverter = ColorConversion;
