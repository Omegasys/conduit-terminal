use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneSplitOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneFocusBehavior {
    FollowMouse,
    ClickToFocus,
    KeyboardOnly,
}

#[derive(Debug, Clone)]
pub struct PaneSettings {
    default_split_orientation: PaneSplitOrientation,
    focus_behavior: PaneFocusBehavior,
    allow_resize: bool,
    allow_swap: bool,
    allow_zoom: bool,
    allow_synchronization: bool,
    show_borders: bool,
    show_titles: bool,
    dim_inactive_panes: bool,
    inactive_opacity: f64,
    remember_layout: bool,
    confirm_close_pane: bool,
    minimum_width: u32,
    minimum_height: u32,
}

impl Default for PaneSettings {
    fn default() -> Self {
        Self {
            default_split_orientation: PaneSplitOrientation::Vertical,
            focus_behavior: PaneFocusBehavior::ClickToFocus,
            allow_resize: true,
            allow_swap: true,
            allow_zoom: true,
            allow_synchronization: true,
            show_borders: true,
            show_titles: false,
            dim_inactive_panes: false,
            inactive_opacity: 0.75,
            remember_layout: true,
            confirm_close_pane: false,
            minimum_width: 120,
            minimum_height: 60,
        }
    }
}

impl PaneSettings {
    pub fn default_split_orientation(&self) -> PaneSplitOrientation {
        self.default_split_orientation
    }

    pub fn set_default_split_orientation(&mut self, value: PaneSplitOrientation) {
        self.default_split_orientation = value;
    }

    pub fn focus_behavior(&self) -> PaneFocusBehavior {
        self.focus_behavior
    }

    pub fn set_focus_behavior(&mut self, value: PaneFocusBehavior) {
        self.focus_behavior = value;
    }

    pub fn allow_resize(&self) -> bool {
        self.allow_resize
    }

    pub fn set_allow_resize(&mut self, value: bool) {
        self.allow_resize = value;
    }

    pub fn allow_swap(&self) -> bool {
        self.allow_swap
    }

    pub fn set_allow_swap(&mut self, value: bool) {
        self.allow_swap = value;
    }

    pub fn allow_zoom(&self) -> bool {
        self.allow_zoom
    }

    pub fn set_allow_zoom(&mut self, value: bool) {
        self.allow_zoom = value;
    }

    pub fn allow_synchronization(&self) -> bool {
        self.allow_synchronization
    }

    pub fn set_allow_synchronization(&mut self, value: bool) {
        self.allow_synchronization = value;
    }

    pub fn show_borders(&self) -> bool {
        self.show_borders
    }

    pub fn set_show_borders(&mut self, value: bool) {
        self.show_borders = value;
    }

    pub fn show_titles(&self) -> bool {
        self.show_titles
    }

    pub fn set_show_titles(&mut self, value: bool) {
        self.show_titles = value;
    }

    pub fn dim_inactive_panes(&self) -> bool {
        self.dim_inactive_panes
    }

    pub fn set_dim_inactive_panes(&mut self, value: bool) {
        self.dim_inactive_panes = value;
    }

    pub fn inactive_opacity(&self) -> f64 {
        self.inactive_opacity
    }

    pub fn set_inactive_opacity(&mut self, value: f64) {
        self.inactive_opacity = value.clamp(0.0, 1.0);
    }

    pub fn remember_layout(&self) -> bool {
        self.remember_layout
    }

    pub fn set_remember_layout(&mut self, value: bool) {
        self.remember_layout = value;
    }

    pub fn confirm_close_pane(&self) -> bool {
        self.confirm_close_pane
    }

    pub fn set_confirm_close_pane(&mut self, value: bool) {
        self.confirm_close_pane = value;
    }

    pub fn minimum_width(&self) -> u32 {
        self.minimum_width
    }

    pub fn set_minimum_width(&mut self, value: u32) {
        self.minimum_width = value.max(40);
    }

    pub fn minimum_height(&self) -> u32 {
        self.minimum_height
    }

    pub fn set_minimum_height(&mut self, value: u32) {
        self.minimum_height = value.max(20);
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "default_split_orientation".into(),
            ConfigValue::String(
                format!("{:?}", self.default_split_orientation).to_lowercase(),
            ),
        );
        values.insert(
            "focus_behavior".into(),
            ConfigValue::String(format!("{:?}", self.focus_behavior).to_lowercase()),
        );
        values.insert("allow_resize".into(), ConfigValue::Boolean(self.allow_resize));
        values.insert("allow_swap".into(), ConfigValue::Boolean(self.allow_swap));
        values.insert("allow_zoom".into(), ConfigValue::Boolean(self.allow_zoom));
        values.insert(
            "allow_synchronization".into(),
            ConfigValue::Boolean(self.allow_synchronization),
        );
        values.insert("show_borders".into(), ConfigValue::Boolean(self.show_borders));
        values.insert("show_titles".into(), ConfigValue::Boolean(self.show_titles));
        values.insert(
            "dim_inactive_panes".into(),
            ConfigValue::Boolean(self.dim_inactive_panes),
        );
        values.insert(
            "inactive_opacity".into(),
            ConfigValue::Float(self.inactive_opacity),
        );
        values.insert(
            "remember_layout".into(),
            ConfigValue::Boolean(self.remember_layout),
        );
        values.insert(
            "confirm_close_pane".into(),
            ConfigValue::Boolean(self.confirm_close_pane),
        );
        values.insert(
            "minimum_width".into(),
            ConfigValue::Integer(self.minimum_width as i64),
        );
        values.insert(
            "minimum_height".into(),
            ConfigValue::Integer(self.minimum_height as i64),
        );

        values
    }
}
