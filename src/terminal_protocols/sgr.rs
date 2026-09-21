//! SGR (Select Graphic Rendition) parsing.
//!
//! SGR controls terminal text attributes such as bold, underline, inverse,
//! blink, foreground/background colors, and extended RGB colors.

use crate::colors::{AnsiColor, Color, Rgba};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SgrIntensity {
    Normal,
    Bold,
    Faint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SgrUnderline {
    None,
    Single,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SgrBlink {
    None,
    Slow,
    Rapid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SgrAttributes {
    pub intensity: SgrIntensity,
    pub italic: bool,
    pub underline: SgrUnderline,
    pub blink: SgrBlink,
    pub inverse: bool,
    pub invisible: bool,
    pub crossed_out: bool,
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub underline_color: Option<Color>,
    pub overline: bool,
    pub framed: bool,
    pub encircled: bool,
}

impl Default for SgrAttributes {
    fn default() -> Self {
        Self {
            intensity: SgrIntensity::Normal,
            italic: false,
            underline: SgrUnderline::None,
            blink: SgrBlink::None,
            inverse: false,
            invisible: false,
            crossed_out: false,
            foreground: None,
            background: None,
            underline_color: None,
            overline: false,
            framed: false,
            encircled: false,
        }
    }
}

impl SgrAttributes {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn apply(&mut self, params: &[u16]) {
        let mut index = 0;

        if params.is_empty() {
            self.reset();
            return;
        }

        while index < params.len() {
            let code = params[index];

            match code {
                0 => self.reset(),

                1 => self.intensity = SgrIntensity::Bold,
                2 => self.intensity = SgrIntensity::Faint,
                3 => self.italic = true,
                4 => self.underline = SgrUnderline::Single,
                5 => self.blink = SgrBlink::Slow,
                6 => self.blink = SgrBlink::Rapid,
                7 => self.inverse = true,
                8 => self.invisible = true,
                9 => self.crossed_out = true,

                21 => self.underline = SgrUnderline::Double,

                22 => self.intensity = SgrIntensity::Normal,
                23 => self.italic = false,
                24 => self.underline = SgrUnderline::None,
                25 => self.blink = SgrBlink::None,
                27 => self.inverse = false,
                28 => self.invisible = false,
                29 => self.crossed_out = false,

                30..=37 => {
                    self.foreground = Some(Color::Ansi(
                        AnsiColor::new((code - 30) as u8),
                    ));
                }

                38 => {
                    if let Some((color, consumed)) = parse_extended_color(params, index) {
                        self.foreground = color;
                        index += consumed;
                    }
                }

                39 => self.foreground = None,

                40..=47 => {
                    self.background = Some(Color::Ansi(
                        AnsiColor::new((code - 40) as u8),
                    ));
                }

                48 => {
                    if let Some((color, consumed)) = parse_extended_color(params, index) {
                        self.background = color;
                        index += consumed;
                    }
                }

                49 => self.background = None,

                51 => self.framed = true,
                52 => self.encircled = true,
                53 => self.overline = true,

                54 => {
                    self.framed = false;
                    self.encircled = false;
                }

                55 => self.overline = false,

                58 => {
                    if let Some((color, consumed)) = parse_extended_color(params, index) {
                        self.underline_color = color;
                        index += consumed;
                    }
                }

                59 => self.underline_color = None,

                90..=97 => {
                    self.foreground = Some(Color::Ansi(
                        AnsiColor::new((code - 90 + 8) as u8),
                    ));
                }

                100..=107 => {
                    self.background = Some(Color::Ansi(
                        AnsiColor::new((code - 100 + 8) as u8),
                    ));
                }

                _ => {}
            }

            index += 1;
        }
    }
}

fn parse_extended_color(
    params: &[u16],
    index: usize,
) -> Option<(Option<Color>, usize)> {
    let mode = *params.get(index + 1)?;

    match mode {
        5 => {
            let value = *params.get(index + 2)? as u8;

            Some((
                Some(Color::Ansi(AnsiColor::new(value))),
                2,
            ))
        }

        2 => {
            let r = *params.get(index + 2)? as u8;
            let g = *params.get(index + 3)? as u8;
            let b = *params.get(index + 4)? as u8;

            Some((
                Some(Color::Rgba(Rgba::rgb(r, g, b))),
                4,
            ))
        }

        _ => None,
    }
}
