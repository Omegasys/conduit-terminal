use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenReaderMode {
    Automatic,
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionMode {
    Normal,
    Reduced,
    None,
}

#[derive(Debug, Clone)]
pub struct AccessibilitySettings {
    high_contrast: bool,
    large_text: bool,
    text_scale: f64,
    ui_scale: f64,
    screen_reader: ScreenReaderMode,
    keyboard_navigation: bool,
    focus_indicators: bool,
    show_tooltips: bool,
    reduce_motion: bool,
    motion_mode: MotionMode,
    reduce_transparency: bool,
    bold_text: bool,
    underline_links: bool,
    distinguish_colors: bool,
    audible_focus: bool,
    extended_timeouts: bool,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            high_contrast: false,
            large_text: false,
            text_scale: 1.0,
            ui_scale: 1.0,
            screen_reader: ScreenReaderMode::Automatic,
            keyboard_navigation: true,
            focus_indicators: true,
            show_tooltips: true,
            reduce_motion: false,
            motion_mode: MotionMode::Normal,
            reduce_transparency: false,
            bold_text: false,
            underline_links: true,
            distinguish_colors: true,
            audible_focus: false,
            extended_timeouts: false,
        }
    }
}

impl AccessibilitySettings {
    pub fn high_contrast(&self) -> bool {
        self.high_contrast
    }

    pub fn set_high_contrast(&mut self, value: bool) {
        self.high_contrast = value;
    }

    pub fn large_text(&self) -> bool {
        self.large_text
    }

    pub fn set_large_text(&mut self, value: bool) {
        self.large_text = value;

        if value && self.text_scale < 1.25 {
            self.text_scale = 1.25;
        }
    }

    pub fn text_scale(&self) -> f64 {
        self.text_scale
    }

    pub fn set_text_scale(&mut self, value: f64) {
        self.text_scale = value.clamp(0.5, 4.0);

        if self.large_text && self.text_scale < 1.25 {
            self.text_scale = 1.25;
        }
    }

    pub fn effective_text_scale(&self) -> f64 {
        if self.large_text {
            self.text_scale.max(1.25)
        } else {
            self.text_scale
        }
    }

    pub fn ui_scale(&self) -> f64 {
        self.ui_scale
    }

    pub fn set_ui_scale(&mut self, value: f64) {
        self.ui_scale = value.clamp(0.5, 4.0);
    }

    pub fn screen_reader(&self) -> ScreenReaderMode {
        self.screen_reader
    }

    pub fn set_screen_reader(&mut self, value: ScreenReaderMode) {
        self.screen_reader = value;
    }

    pub fn keyboard_navigation(&self) -> bool {
        self.keyboard_navigation
    }

    pub fn set_keyboard_navigation(&mut self, value: bool) {
        self.keyboard_navigation = value;
    }

    pub fn focus_indicators(&self) -> bool {
        self.focus_indicators
    }

    pub fn set_focus_indicators(&mut self, value: bool) {
        self.focus_indicators = value;
    }

    pub fn show_tooltips(&self) -> bool {
        self.show_tooltips
    }

    pub fn set_show_tooltips(&mut self, value: bool) {
        self.show_tooltips = value;
    }

    pub fn reduce_motion(&self) -> bool {
        self.reduce_motion
    }

    pub fn set_reduce_motion(&mut self, value: bool) {
        self.reduce_motion = value;
    }

    pub fn motion_mode(&self) -> MotionMode {
        self.motion_mode
    }

    pub fn set_motion_mode(&mut self, value: MotionMode) {
        self.motion_mode = value;
    }

    pub fn reduce_transparency(&self) -> bool {
        self.reduce_transparency
    }

    pub fn set_reduce_transparency(&mut self, value: bool) {
        self.reduce_transparency = value;
    }

    pub fn bold_text(&self) -> bool {
        self.bold_text
    }

    pub fn set_bold_text(&mut self, value: bool) {
        self.bold_text = value;
    }

    pub fn underline_links(&self) -> bool {
        self.underline_links
    }

    pub fn set_underline_links(&mut self, value: bool) {
        self.underline_links = value;
    }

    pub fn distinguish_colors(&self) -> bool {
        self.distinguish_colors
    }

    pub fn set_distinguish_colors(&mut self, value: bool) {
        self.distinguish_colors = value;
    }

    pub fn audible_focus(&self) -> bool {
        self.audible_focus
    }

    pub fn set_audible_focus(&mut self, value: bool) {
        self.audible_focus = value;
    }

    pub fn extended_timeouts(&self) -> bool {
        self.extended_timeouts
    }

    pub fn set_extended_timeouts(&mut self, value: bool) {
        self.extended_timeouts = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "high_contrast".into(),
            ConfigValue::Boolean(self.high_contrast),
        );
        values.insert(
            "large_text".into(),
            ConfigValue::Boolean(self.large_text),
        );
        values.insert(
            "text_scale".into(),
            ConfigValue::Float(self.text_scale),
        );
        values.insert("ui_scale".into(), ConfigValue::Float(self.ui_scale));
        values.insert(
            "screen_reader".into(),
            ConfigValue::String(format!("{:?}", self.screen_reader).to_lowercase()),
        );
        values.insert(
            "keyboard_navigation".into(),
            ConfigValue::Boolean(self.keyboard_navigation),
        );
        values.insert(
            "focus_indicators".into(),
            ConfigValue::Boolean(self.focus_indicators),
        );
        values.insert(
            "show_tooltips".into(),
            ConfigValue::Boolean(self.show_tooltips),
        );
        values.insert(
            "reduce_motion".into(),
            ConfigValue::Boolean(self.reduce_motion),
        );
        values.insert(
            "motion_mode".into(),
            ConfigValue::String(format!("{:?}", self.motion_mode).to_lowercase()),
        );
        values.insert(
            "reduce_transparency".into(),
            ConfigValue::Boolean(self.reduce_transparency),
        );
        values.insert(
            "bold_text".into(),
            ConfigValue::Boolean(self.bold_text),
        );
        values.insert(
            "underline_links".into(),
            ConfigValue::Boolean(self.underline_links),
        );
        values.insert(
            "distinguish_colors".into(),
            ConfigValue::Boolean(self.distinguish_colors),
        );
        values.insert(
            "audible_focus".into(),
            ConfigValue::Boolean(self.audible_focus),
        );
        values.insert(
            "extended_timeouts".into(),
            ConfigValue::Boolean(self.extended_timeouts),
        );

        values
    }
}
