//! Theme data structures.

use super::{
    appearance::Appearance,
    colors::{AnsiPalette, ColorPalette},
    fonts::FontSet,
};

/// Unique theme identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThemeId(String);

impl ThemeId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ThemeId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ThemeId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Human-readable theme metadata.
#[derive(Debug, Clone)]
pub struct ThemeMetadata {
    pub name: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
}

impl ThemeMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            author: None,
            version: None,
            description: None,
            homepage: None,
        }
    }
}

/// Complete Conduit theme.
#[derive(Debug, Clone)]
pub struct Theme {
    pub id: ThemeId,
    pub metadata: ThemeMetadata,
    pub colors: ColorPalette,
    pub ansi: AnsiPalette,
    pub fonts: FontSet,
    pub appearance: Appearance,
}

impl Theme {
    pub fn new(id: impl Into<ThemeId>, metadata: ThemeMetadata) -> Self {
        Self {
            id: id.into(),
            metadata,
            colors: ColorPalette::default(),
            ansi: AnsiPalette::default(),
            fonts: FontSet::default(),
            appearance: Appearance::default(),
        }
    }

    pub fn id(&self) -> &ThemeId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.metadata.name.trim().is_empty() {
            return Err("theme name cannot be empty".into());
        }

        if self.colors.foreground.contrast_ratio(self.colors.background) < 1.5 {
            return Err("theme foreground/background contrast is too low".into());
        }

        if !(0.1..=1.0).contains(&self.appearance.opacity) {
            return Err("theme opacity must be between 0.1 and 1.0".into());
        }

        Ok(())
    }

    pub fn builtin_dark() -> Self {
        Self::new(
            ThemeId::new("conduit-dark"),
            ThemeMetadata::new("Conduit Dark"),
        )
    }

    pub fn builtin_light() -> Self {
        let mut theme = Self::new(
            ThemeId::new("conduit-light"),
            ThemeMetadata::new("Conduit Light"),
        );

        theme.colors.background = super::colors::Color::rgb(245, 245, 245);
        theme.colors.foreground = super::colors::Color::rgb(25, 25, 25);
        theme.colors.surface = super::colors::Color::rgb(255, 255, 255);
        theme.colors.surface_alt = super::colors::Color::rgb(235, 235, 235);

        theme
    }
}
