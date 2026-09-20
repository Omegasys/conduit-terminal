use crate::config_engine::ConfigValue;
use std::collections::BTreeMap;

/// Controls how Conduit selects and applies themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeSelectionMode {
    System,
    Default,
    LastUsed,
    Workspace,
    Profile,
    Manual,
}

impl ThemeSelectionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Default => "default",
            Self::LastUsed => "last_used",
            Self::Workspace => "workspace",
            Self::Profile => "profile",
            Self::Manual => "manual",
        }
    }
}

/// Controls automatic theme switching.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeScheduleMode {
    Disabled,
    System,
    Time,
    Custom,
}

impl ThemeScheduleMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::System => "system",
            Self::Time => "time",
            Self::Custom => "custom",
        }
    }
}

/// GUI theme settings.
#[derive(Debug, Clone)]
pub struct ThemeSettings {
    theme_name: String,
    selection_mode: ThemeSelectionMode,
    schedule_mode: ThemeScheduleMode,
    light_theme: String,
    dark_theme: String,
    automatic_switching: bool,
    light_start_time: String,
    dark_start_time: String,
    preview_enabled: bool,
    live_preview: bool,
    remember_theme_per_workspace: bool,
    remember_theme_per_profile: bool,
    allow_theme_extensions: bool,
    reload_themes_on_change: bool,
    custom_theme_directory: Option<String>,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            theme_name: "default".to_string(),
            selection_mode: ThemeSelectionMode::Default,
            schedule_mode: ThemeScheduleMode::Disabled,
            light_theme: "default-light".to_string(),
            dark_theme: "default-dark".to_string(),
            automatic_switching: false,
            light_start_time: "07:00".to_string(),
            dark_start_time: "19:00".to_string(),
            preview_enabled: true,
            live_preview: true,
            remember_theme_per_workspace: false,
            remember_theme_per_profile: false,
            allow_theme_extensions: true,
            reload_themes_on_change: true,
            custom_theme_directory: None,
        }
    }
}

impl ThemeSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn theme_name(&self) -> &str {
        &self.theme_name
    }

    pub fn selection_mode(&self) -> ThemeSelectionMode {
        self.selection_mode
    }

    pub fn schedule_mode(&self) -> ThemeScheduleMode {
        self.schedule_mode
    }

    pub fn light_theme(&self) -> &str {
        &self.light_theme
    }

    pub fn dark_theme(&self) -> &str {
        &self.dark_theme
    }

    pub fn automatic_switching(&self) -> bool {
        self.automatic_switching
    }

    pub fn light_start_time(&self) -> &str {
        &self.light_start_time
    }

    pub fn dark_start_time(&self) -> &str {
        &self.dark_start_time
    }

    pub fn preview_enabled(&self) -> bool {
        self.preview_enabled
    }

    pub fn live_preview(&self) -> bool {
        self.live_preview
    }

    pub fn remember_theme_per_workspace(&self) -> bool {
        self.remember_theme_per_workspace
    }

    pub fn remember_theme_per_profile(&self) -> bool {
        self.remember_theme_per_profile
    }

    pub fn allow_theme_extensions(&self) -> bool {
        self.allow_theme_extensions
    }

    pub fn reload_themes_on_change(&self) -> bool {
        self.reload_themes_on_change
    }

    pub fn custom_theme_directory(&self) -> Option<&str> {
        self.custom_theme_directory.as_deref()
    }

    pub fn set_theme_name<S: Into<String>>(&mut self, name: S) {
        let name = name.into();
        if !name.trim().is_empty() {
            self.theme_name = name;
        }
    }

    pub fn set_selection_mode(&mut self, mode: ThemeSelectionMode) {
        self.selection_mode = mode;
    }

    pub fn set_schedule_mode(&mut self, mode: ThemeScheduleMode) {
        self.schedule_mode = mode;
    }

    pub fn set_light_theme<S: Into<String>>(&mut self, theme: S) {
        let theme = theme.into();
        if !theme.trim().is_empty() {
            self.light_theme = theme;
        }
    }

    pub fn set_dark_theme<S: Into<String>>(&mut self, theme: S) {
        let theme = theme.into();
        if !theme.trim().is_empty() {
            self.dark_theme = theme;
        }
    }

    pub fn set_automatic_switching(&mut self, enabled: bool) {
        self.automatic_switching = enabled;
    }

    pub fn set_light_start_time<S: Into<String>>(&mut self, time: S) {
        self.light_start_time = time.into();
    }

    pub fn set_dark_start_time<S: Into<String>>(&mut self, time: S) {
        self.dark_start_time = time.into();
    }

    pub fn set_preview_enabled(&mut self, enabled: bool) {
        self.preview_enabled = enabled;
    }

    pub fn set_live_preview(&mut self, enabled: bool) {
        self.live_preview = enabled;
    }

    pub fn set_remember_theme_per_workspace(&mut self, enabled: bool) {
        self.remember_theme_per_workspace = enabled;
    }

    pub fn set_remember_theme_per_profile(&mut self, enabled: bool) {
        self.remember_theme_per_profile = enabled;
    }

    pub fn set_allow_theme_extensions(&mut self, enabled: bool) {
        self.allow_theme_extensions = enabled;
    }

    pub fn set_reload_themes_on_change(&mut self, enabled: bool) {
        self.reload_themes_on_change = enabled;
    }

    pub fn set_custom_theme_directory<S: Into<String>>(&mut self, path: Option<S>) {
        self.custom_theme_directory = path.map(Into::into);
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "theme_name".into(),
            ConfigValue::String(self.theme_name.clone()),
        );
        values.insert(
            "selection_mode".into(),
            ConfigValue::String(self.selection_mode.as_str().into()),
        );
        values.insert(
            "schedule_mode".into(),
            ConfigValue::String(self.schedule_mode.as_str().into()),
        );
        values.insert(
            "light_theme".into(),
            ConfigValue::String(self.light_theme.clone()),
        );
        values.insert(
            "dark_theme".into(),
            ConfigValue::String(self.dark_theme.clone()),
        );
        values.insert(
            "automatic_switching".into(),
            ConfigValue::Boolean(self.automatic_switching),
        );
        values.insert(
            "light_start_time".into(),
            ConfigValue::String(self.light_start_time.clone()),
        );
        values.insert(
            "dark_start_time".into(),
            ConfigValue::String(self.dark_start_time.clone()),
        );
        values.insert(
            "preview_enabled".into(),
            ConfigValue::Boolean(self.preview_enabled),
        );
        values.insert(
            "live_preview".into(),
            ConfigValue::Boolean(self.live_preview),
        );
        values.insert(
            "remember_theme_per_workspace".into(),
            ConfigValue::Boolean(self.remember_theme_per_workspace),
        );
        values.insert(
            "remember_theme_per_profile".into(),
            ConfigValue::Boolean(self.remember_theme_per_profile),
        );
        values.insert(
            "allow_theme_extensions".into(),
            ConfigValue::Boolean(self.allow_theme_extensions),
        );
        values.insert(
            "reload_themes_on_change".into(),
            ConfigValue::Boolean(self.reload_themes_on_change),
        );

        if let Some(path) = &self.custom_theme_directory {
            values.insert(
                "custom_theme_directory".into(),
                ConfigValue::String(path.clone()),
            );
        }

        values
    }
}
