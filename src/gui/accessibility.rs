/// Accessibility configuration for the GUI.
#[derive(Debug, Clone)]
pub struct AccessibilitySettings {
    high_contrast: bool,
    large_text: bool,
    reduce_motion: bool,
    screen_reader: bool,
    keyboard_navigation: bool,
    focus_indicators: bool,
    show_tooltips: bool,
    text_scale: f32,
    ui_scale: f32,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            high_contrast: false,
            large_text: false,
            reduce_motion: false,
            screen_reader: false,
            keyboard_navigation: true,
            focus_indicators: true,
            show_tooltips: true,
            text_scale: 1.0,
            ui_scale: 1.0,
        }
    }
}

impl AccessibilitySettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn high_contrast(&self) -> bool {
        self.high_contrast
    }

    pub fn large_text(&self) -> bool {
        self.large_text
    }

    pub fn reduce_motion(&self) -> bool {
        self.reduce_motion
    }

    pub fn screen_reader(&self) -> bool {
        self.screen_reader
    }

    pub fn keyboard_navigation(&self) -> bool {
        self.keyboard_navigation
    }

    pub fn focus_indicators(&self) -> bool {
        self.focus_indicators
    }

    pub fn show_tooltips(&self) -> bool {
        self.show_tooltips
    }

    pub fn text_scale(&self) -> f32 {
        self.text_scale
    }

    pub fn ui_scale(&self) -> f32 {
        self.ui_scale
    }

    pub fn set_high_contrast(&mut self, enabled: bool) {
        self.high_contrast = enabled;
    }

    pub fn set_large_text(&mut self, enabled: bool) {
        self.large_text = enabled;

        if enabled && self.text_scale < 1.25 {
            self.text_scale = 1.25;
        }
    }

    pub fn set_reduce_motion(&mut self, enabled: bool) {
        self.reduce_motion = enabled;
    }

    pub fn set_screen_reader(&mut self, enabled: bool) {
        self.screen_reader = enabled;
    }

    pub fn set_keyboard_navigation(&mut self, enabled: bool) {
        self.keyboard_navigation = enabled;
    }

    pub fn set_focus_indicators(&mut self, enabled: bool) {
        self.focus_indicators = enabled;
    }

    pub fn set_show_tooltips(&mut self, enabled: bool) {
        self.show_tooltips = enabled;
    }

    pub fn set_text_scale(&mut self, scale: f32) {
        self.text_scale = scale.clamp(0.5, 4.0);
    }

    pub fn set_ui_scale(&mut self, scale: f32) {
        self.ui_scale = scale.clamp(0.5, 4.0);
    }

    pub fn effective_text_scale(&self) -> f32 {
        let mut scale = self.text_scale;

        if self.large_text {
            scale = scale.max(1.25);
        }

        scale
    }
}
