use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    None,
    Errors,
    Warnings,
    Normal,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationSound {
    None,
    System,
    Custom,
}

#[derive(Debug, Clone)]
pub struct NotificationSettings {
    enabled: bool,
    level: NotificationLevel,
    position: NotificationPosition,
    duration_ms: u64,
    maximum_visible: usize,
    sound: NotificationSound,
    sound_volume: f64,
    desktop_notifications: bool,
    terminal_bell_notifications: bool,
    tab_activity_notifications: bool,
    background_process_notifications: bool,
    connection_notifications: bool,
    update_notifications: bool,
    plugin_notifications: bool,
    security_notifications: bool,
    errors_persistent: bool,
    show_timestamps: bool,
    show_icons: bool,
    allow_interaction: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            level: NotificationLevel::All,
            position: NotificationPosition::BottomRight,
            duration_ms: 5000,
            maximum_visible: 5,
            sound: NotificationSound::System,
            sound_volume: 0.5,
            desktop_notifications: true,
            terminal_bell_notifications: true,
            tab_activity_notifications: true,
            background_process_notifications: true,
            connection_notifications: true,
            update_notifications: true,
            plugin_notifications: true,
            security_notifications: true,
            errors_persistent: true,
            show_timestamps: false,
            show_icons: true,
            allow_interaction: true,
        }
    }
}

impl NotificationSettings {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn level(&self) -> NotificationLevel {
        self.level
    }

    pub fn set_level(&mut self, value: NotificationLevel) {
        self.level = value;
    }

    pub fn position(&self) -> NotificationPosition {
        self.position
    }

    pub fn set_position(&mut self, value: NotificationPosition) {
        self.position = value;
    }

    pub fn duration_ms(&self) -> u64 {
        self.duration_ms
    }

    pub fn set_duration_ms(&mut self, value: u64) {
        self.duration_ms = value.clamp(500, 60_000);
    }

    pub fn maximum_visible(&self) -> usize {
        self.maximum_visible
    }

    pub fn set_maximum_visible(&mut self, value: usize) {
        self.maximum_visible = value.clamp(1, 100);
    }

    pub fn sound(&self) -> NotificationSound {
        self.sound
    }

    pub fn set_sound(&mut self, value: NotificationSound) {
        self.sound = value;
    }

    pub fn sound_volume(&self) -> f64 {
        self.sound_volume
    }

    pub fn set_sound_volume(&mut self, value: f64) {
        self.sound_volume = value.clamp(0.0, 1.0);
    }

    pub fn desktop_notifications(&self) -> bool {
        self.desktop_notifications
    }

    pub fn set_desktop_notifications(&mut self, value: bool) {
        self.desktop_notifications = value;
    }

    pub fn terminal_bell_notifications(&self) -> bool {
        self.terminal_bell_notifications
    }

    pub fn set_terminal_bell_notifications(&mut self, value: bool) {
        self.terminal_bell_notifications = value;
    }

    pub fn tab_activity_notifications(&self) -> bool {
        self.tab_activity_notifications
    }

    pub fn set_tab_activity_notifications(&mut self, value: bool) {
        self.tab_activity_notifications = value;
    }

    pub fn background_process_notifications(&self) -> bool {
        self.background_process_notifications
    }

    pub fn set_background_process_notifications(&mut self, value: bool) {
        self.background_process_notifications = value;
    }

    pub fn connection_notifications(&self) -> bool {
        self.connection_notifications
    }

    pub fn set_connection_notifications(&mut self, value: bool) {
        self.connection_notifications = value;
    }

    pub fn update_notifications(&self) -> bool {
        self.update_notifications
    }

    pub fn set_update_notifications(&mut self, value: bool) {
        self.update_notifications = value;
    }

    pub fn plugin_notifications(&self) -> bool {
        self.plugin_notifications
    }

    pub fn set_plugin_notifications(&mut self, value: bool) {
        self.plugin_notifications = value;
    }

    pub fn security_notifications(&self) -> bool {
        self.security_notifications
    }

    pub fn set_security_notifications(&mut self, value: bool) {
        self.security_notifications = value;
    }

    pub fn errors_persistent(&self) -> bool {
        self.errors_persistent
    }

    pub fn set_errors_persistent(&mut self, value: bool) {
        self.errors_persistent = value;
    }

    pub fn show_timestamps(&self) -> bool {
        self.show_timestamps
    }

    pub fn set_show_timestamps(&mut self, value: bool) {
        self.show_timestamps = value;
    }

    pub fn show_icons(&self) -> bool {
        self.show_icons
    }

    pub fn set_show_icons(&mut self, value: bool) {
        self.show_icons = value;
    }

    pub fn allow_interaction(&self) -> bool {
        self.allow_interaction
    }

    pub fn set_allow_interaction(&mut self, value: bool) {
        self.allow_interaction = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert("enabled".into(), ConfigValue::Boolean(self.enabled));
        values.insert(
            "level".into(),
            ConfigValue::String(format!("{:?}", self.level).to_lowercase()),
        );
        values.insert(
            "position".into(),
            ConfigValue::String(format!("{:?}", self.position).to_lowercase()),
        );
        values.insert(
            "duration_ms".into(),
            ConfigValue::Integer(self.duration_ms as i64),
        );
        values.insert(
            "maximum_visible".into(),
            ConfigValue::Integer(self.maximum_visible as i64),
        );
        values.insert(
            "sound".into(),
            ConfigValue::String(format!("{:?}", self.sound).to_lowercase()),
        );
        values.insert(
            "sound_volume".into(),
            ConfigValue::Float(self.sound_volume),
        );
        values.insert(
            "desktop_notifications".into(),
            ConfigValue::Boolean(self.desktop_notifications),
        );
        values.insert(
            "terminal_bell_notifications".into(),
            ConfigValue::Boolean(self.terminal_bell_notifications),
        );
        values.insert(
            "tab_activity_notifications".into(),
            ConfigValue::Boolean(self.tab_activity_notifications),
        );
        values.insert(
            "background_process_notifications".into(),
            ConfigValue::Boolean(self.background_process_notifications),
        );
        values.insert(
            "connection_notifications".into(),
            ConfigValue::Boolean(self.connection_notifications),
        );
        values.insert(
            "update_notifications".into(),
            ConfigValue::Boolean(self.update_notifications),
        );
        values.insert(
            "plugin_notifications".into(),
            ConfigValue::Boolean(self.plugin_notifications),
        );
        values.insert(
            "security_notifications".into(),
            ConfigValue::Boolean(self.security_notifications),
        );
        values.insert(
            "errors_persistent".into(),
            ConfigValue::Boolean(self.errors_persistent),
        );
        values.insert(
            "show_timestamps".into(),
            ConfigValue::Boolean(self.show_timestamps),
        );
        values.insert(
            "show_icons".into(),
            ConfigValue::Boolean(self.show_icons),
        );
        values.insert(
            "allow_interaction".into(),
            ConfigValue::Boolean(self.allow_interaction),
        );

        values
    }
}
