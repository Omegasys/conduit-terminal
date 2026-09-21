use crate::colors::Rgba;

/// Text rendering mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextRenderMode {
    Grayscale,
    AntiAliased,
    Subpixel,
    Monochrome,
}

impl Default for TextRenderMode {
    fn default() -> Self {
        Self::AntiAliased
    }
}

/// Terminal text weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextWeight {
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
}

impl Default for TextWeight {
    fn default() -> Self {
        Self::Normal
    }
}

/// Text metrics used by the terminal grid.
#[derive(Debug, Clone, Copy)]
pub struct TextMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
    pub advance_width: f32,
    pub line_height: f32,
}

impl TextMetrics {
    pub fn new(
        ascent: f32,
        descent: f32,
        line_gap: f32,
        advance_width: f32,
    ) -> Self {
        Self {
            ascent,
            descent,
            line_gap,
            advance_width,
            line_height: ascent + descent + line_gap,
        }
    }
}

/// A terminal text cell.
#[derive(Debug, Clone)]
pub struct TextCell {
    character: String,
    position: [f32; 2],
    foreground: Rgba,
    background: Rgba,
    size: f32,
    weight: TextWeight,
}

impl TextCell {
    pub fn new<S>(
        character: S,
        x: f32,
        y: f32,
        foreground: Rgba,
        background: Rgba,
        size: f32,
    ) -> Self
    where
        S: Into<String>,
    {
        Self {
            character: character.into(),
            position: [x, y],
            foreground,
            background,
            size,
            weight: TextWeight::Normal,
        }
    }

    pub fn character(&self) -> &str {
        &self.character
    }

    pub fn position(&self) -> [f32; 2] {
        self.position
    }

    pub fn foreground(&self) -> Rgba {
        self.foreground
    }

    pub fn background(&self) -> Rgba {
        self.background
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn weight(&self) -> TextWeight {
        self.weight
    }

    pub fn set_weight(&mut self, weight: TextWeight) {
        self.weight = weight;
    }
}

/// Text rendering configuration.
#[derive(Debug, Clone)]
pub struct TextRenderer {
    mode: TextRenderMode,
    weight: TextWeight,
    font_size: f32,
    letter_spacing: f32,
    line_spacing: f32,
    bold_is_bright: bool,
    use_color_emoji: bool,
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self {
            mode: TextRenderMode::AntiAliased,
            weight: TextWeight::Normal,
            font_size: 14.0,
            letter_spacing: 0.0,
            line_spacing: 0.0,
            bold_is_bright: true,
            use_color_emoji: true,
        }
    }
}

impl TextRenderer {
    pub fn mode(&self) -> TextRenderMode {
        self.mode
    }

    pub fn weight(&self) -> TextWeight {
        self.weight
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn letter_spacing(&self) -> f32 {
        self.letter_spacing
    }

    pub fn line_spacing(&self) -> f32 {
        self.line_spacing
    }

    pub fn bold_is_bright(&self) -> bool {
        self.bold_is_bright
    }

    pub fn use_color_emoji(&self) -> bool {
        self.use_color_emoji
    }

    pub fn set_mode(&mut self, mode: TextRenderMode) {
        self.mode = mode;
    }

    pub fn set_weight(&mut self, weight: TextWeight) {
        self.weight = weight;
    }

    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size.max(1.0);
    }

    pub fn set_letter_spacing(&mut self, spacing: f32) {
        self.letter_spacing = spacing;
    }

    pub fn set_line_spacing(&mut self, spacing: f32) {
        self.line_spacing = spacing;
    }

    pub fn set_bold_is_bright(&mut self, enabled: bool) {
        self.bold_is_bright = enabled;
    }

    pub fn set_use_color_emoji(&mut self, enabled: bool) {
        self.use_color_emoji = enabled;
    }
}
