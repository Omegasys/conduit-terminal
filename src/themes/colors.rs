//! Theme color definitions.

/// RGBA color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);

    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn from_hex(value: &str) -> Option<Self> {
        let value = value.trim().trim_start_matches('#');

        match value.len() {
            6 => {
                let red = u8::from_str_radix(&value[0..2], 16).ok()?;
                let green = u8::from_str_radix(&value[2..4], 16).ok()?;
                let blue = u8::from_str_radix(&value[4..6], 16).ok()?;

                Some(Self::rgb(red, green, blue))
            }
            8 => {
                let red = u8::from_str_radix(&value[0..2], 16).ok()?;
                let green = u8::from_str_radix(&value[2..4], 16).ok()?;
                let blue = u8::from_str_radix(&value[4..6], 16).ok()?;
                let alpha = u8::from_str_radix(&value[6..8], 16).ok()?;

                Some(Self::rgba(red, green, blue, alpha))
            }
            _ => None,
        }
    }

    pub fn to_hex(self) -> String {
        if self.alpha == 255 {
            format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
        } else {
            format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                self.red, self.green, self.blue, self.alpha
            )
        }
    }

    pub fn with_alpha(self, alpha: u8) -> Self {
        Self { alpha, ..self }
    }

    pub fn luminance(self) -> f32 {
        let red = self.red as f32 / 255.0;
        let green = self.green as f32 / 255.0;
        let blue = self.blue as f32 / 255.0;

        0.2126 * red + 0.7152 * green + 0.0722 * blue
    }

    pub fn contrast_ratio(self, other: Self) -> f32 {
        let a = self.luminance();
        let b = other.luminance();

        let lighter = a.max(b);
        let darker = a.min(b);

        (lighter + 0.05) / (darker + 0.05)
    }
}

/// Main semantic color palette.
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub background: Color,
    pub foreground: Color,
    pub surface: Color,
    pub surface_alt: Color,

    pub border: Color,
    pub border_active: Color,

    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,

    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    pub selection: Color,
    pub selection_foreground: Color,

    pub cursor: Color,
    pub cursor_foreground: Color,

    pub link: Color,
    pub muted: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            background: Color::rgb(18, 18, 18),
            foreground: Color::rgb(235, 235, 235),
            surface: Color::rgb(28, 28, 28),
            surface_alt: Color::rgb(38, 38, 38),

            border: Color::rgb(70, 70, 70),
            border_active: Color::rgb(120, 120, 120),

            primary: Color::rgb(90, 160, 255),
            secondary: Color::rgb(150, 150, 150),
            accent: Color::rgb(255, 170, 70),

            success: Color::rgb(80, 200, 120),
            warning: Color::rgb(240, 190, 70),
            error: Color::rgb(240, 90, 90),
            info: Color::rgb(80, 170, 230),

            selection: Color::rgb(70, 100, 150),
            selection_foreground: Color::WHITE,

            cursor: Color::WHITE,
            cursor_foreground: Color::BLACK,

            link: Color::rgb(90, 180, 255),
            muted: Color::rgb(140, 140, 140),
        }
    }
}

impl ColorPalette {
    pub fn is_high_contrast(&self) -> bool {
        self.foreground.contrast_ratio(self.background) >= 7.0
    }

    pub fn ensure_readability(&self) -> bool {
        self.foreground.contrast_ratio(self.background) >= 4.5
    }
}

/// Standard 16-color ANSI palette.
#[derive(Debug, Clone)]
pub struct AnsiPalette {
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,

    pub bright_black: Color,
    pub bright_red: Color,
    pub bright_green: Color,
    pub bright_yellow: Color,
    pub bright_blue: Color,
    pub bright_magenta: Color,
    pub bright_cyan: Color,
    pub bright_white: Color,
}

impl Default for AnsiPalette {
    fn default() -> Self {
        Self {
            black: Color::rgb(0, 0, 0),
            red: Color::rgb(205, 49, 49),
            green: Color::rgb(13, 188, 121),
            yellow: Color::rgb(229, 229, 16),
            blue: Color::rgb(36, 114, 200),
            magenta: Color::rgb(188, 63, 188),
            cyan: Color::rgb(17, 168, 205),
            white: Color::rgb(229, 229, 229),

            bright_black: Color::rgb(102, 102, 102),
            bright_red: Color::rgb(241, 76, 76),
            bright_green: Color::rgb(35, 209, 139),
            bright_yellow: Color::rgb(245, 245, 67),
            bright_blue: Color::rgb(59, 142, 234),
            bright_magenta: Color::rgb(214, 112, 214),
            bright_cyan: Color::rgb(41, 184, 219),
            bright_white: Color::rgb(255, 255, 255),
        }
    }
}
