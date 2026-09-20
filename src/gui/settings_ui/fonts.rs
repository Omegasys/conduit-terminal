#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Medium,
    SemiBold,
    Bold,
    Light,
}

#[derive(Debug, Clone)]
pub struct FontSettings {
    terminal_font: String,
    terminal_size: f32,
    terminal_weight: FontWeight,
    ui_font: String,
    ui_size: f32,
    ui_weight: FontWeight,
    line_spacing: f32,
    letter_spacing: f32,
    ligatures_enabled: bool,
    antialiasing_enabled: bool,
    fallback_fonts: Vec<String>,
}

impl FontSettings {
    pub fn new() -> Self {
        Self {
            terminal_font: "monospace".to_string(),
            terminal_size: 12.0,
            terminal_weight: FontWeight::Normal,
            ui_font: "sans-serif".to_string(),
            ui_size: 10.0,
            ui_weight: FontWeight::Normal,
            line_spacing: 1.0,
            letter_spacing: 0.0,
            ligatures_enabled: true,
            antialiasing_enabled: true,
            fallback_fonts: Vec::new(),
        }
    }

    pub fn terminal_font(&self) -> &str {
        &self.terminal_font
    }

    pub fn terminal_size(&self) -> f32 {
        self.terminal_size
    }

    pub fn terminal_weight(&self) -> FontWeight {
        self.terminal_weight
    }

    pub fn ui_font(&self) -> &str {
        &self.ui_font
    }

    pub fn ui_size(&self) -> f32 {
        self.ui_size
    }

    pub fn ui_weight(&self) -> FontWeight {
        self.ui_weight
    }

    pub fn line_spacing(&self) -> f32 {
        self.line_spacing
    }

    pub fn letter_spacing(&self) -> f32 {
        self.letter_spacing
    }

    pub fn ligatures_enabled(&self) -> bool {
        self.ligatures_enabled
    }

    pub fn antialiasing_enabled(&self) -> bool {
        self.antialiasing_enabled
    }

    pub fn fallback_fonts(&self) -> &[String] {
        &self.fallback_fonts
    }

    pub fn set_terminal_font(&mut self, value: impl Into<String>) {
        self.terminal_font = value.into();
    }

    pub fn set_terminal_size(&mut self, value: f32) {
        self.terminal_size = value.clamp(6.0, 72.0);
    }

    pub fn set_terminal_weight(&mut self, value: FontWeight) {
        self.terminal_weight = value;
    }

    pub fn set_ui_font(&mut self, value: impl Into<String>) {
        self.ui_font = value.into();
    }

    pub fn set_ui_size(&mut self, value: f32) {
        self.ui_size = value.clamp(6.0, 48.0);
    }

    pub fn set_ui_weight(&mut self, value: FontWeight) {
        self.ui_weight = value;
    }

    pub fn set_line_spacing(&mut self, value: f32) {
        self.line_spacing = value.clamp(0.5, 3.0);
    }

    pub fn set_letter_spacing(&mut self, value: f32) {
        self.letter_spacing = value.clamp(-2.0, 10.0);
    }

    pub fn set_ligatures_enabled(&mut self, value: bool) {
        self.ligatures_enabled = value;
    }

    pub fn set_antialiasing_enabled(&mut self, value: bool) {
        self.antialiasing_enabled = value;
    }

    pub fn add_fallback_font(&mut self, font: impl Into<String>) {
        let font = font.into();

        if !self.fallback_fonts.contains(&font) {
            self.fallback_fonts.push(font);
        }
    }

    pub fn remove_fallback_font(&mut self, font: &str) {
        self.fallback_fonts.retain(|item| item != font);
    }

    pub fn clear_fallback_fonts(&mut self) {
        self.fallback_fonts.clear();
    }
}

impl Default for FontSettings {
    fn default() -> Self {
        Self::new()
    }
}
