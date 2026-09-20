use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseFocusMode {
    Click,
    Follow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Normal,
    Natural,
}

#[derive(Debug, Clone)]
pub struct MouseSettings {
    focus_mode: MouseFocusMode,
    scroll_direction: ScrollDirection,
    scroll_multiplier: f64,
    double_click_interval_ms: u32,
    middle_click_paste: bool,
    right_click_context_menu: bool,
    shift_right_click_paste: bool,
    mouse_reporting: bool,
    allow_text_selection: bool,
    allow_drag_selection: bool,
    allow_hyperlink_clicks: bool,
    block_mouse_reporting_for_security: bool,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            focus_mode: MouseFocusMode::Click,
            scroll_direction: ScrollDirection::Normal,
            scroll_multiplier: 1.0,
            double_click_interval_ms: 400,
            middle_click_paste: true,
            right_click_context_menu: true,
            shift_right_click_paste: true,
            mouse_reporting: true,
            allow_text_selection: true,
            allow_drag_selection: true,
            allow_hyperlink_clicks: true,
            block_mouse_reporting_for_security: false,
        }
    }
}

impl MouseSettings {
    pub fn focus_mode(&self) -> MouseFocusMode {
        self.focus_mode
    }

    pub fn set_focus_mode(&mut self, value: MouseFocusMode) {
        self.focus_mode = value;
    }

    pub fn scroll_direction(&self) -> ScrollDirection {
        self.scroll_direction
    }

    pub fn set_scroll_direction(&mut self, value: ScrollDirection) {
        self.scroll_direction = value;
    }

    pub fn scroll_multiplier(&self) -> f64 {
        self.scroll_multiplier
    }

    pub fn set_scroll_multiplier(&mut self, value: f64) {
        self.scroll_multiplier = value.clamp(0.1, 10.0);
    }

    pub fn double_click_interval_ms(&self) -> u32 {
        self.double_click_interval_ms
    }

    pub fn set_double_click_interval_ms(&mut self, value: u32) {
        self.double_click_interval_ms = value.clamp(100, 2000);
    }

    pub fn middle_click_paste(&self) -> bool {
        self.middle_click_paste
    }

    pub fn set_middle_click_paste(&mut self, value: bool) {
        self.middle_click_paste = value;
    }

    pub fn right_click_context_menu(&self) -> bool {
        self.right_click_context_menu
    }

    pub fn set_right_click_context_menu(&mut self, value: bool) {
        self.right_click_context_menu = value;
    }

    pub fn shift_right_click_paste(&self) -> bool {
        self.shift_right_click_paste
    }

    pub fn set_shift_right_click_paste(&mut self, value: bool) {
        self.shift_right_click_paste = value;
    }

    pub fn mouse_reporting(&self) -> bool {
        self.mouse_reporting
    }

    pub fn set_mouse_reporting(&mut self, value: bool) {
        self.mouse_reporting = value;
    }

    pub fn allow_text_selection(&self) -> bool {
        self.allow_text_selection
    }

    pub fn set_allow_text_selection(&mut self, value: bool) {
        self.allow_text_selection = value;
    }

    pub fn allow_drag_selection(&self) -> bool {
        self.allow_drag_selection
    }

    pub fn set_allow_drag_selection(&mut self, value: bool) {
        self.allow_drag_selection = value;
    }

    pub fn allow_hyperlink_clicks(&self) -> bool {
        self.allow_hyperlink_clicks
    }

    pub fn set_allow_hyperlink_clicks(&mut self, value: bool) {
        self.allow_hyperlink_clicks = value;
    }

    pub fn block_mouse_reporting_for_security(&self) -> bool {
        self.block_mouse_reporting_for_security
    }

    pub fn set_block_mouse_reporting_for_security(&mut self, value: bool) {
        self.block_mouse_reporting_for_security = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "focus_mode".into(),
            ConfigValue::String(format!("{:?}", self.focus_mode).to_lowercase()),
        );
        values.insert(
            "scroll_direction".into(),
            ConfigValue::String(format!("{:?}", self.scroll_direction).to_lowercase()),
        );
        values.insert(
            "scroll_multiplier".into(),
            ConfigValue::Float(self.scroll_multiplier),
        );
        values.insert(
            "double_click_interval_ms".into(),
            ConfigValue::Integer(self.double_click_interval_ms as i64),
        );
        values.insert(
            "middle_click_paste".into(),
            ConfigValue::Boolean(self.middle_click_paste),
        );
        values.insert(
            "right_click_context_menu".into(),
            ConfigValue::Boolean(self.right_click_context_menu),
        );
        values.insert(
            "shift_right_click_paste".into(),
            ConfigValue::Boolean(self.shift_right_click_paste),
        );
        values.insert(
            "mouse_reporting".into(),
            ConfigValue::Boolean(self.mouse_reporting),
        );
        values.insert(
            "allow_text_selection".into(),
            ConfigValue::Boolean(self.allow_text_selection),
        );
        values.insert(
            "allow_drag_selection".into(),
            ConfigValue::Boolean(self.allow_drag_selection),
        );
        values.insert(
            "allow_hyperlink_clicks".into(),
            ConfigValue::Boolean(self.allow_hyperlink_clicks),
        );
        values.insert(
            "block_mouse_reporting_for_security".into(),
            ConfigValue::Boolean(self.block_mouse_reporting_for_security),
        );

        values
    }
}
