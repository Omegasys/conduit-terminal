use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabCloseBehavior {
    Close,
    Confirm,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabOverflowBehavior {
    Scroll,
    Menu,
    ScrollAndMenu,
}

#[derive(Debug, Clone)]
pub struct TabSettings {
    close_behavior: TabCloseBehavior,
    overflow_behavior: TabOverflowBehavior,
    show_close_buttons: bool,
    show_new_tab_button: bool,
    show_tab_icons: bool,
    show_activity_indicators: bool,
    show_pinned_tabs: bool,
    allow_reordering: bool,
    allow_drag_out: bool,
    double_click_rename: bool,
    middle_click_close: bool,
    middle_click_new_tab: bool,
    restore_closed_tabs: bool,
    max_title_length: usize,
    confirm_close_multiple_tabs: bool,
}

impl Default for TabSettings {
    fn default() -> Self {
        Self {
            close_behavior: TabCloseBehavior::Close,
            overflow_behavior: TabOverflowBehavior::ScrollAndMenu,
            show_close_buttons: true,
            show_new_tab_button: true,
            show_tab_icons: true,
            show_activity_indicators: true,
            show_pinned_tabs: true,
            allow_reordering: true,
            allow_drag_out: true,
            double_click_rename: true,
            middle_click_close: true,
            middle_click_new_tab: false,
            restore_closed_tabs: true,
            max_title_length: 80,
            confirm_close_multiple_tabs: true,
        }
    }
}

impl TabSettings {
    pub fn close_behavior(&self) -> TabCloseBehavior {
        self.close_behavior
    }

    pub fn set_close_behavior(&mut self, value: TabCloseBehavior) {
        self.close_behavior = value;
    }

    pub fn overflow_behavior(&self) -> TabOverflowBehavior {
        self.overflow_behavior
    }

    pub fn set_overflow_behavior(&mut self, value: TabOverflowBehavior) {
        self.overflow_behavior = value;
    }

    pub fn show_close_buttons(&self) -> bool {
        self.show_close_buttons
    }

    pub fn set_show_close_buttons(&mut self, value: bool) {
        self.show_close_buttons = value;
    }

    pub fn show_new_tab_button(&self) -> bool {
        self.show_new_tab_button
    }

    pub fn set_show_new_tab_button(&mut self, value: bool) {
        self.show_new_tab_button = value;
    }

    pub fn show_tab_icons(&self) -> bool {
        self.show_tab_icons
    }

    pub fn set_show_tab_icons(&mut self, value: bool) {
        self.show_tab_icons = value;
    }

    pub fn show_activity_indicators(&self) -> bool {
        self.show_activity_indicators
    }

    pub fn set_show_activity_indicators(&mut self, value: bool) {
        self.show_activity_indicators = value;
    }

    pub fn show_pinned_tabs(&self) -> bool {
        self.show_pinned_tabs
    }

    pub fn set_show_pinned_tabs(&mut self, value: bool) {
        self.show_pinned_tabs = value;
    }

    pub fn allow_reordering(&self) -> bool {
        self.allow_reordering
    }

    pub fn set_allow_reordering(&mut self, value: bool) {
        self.allow_reordering = value;
    }

    pub fn allow_drag_out(&self) -> bool {
        self.allow_drag_out
    }

    pub fn set_allow_drag_out(&mut self, value: bool) {
        self.allow_drag_out = value;
    }

    pub fn double_click_rename(&self) -> bool {
        self.double_click_rename
    }

    pub fn set_double_click_rename(&mut self, value: bool) {
        self.double_click_rename = value;
    }

    pub fn middle_click_close(&self) -> bool {
        self.middle_click_close
    }

    pub fn set_middle_click_close(&mut self, value: bool) {
        self.middle_click_close = value;
    }

    pub fn middle_click_new_tab(&self) -> bool {
        self.middle_click_new_tab
    }

    pub fn set_middle_click_new_tab(&mut self, value: bool) {
        self.middle_click_new_tab = value;
    }

    pub fn restore_closed_tabs(&self) -> bool {
        self.restore_closed_tabs
    }

    pub fn set_restore_closed_tabs(&mut self, value: bool) {
        self.restore_closed_tabs = value;
    }

    pub fn max_title_length(&self) -> usize {
        self.max_title_length
    }

    pub fn set_max_title_length(&mut self, value: usize) {
        self.max_title_length = value.clamp(10, 500);
    }

    pub fn confirm_close_multiple_tabs(&self) -> bool {
        self.confirm_close_multiple_tabs
    }

    pub fn set_confirm_close_multiple_tabs(&mut self, value: bool) {
        self.confirm_close_multiple_tabs = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "close_behavior".into(),
            ConfigValue::String(format!("{:?}", self.close_behavior).to_lowercase()),
        );
        values.insert(
            "overflow_behavior".into(),
            ConfigValue::String(format!("{:?}", self.overflow_behavior).to_lowercase()),
        );
        values.insert(
            "show_close_buttons".into(),
            ConfigValue::Boolean(self.show_close_buttons),
        );
        values.insert(
            "show_new_tab_button".into(),
            ConfigValue::Boolean(self.show_new_tab_button),
        );
        values.insert(
            "show_tab_icons".into(),
            ConfigValue::Boolean(self.show_tab_icons),
        );
        values.insert(
            "show_activity_indicators".into(),
            ConfigValue::Boolean(self.show_activity_indicators),
        );
        values.insert(
            "show_pinned_tabs".into(),
            ConfigValue::Boolean(self.show_pinned_tabs),
        );
        values.insert(
            "allow_reordering".into(),
            ConfigValue::Boolean(self.allow_reordering),
        );
        values.insert(
            "allow_drag_out".into(),
            ConfigValue::Boolean(self.allow_drag_out),
        );
        values.insert(
            "double_click_rename".into(),
            ConfigValue::Boolean(self.double_click_rename),
        );
        values.insert(
            "middle_click_close".into(),
            ConfigValue::Boolean(self.middle_click_close),
        );
        values.insert(
            "middle_click_new_tab".into(),
            ConfigValue::Boolean(self.middle_click_new_tab),
        );
        values.insert(
            "restore_closed_tabs".into(),
            ConfigValue::Boolean(self.restore_closed_tabs),
        );
        values.insert(
            "max_title_length".into(),
            ConfigValue::Integer(self.max_title_length as i64),
        );
        values.insert(
            "confirm_close_multiple_tabs".into(),
            ConfigValue::Boolean(self.confirm_close_multiple_tabs),
        );

        values
    }
}
