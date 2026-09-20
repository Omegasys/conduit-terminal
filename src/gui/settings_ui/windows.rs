use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowPlacement {
    Remember,
    Center,
    Maximized,
    Fullscreen,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowCloseBehavior {
    Confirm,
    Close,
    Minimize,
}

#[derive(Debug, Clone)]
pub struct WindowSettings {
    placement: WindowPlacement,
    close_behavior: WindowCloseBehavior,
    remember_size: bool,
    remember_position: bool,
    remember_state: bool,
    resizable: bool,
    decorated: bool,
    always_on_top: bool,
    minimize_to_tray: bool,
    confirm_close_window: bool,
    open_new_windows_on_current_workspace: bool,
    reuse_existing_window: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            placement: WindowPlacement::Remember,
            close_behavior: WindowCloseBehavior::Close,
            remember_size: true,
            remember_position: true,
            remember_state: true,
            resizable: true,
            decorated: true,
            always_on_top: false,
            minimize_to_tray: false,
            confirm_close_window: true,
            open_new_windows_on_current_workspace: true,
            reuse_existing_window: false,
        }
    }
}

impl WindowSettings {
    pub fn placement(&self) -> WindowPlacement {
        self.placement
    }

    pub fn set_placement(&mut self, placement: WindowPlacement) {
        self.placement = placement;
    }

    pub fn close_behavior(&self) -> WindowCloseBehavior {
        self.close_behavior
    }

    pub fn set_close_behavior(&mut self, behavior: WindowCloseBehavior) {
        self.close_behavior = behavior;
    }

    pub fn remember_size(&self) -> bool {
        self.remember_size
    }

    pub fn set_remember_size(&mut self, value: bool) {
        self.remember_size = value;
    }

    pub fn remember_position(&self) -> bool {
        self.remember_position
    }

    pub fn set_remember_position(&mut self, value: bool) {
        self.remember_position = value;
    }

    pub fn remember_state(&self) -> bool {
        self.remember_state
    }

    pub fn set_remember_state(&mut self, value: bool) {
        self.remember_state = value;
    }

    pub fn resizable(&self) -> bool {
        self.resizable
    }

    pub fn set_resizable(&mut self, value: bool) {
        self.resizable = value;
    }

    pub fn decorated(&self) -> bool {
        self.decorated
    }

    pub fn set_decorated(&mut self, value: bool) {
        self.decorated = value;
    }

    pub fn always_on_top(&self) -> bool {
        self.always_on_top
    }

    pub fn set_always_on_top(&mut self, value: bool) {
        self.always_on_top = value;
    }

    pub fn minimize_to_tray(&self) -> bool {
        self.minimize_to_tray
    }

    pub fn set_minimize_to_tray(&mut self, value: bool) {
        self.minimize_to_tray = value;
    }

    pub fn confirm_close_window(&self) -> bool {
        self.confirm_close_window
    }

    pub fn set_confirm_close_window(&mut self, value: bool) {
        self.confirm_close_window = value;
    }

    pub fn open_new_windows_on_current_workspace(&self) -> bool {
        self.open_new_windows_on_current_workspace
    }

    pub fn set_open_new_windows_on_current_workspace(&mut self, value: bool) {
        self.open_new_windows_on_current_workspace = value;
    }

    pub fn reuse_existing_window(&self) -> bool {
        self.reuse_existing_window
    }

    pub fn set_reuse_existing_window(&mut self, value: bool) {
        self.reuse_existing_window = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "placement".into(),
            ConfigValue::String(format!("{:?}", self.placement).to_lowercase()),
        );
        values.insert(
            "close_behavior".into(),
            ConfigValue::String(format!("{:?}", self.close_behavior).to_lowercase()),
        );
        values.insert("remember_size".into(), ConfigValue::Boolean(self.remember_size));
        values.insert(
            "remember_position".into(),
            ConfigValue::Boolean(self.remember_position),
        );
        values.insert(
            "remember_state".into(),
            ConfigValue::Boolean(self.remember_state),
        );
        values.insert("resizable".into(), ConfigValue::Boolean(self.resizable));
        values.insert("decorated".into(), ConfigValue::Boolean(self.decorated));
        values.insert(
            "always_on_top".into(),
            ConfigValue::Boolean(self.always_on_top),
        );
        values.insert(
            "minimize_to_tray".into(),
            ConfigValue::Boolean(self.minimize_to_tray),
        );
        values.insert(
            "confirm_close_window".into(),
            ConfigValue::Boolean(self.confirm_close_window),
        );
        values.insert(
            "open_new_windows_on_current_workspace".into(),
            ConfigValue::Boolean(self.open_new_windows_on_current_workspace),
        );
        values.insert(
            "reuse_existing_window".into(),
            ConfigValue::Boolean(self.reuse_existing_window),
        );

        values
    }
}
