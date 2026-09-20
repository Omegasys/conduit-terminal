use super::rgba::Rgba;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Add,
}

pub struct ColorBlender;

impl ColorBlender {
    pub fn blend(foreground: Rgba, background: Rgba, mode: BlendMode) -> Rgba {
        let blend_channel = |a: u8, b: u8| -> u8 {
            match mode {
                BlendMode::Normal => a,
                BlendMode::Multiply => ((a as u16 * b as u16) / 255) as u8,
                BlendMode::Screen => {
                    255 - (((255 - a) as u16 * (255 - b) as u16) / 255) as u8
                }
                BlendMode::Add => a.saturating_add(b),
            }
        };

        let alpha = foreground.alpha() as f32 / 255.0;
        let inverse_alpha = 1.0 - alpha;

        let red = (blend_channel(foreground.red(), background.red()) as f32 * alpha
            + background.red() as f32 * inverse_alpha)
            .round() as u8;

        let green = (blend_channel(foreground.green(), background.green()) as f32 * alpha
            + background.green() as f32 * inverse_alpha)
            .round() as u8;

        let blue = (blend_channel(foreground.blue(), background.blue()) as f32 * alpha
            + background.blue() as f32 * inverse_alpha)
            .round() as u8;

        Rgba::rgb(red, green, blue)
    }

    pub fn overlay(foreground: Rgba, background: Rgba) -> Rgba {
        Self::blend(foreground, background, BlendMode::Normal)
    }

    pub fn lighten(color: Rgba, amount: u8) -> Rgba {
        Rgba::rgb(
            color.red().saturating_add(amount),
            color.green().saturating_add(amount),
            color.blue().saturating_add(amount),
        )
    }

    pub fn darken(color: Rgba, amount: u8) -> Rgba {
        Rgba::rgb(
            color.red().saturating_sub(amount),
            color.green().saturating_sub(amount),
            color.blue().saturating_sub(amount),
        )
    }
}
