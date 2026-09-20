use super::rgba::Rgba;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContrastLevel {
    Fail,
    LargeText,
    NormalText,
}

#[derive(Clone, Copy, Debug)]
pub struct ContrastResult {
    ratio: f32,
    level: ContrastLevel,
}

impl ContrastResult {
    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    pub fn level(&self) -> ContrastLevel {
        self.level
    }

    pub fn passes(&self) -> bool {
        !matches!(self.level, ContrastLevel::Fail)
    }
}

pub struct ColorContrast;

impl ColorContrast {
    pub fn relative_luminance(color: Rgba) -> f32 {
        fn channel(value: u8) -> f32 {
            let value = value as f32 / 255.0;

            if value <= 0.03928 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        }

        0.2126 * channel(color.red())
            + 0.7152 * channel(color.green())
            + 0.0722 * channel(color.blue())
    }

    pub fn ratio(foreground: Rgba, background: Rgba) -> f32 {
        let foreground = Self::relative_luminance(foreground);
        let background = Self::relative_luminance(background);

        let lighter = foreground.max(background);
        let darker = foreground.min(background);

        (lighter + 0.05) / (darker + 0.05)
    }

    pub fn evaluate(foreground: Rgba, background: Rgba) -> ContrastResult {
        let ratio = Self::ratio(foreground, background);

        let level = if ratio >= 7.0 {
            ContrastLevel::NormalText
        } else if ratio >= 4.5 {
            ContrastLevel::NormalText
        } else if ratio >= 3.0 {
            ContrastLevel::LargeText
        } else {
            ContrastLevel::Fail
        };

        ContrastResult { ratio, level }
    }
}
