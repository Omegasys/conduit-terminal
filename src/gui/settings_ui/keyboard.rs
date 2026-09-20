use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardLayout {
    System,
    US,
    UK,
    German,
    French,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyRepeatMode {
    System,
    Enabled,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct KeyboardSettings {
    layout: KeyboardLayout,
    repeat_mode: KeyRepeatMode,
    repeat_delay_ms: u32,
    repeat_rate_hz: u32,
    enable_alt_screen: bool,
    use_emacs_keys: bool,
    use_vi_keys: bool,
    allow_global_shortcuts: bool,
    command_palette_shortcut: String,
    copy_shortcut: String,
    paste_shortcut: String,
    new_tab_shortcut: String,
    close_tab_shortcut: String,
    new_window_shortcut: String,
}

impl Default for KeyboardSettings {
    fn default() -> Self {
        Self {
            layout: KeyboardLayout::System,
            repeat_mode: KeyRepeatMode::System,
            repeat_delay_ms: 500,
            repeat_rate_hz: 30,
            enable_alt_screen: true,
            use_emacs_keys: false,
            use_vi_keys: false,
            allow_global_shortcuts: true,
            command_palette_shortcut: "Ctrl+Shift+P".into(),
            copy_shortcut: "Ctrl+C".into(),
            paste_shortcut: "Ctrl+V".into(),
            new_tab_shortcut: "Ctrl+T".into(),
            close_tab_shortcut: "Ctrl+W".into(),
            new_window_shortcut: "Ctrl+Shift+N".into(),
        }
    }
}

impl KeyboardSettings {
    pub fn layout(&self) -> KeyboardLayout {
        self.layout
    }

    pub fn set_layout(&mut self, value: KeyboardLayout) {
        self.layout = value;
    }

    pub fn repeat_mode(&self) -> KeyRepeatMode {
        self.repeat_mode
    }

    pub fn set_repeat_mode(&mut self, value: KeyRepeatMode) {
        self.repeat_mode = value;
    }

    pub fn repeat_delay_ms(&self) -> u32 {
        self.repeat_delay_ms
    }

    pub fn set_repeat_delay_ms(&mut self, value: u32) {
        self.repeat_delay_ms = value.clamp(50, 2000);
    }

    pub fn repeat_rate_hz(&self) -> u32 {
        self.repeat_rate_hz
    }

    pub fn set_repeat_rate_hz(&mut self, value: u32) {
        self.repeat_rate_hz = value.clamp(1, 120);
    }

    pub fn enable_alt_screen(&self) -> bool {
        self.enable_alt_screen
    }

    pub fn set_enable_alt_screen(&mut self, value: bool) {
        self.enable_alt_screen = value;
    }

    pub fn use_emacs_keys(&self) -> bool {
        self.use_emacs_keys
    }

    pub fn set_use_emacs_keys(&mut self, value: bool) {
        self.use_emacs_keys = value;
        if value {
            self.use_vi_keys = false;
        }
    }

    pub fn use_vi_keys(&self) -> bool {
        self.use_vi_keys
    }

    pub fn set_use_vi_keys(&mut self, value: bool) {
        self.use_vi_keys = value;
        if value {
            self.use_emacs_keys = false;
        }
    }

    pub fn allow_global_shortcuts(&self) -> bool {
        self.allow_global_shortcuts
    }

    pub fn set_allow_global_shortcuts(&mut self, value: bool) {
        self.allow_global_shortcuts = value;
    }

    pub fn command_palette_shortcut(&self) -> &str {
        &self.command_palette_shortcut
    }

    pub fn set_command_palette_shortcut(&mut self, value: impl Into<String>) {
        self.command_palette_shortcut = value.into();
    }

    pub fn copy_shortcut(&self) -> &str {
        &self.copy_shortcut
    }

    pub fn set_copy_shortcut(&mut self, value: impl Into<String>) {
        self.copy_shortcut = value.into();
    }

    pub fn paste_shortcut(&self) -> &str {
        &self.paste_shortcut
    }

    pub fn set_paste_shortcut(&mut self, value: impl Into<String>) {
        self.paste_shortcut = value.into();
    }

    pub fn new_tab_shortcut(&self) -> &str {
        &self.new_tab_shortcut
    }

    pub fn set_new_tab_shortcut(&mut self, value: impl Into<String>) {
        self.new_tab_shortcut = value.into();
    }

    pub fn close_tab_shortcut(&self) -> &str {
        &self.close_tab_shortcut
    }

    pub fn set_close_tab_shortcut(&mut self, value: impl Into<String>) {
        self.close_tab_shortcut = value.into();
    }

    pub fn new_window_shortcut(&self) -> &str {
        &self.new_window_shortcut
    }

    pub fn set_new_window_shortcut(&mut self, value: impl Into<String>) {
        self.new_window_shortcut = value.into();
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "layout".into(),
            ConfigValue::String(format!("{:?}", self.layout).to_lowercase()),
        );
        values.insert(
            "repeat_mode".into(),
            ConfigValue::String(format!("{:?}", self.repeat_mode).to_lowercase()),
        );
        values.insert(
            "repeat_delay_ms".into(),
            ConfigValue::Integer(self.repeat_delay_ms as i64),
        );
        values.insert(
            "repeat_rate_hz".into(),
            ConfigValue::Integer(self.repeat_rate_hz as i64),
        );
        values.insert(
            "enable_alt_screen".into(),
            ConfigValue::Boolean(self.enable_alt_screen),
        );
        values.insert(
            "use_emacs_keys".into(),
            ConfigValue::Boolean(self.use_emacs_keys),
        );
        values.insert(
            "use_vi_keys".into(),
            ConfigValue::Boolean(self.use_vi_keys),
        );
        values.insert(
            "allow_global_shortcuts".into(),
            ConfigValue::Boolean(self.allow_global_shortcuts),
        );
        values.insert(
            "command_palette_shortcut".into(),
            ConfigValue::String(self.command_palette_shortcut.clone()),
        );
        values.insert(
            "copy_shortcut".into(),
            ConfigValue::String(self.copy_shortcut.clone()),
        );
        values.insert(
            "paste_shortcut".into(),
            ConfigValue::String(self.paste_shortcut.clone()),
        );
        values.insert(
            "new_tab_shortcut".into(),
            ConfigValue::String(self.new_tab_shortcut.clone()),
        );
        values.insert(
            "close_tab_shortcut".into(),
            ConfigValue::String(self.close_tab_shortcut.clone()),
        );
        values.insert(
            "new_window_shortcut".into(),
            ConfigValue::String(self.new_window_shortcut.clone()),
        );

        values
    }
}
