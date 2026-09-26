//! Global appearance preferences.

use super::colors::Color;

/// Determines how Conduit chooses light/dark appearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppearanceMode {
    System,
    Light,
    Dark,
}

impl Default for AppearanceMode {
    fn default() -> Self {
        Self::System
    }
}

/// Appearance configuration.
#[derive(Debug, Clone)]
pub struct Appearance {
    pub mode: AppearanceMode,
    pub opacity: f32,
    pub transparency: bool,
    pub rounded_corners: bool,
    pub shadows: bool,
    pub animations: bool,
    pub accent: Color,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            mode: AppearanceMode::System,
            opacity: 1.0,
            transparency: false,
            rounded_corners: true,
            shadows: true,
            animations: true,
            accent: Color::rgb(90, 160, 255),
        }
    }
}

impl Appearance {
    pub fn set_mode(&mut self, mode: AppearanceMode) {
        self.mode = mode;
    }

    pub fn is_dark(&self, system_dark: bool) -> bool {
        match self.mode {
            AppearanceMode::System => system_dark,
            AppearanceMode::Light => false,
            AppearanceMode::Dark => true,
        }
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.1, 1.0);
    }

    pub fn effective_opacity(&self) -> f32 {
        if self.transparency {
            self.opacity
        } else {
            1.0
        }
    }

    pub fn set_transparency(&mut self, enabled: bool) {
        self.transparency = enabled;
    }

    pub fn set_animations(&mut self, enabled: bool) {
        self.animations = enabled;
    }

    pub fn set_rounded_corners(&mut self, enabled: bool) {
        self.rounded_corners = enabled;
    }

    pub fn set_shadows(&mut self, enabled: bool) {
        self.shadows = enabled;
    }
}
