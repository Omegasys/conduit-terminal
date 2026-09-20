use super::rgba::Rgba;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TrueColor {
    color: Rgba,
}

impl TrueColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            color: Rgba::rgb(red, green, blue),
        }
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            color: Rgba::new(red, green, blue, alpha),
        }
    }

    pub fn color(&self) -> Rgba {
        self.color
    }

    pub fn red(&self) -> u8 {
        self.color.red()
    }

    pub fn green(&self) -> u8 {
        self.color.green()
    }

    pub fn blue(&self) -> u8 {
        self.color.blue()
    }

    pub fn alpha(&self) -> u8 {
        self.color.alpha()
    }

    pub fn to_sgr_foreground(&self) -> String {
        format!(
            "\x1b[38;2;{};{};{}m",
            self.red(),
            self.green(),
            self.blue()
        )
    }

    pub fn to_sgr_background(&self) -> String {
        format!(
            "\x1b[48;2;{};{};{}m",
            self.red(),
            self.green(),
            self.blue()
        )
    }
}
