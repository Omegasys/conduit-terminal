use std::fmt;

/// Font style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self::Normal
    }
}

/// Font weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontWeight {
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

/// Font descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontDescriptor {
    family: String,
    fallback: Vec<String>,
    style: FontStyle,
    weight: FontWeight,
    monospace: bool,
}

impl FontDescriptor {
    pub fn new<S>(family: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            family: family.into(),
            fallback: Vec::new(),
            style: FontStyle::Normal,
            weight: FontWeight::Normal,
            monospace: true,
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn fallback(&self) -> &[String] {
        &self.fallback
    }

    pub fn style(&self) -> FontStyle {
        self.style
    }

    pub fn weight(&self) -> FontWeight {
        self.weight
    }

    pub fn monospace(&self) -> bool {
        self.monospace
    }

    pub fn set_family<S>(&mut self, family: S)
    where
        S: Into<String>,
    {
        self.family = family.into();
    }

    pub fn add_fallback<S>(&mut self, family: S)
    where
        S: Into<String>,
    {
        let family = family.into();

        if !self.fallback.contains(&family) {
            self.fallback.push(family);
        }
    }

    pub fn remove_fallback(&mut self, family: &str) {
        self.fallback.retain(|item| item != family);
    }

    pub fn set_style(&mut self, style: FontStyle) {
        self.style = style;
    }

    pub fn set_weight(&mut self, weight: FontWeight) {
        self.weight = weight;
    }

    pub fn set_monospace(&mut self, monospace: bool) {
        self.monospace = monospace;
    }
}

impl fmt::Display for FontDescriptor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {:?} {:?}",
            self.family, self.style, self.weight
        )
    }
}

/// Font metrics.
#[derive(Debug, Clone, Copy)]
pub struct FontMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
    pub advance_width: f32,
    pub cap_height: f32,
    pub x_height: f32,
}

impl Default for FontMetrics {
    fn default() -> Self {
        Self {
            ascent: 11.0,
            descent: 3.0,
            line_gap: 0.0,
            advance_width: 8.0,
            cap_height: 10.0,
            x_height: 7.0,
        }
    }
}

/// Font manager used by the renderer.
#[derive(Debug, Default)]
pub struct FontManager {
    fonts: Vec<FontDescriptor>,
    primary: Option<String>,
}

impl FontManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, font: FontDescriptor) {
        if !self
            .fonts
            .iter()
            .any(|existing| existing.family() == font.family())
        {
            self.fonts.push(font);
        }
    }

    pub fn remove(&mut self, family: &str) {
        self.fonts.retain(|font| font.family() != family);

        if self.primary.as_deref() == Some(family) {
            self.primary = None;
        }
    }

    pub fn fonts(&self) -> &[FontDescriptor] {
        &self.fonts
    }

    pub fn find(&self, family: &str) -> Option<&FontDescriptor> {
        self.fonts
            .iter()
            .find(|font| font.family() == family)
    }

    pub fn set_primary<S>(&mut self, family: S)
    where
        S: Into<String>,
    {
        self.primary = Some(family.into());
    }

    pub fn primary(&self) -> Option<&str> {
        self.primary.as_deref()
    }

    pub fn len(&self) -> usize {
        self.fonts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fonts.is_empty()
    }
}
