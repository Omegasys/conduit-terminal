//! Accessibility font scaling.

/// Preset font scaling levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontScalingPreset {
    Normal,
    Large,
    ExtraLarge,
    Huge,
    Custom,
}

impl Default for FontScalingPreset {
    fn default() -> Self {
        Self::Normal
    }
}

impl FontScalingPreset {
    pub fn scale(self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Large => 1.25,
            Self::ExtraLarge => 1.5,
            Self::Huge => 2.0,
            Self::Custom => 1.0,
        }
    }
}

/// Font scaling configuration.
#[derive(Debug, Clone, Copy)]
pub struct FontScaling {
    preset: FontScalingPreset,
    scale: f32,
    min_scale: f32,
    max_scale: f32,
}

impl Default for FontScaling {
    fn default() -> Self {
        Self {
            preset: FontScalingPreset::Normal,
            scale: 1.0,
            min_scale: 0.5,
            max_scale: 4.0,
        }
    }
}

impl FontScaling {
    pub fn preset(&self) -> FontScalingPreset {
        self.preset
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn set_preset(&mut self, preset: FontScalingPreset) {
        self.preset = preset;

        if preset != FontScalingPreset::Custom {
            self.scale = preset.scale();
        }
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale.clamp(self.min_scale, self.max_scale);
        self.preset = if (self.scale - 1.0).abs() < f32::EPSILON {
            FontScalingPreset::Normal
        } else {
            FontScalingPreset::Custom
        };
    }

    pub fn increase(&mut self) {
        self.set_scale(self.scale + 0.1);
    }

    pub fn decrease(&mut self) {
        self.set_scale(self.scale - 0.1);
    }

    pub fn reset(&mut self) {
        self.set_preset(FontScalingPreset::Normal);
    }

    pub fn is_scaled(&self) -> bool {
        (self.scale - 1.0).abs() > f32::EPSILON
    }

    /// Scale a logical font size.
    pub fn scale_size(&self, size: f32) -> f32 {
        (size * self.scale).max(1.0)
    }
}
