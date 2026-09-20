#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgba {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Rgba {
    pub const BLACK: Self = Self::new_const(0, 0, 0, 255);
    pub const WHITE: Self = Self::new_const(255, 255, 255, 255);
    pub const TRANSPARENT: Self = Self::new_const(0, 0, 0, 0);

    pub const fn new_const(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::new_const(red, green, blue, alpha)
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::new(red, green, blue, 255)
    }

    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    pub fn with_alpha(self, alpha: u8) -> Self {
        Self { alpha, ..self }
    }

    pub fn with_red(self, red: u8) -> Self {
        Self { red, ..self }
    }

    pub fn with_green(self, green: u8) -> Self {
        Self { green, ..self }
    }

    pub fn with_blue(self, blue: u8) -> Self {
        Self { blue, ..self }
    }

    pub fn to_hex_rgb(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    pub fn to_hex_rgba(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.alpha
        )
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

                Some(Self::new(red, green, blue, alpha))
            }
            _ => None,
        }
    }

    pub fn is_opaque(&self) -> bool {
        self.alpha == 255
    }

    pub fn is_transparent(&self) -> bool {
        self.alpha == 0
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self::BLACK
    }
}
