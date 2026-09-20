use super::rgba::Rgba;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorFormat {
    Rgb,
    Rgba,
    Ansi,
    Indexed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Rgba(Rgba),
    Ansi(u8),
    Indexed(u8),
}

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::Rgba(Rgba::rgb(red, green, blue))
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::Rgba(Rgba::new(red, green, blue, alpha))
    }

    pub fn ansi(index: u8) -> Self {
        Self::Ansi(index.min(15))
    }

    pub fn indexed(index: u8) -> Self {
        Self::Indexed(index)
    }

    pub fn format(&self) -> ColorFormat {
        match self {
            Self::Rgba(rgba) => {
                if rgba.is_opaque() {
                    ColorFormat::Rgb
                } else {
                    ColorFormat::Rgba
                }
            }
            Self::Ansi(_) => ColorFormat::Ansi,
            Self::Indexed(_) => ColorFormat::Indexed,
        }
    }

    pub fn rgba(&self) -> Option<Rgba> {
        match self {
            Self::Rgba(value) => Some(*value),
            _ => None,
        }
    }

    pub fn ansi_index(&self) -> Option<u8> {
        match self {
            Self::Ansi(value) => Some(*value),
            _ => None,
        }
    }

    pub fn indexed_value(&self) -> Option<u8> {
        match self {
            Self::Indexed(value) => Some(*value),
            _ => None,
        }
    }

    pub fn is_transparent(&self) -> bool {
        matches!(self, Self::Rgba(value) if value.is_transparent())
    }

    pub fn to_hex(&self) -> Option<String> {
        self.rgba().map(|value| {
            if value.is_opaque() {
                value.to_hex_rgb()
            } else {
                value.to_hex_rgba()
            }
        })
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Rgba(Rgba::BLACK)
    }
}
