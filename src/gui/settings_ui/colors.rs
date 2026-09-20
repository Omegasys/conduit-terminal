use crate::colors::{
    Color,
    ColorCapabilities,
    Rgba,
    SemanticColor,
    SemanticColorMap,
};

#[derive(Clone, Debug)]
pub struct ColorScheme {
    pub foreground: Color,
    pub background: Color,
    pub cursor: Color,
    pub selection: Color,
    pub border: Color,
    pub accent: Color,
    pub semantic: SemanticColorMap,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            foreground: Color::rgb(216, 222, 233),
            background: Color::rgb(46, 52, 64),
            cursor: Color::rgb(136, 192, 208),
            selection: Color::rgba(67, 76, 94, 180),
            border: Color::rgb(76, 86, 106),
            accent: Color::rgb(136, 192, 208),
            semantic: SemanticColorMap::new(),
        }
    }
}

impl ColorScheme {
    pub fn set_semantic(&mut self, color: SemanticColor, value: Color) {
        self.semantic.set(color, value);
    }

    pub fn semantic(&self, color: SemanticColor) -> Option<Color> {
        self.semantic.get(color)
    }
}

#[derive(Clone, Debug)]
pub struct ColorSettings {
    pub scheme: ColorScheme,
    pub truecolor: bool,
    pub transparency: bool,
    pub alpha: u8,
    pub high_contrast: bool,
    pub color_adjustments: bool,
    pub capabilities: ColorCapabilities,
}

impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            scheme: ColorScheme::default(),
            truecolor: true,
            transparency: true,
            alpha: 255,
            high_contrast: false,
            color_adjustments: false,
            capabilities: ColorCapabilities::truecolor_alpha(),
        }
    }
}

impl ColorSettings {
    pub fn set_foreground(&mut self, color: Color) {
        self.scheme.foreground = color;
    }

    pub fn set_background(&mut self, color: Color) {
        self.scheme.background = color;
    }

    pub fn set_cursor(&mut self, color: Color) {
        self.scheme.cursor = color;
    }

    pub fn set_selection(&mut self, color: Color) {
        self.scheme.selection = color;
    }

    pub fn set_border(&mut self, color: Color) {
        self.scheme.border = color;
    }

    pub fn set_accent(&mut self, color: Color) {
        self.scheme.accent = color;
    }

    pub fn set_transparency(&mut self, enabled: bool) {
        self.transparency = enabled;

        if !enabled {
            self.alpha = 255;
        }
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        self.alpha = alpha;
    }

    pub fn set_truecolor(&mut self, enabled: bool) {
        self.truecolor = enabled;

        if enabled {
            self.capabilities = ColorCapabilities::truecolor_alpha();
        }
    }

    pub fn set_high_contrast(&mut self, enabled: bool) {
        self.high_contrast = enabled;
    }

    pub fn set_color_adjustments(&mut self, enabled: bool) {
        self.color_adjustments = enabled;
    }

    pub fn effective_background(&self) -> Rgba {
        let color = self.scheme.background.rgba().unwrap_or(Rgba::BLACK);

        if self.transparency {
            color.with_alpha(self.alpha)
        } else {
            color.with_alpha(255)
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
