//! Theme font definitions.

/// Font family.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFamily {
    pub name: String,
    pub fallback: Vec<String>,
}

impl FontFamily {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fallback: Vec::new(),
        }
    }

    pub fn with_fallback(mut self, family: impl Into<String>) -> Self {
        self.fallback.push(family.into());
        self
    }

    pub fn candidates(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.name.as_str())
            .chain(self.fallback.iter().map(String::as_str))
    }
}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

/// A complete font specification.
#[derive(Debug, Clone)]
pub struct FontSpec {
    pub family: FontFamily,
    pub size: f32,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub line_height: f32,
}

impl FontSpec {
    pub fn new(family: FontFamily, size: f32) -> Self {
        Self {
            family,
            size,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            line_height: 1.2,
        }
    }

    pub fn effective_line_height(&self) -> f32 {
        self.size * self.line_height
    }
}

/// Fonts used throughout Conduit.
#[derive(Debug, Clone)]
pub struct FontSet {
    pub terminal: FontSpec,
    pub interface: FontSpec,
    pub monospace: FontSpec,
    pub title: FontSpec,
    pub status: FontSpec,
}

impl Default for FontSet {
    fn default() -> Self {
        let mono = FontFamily::new("monospace");

        Self {
            terminal: FontSpec::new(mono.clone(), 14.0),
            interface: FontSpec::new(FontFamily::new("sans-serif"), 14.0),
            monospace: FontSpec::new(mono.clone(), 14.0),
            title: FontSpec::new(FontFamily::new("sans-serif"), 16.0),
            status: FontSpec::new(FontFamily::new("sans-serif"), 12.0),
        }
    }
}

impl FontSet {
    /// Scale all fonts by the supplied factor.
    pub fn scale(&mut self, factor: f32) {
        let factor = factor.max(0.1);

        self.terminal.size *= factor;
        self.interface.size *= factor;
        self.monospace.size *= factor;
        self.title.size *= factor;
        self.status.size *= factor;
    }
}
