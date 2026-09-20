use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileStartupBehavior {
    Default,
    New,
    Restore,
    Prompt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileScope {
    Global,
    Workspace,
    Window,
    Tab,
    Pane,
}

#[derive(Debug, Clone)]
pub struct ProfileSettings {
    default_profile: String,
    startup_behavior: ProfileStartupBehavior,
    scope: ProfileScope,
    allow_profile_inheritance: bool,
    allow_profile_overrides: bool,
    remember_last_profile: bool,
    auto_create_profiles: bool,
    validate_profiles: bool,
    inherit_environment: bool,
    inherit_theme: bool,
    inherit_font: bool,
    inherit_shell: bool,
    inherit_terminal: bool,
    inherit_keybindings: bool,
    inherit_security: bool,
    show_profile_selector: bool,
    show_profile_icons: bool,
}

impl Default for ProfileSettings {
    fn default() -> Self {
        Self {
            default_profile: "default".into(),
            startup_behavior: ProfileStartupBehavior::Default,
            scope: ProfileScope::Global,
            allow_profile_inheritance: true,
            allow_profile_overrides: true,
            remember_last_profile: true,
            auto_create_profiles: false,
            validate_profiles: true,
            inherit_environment: true,
            inherit_theme: true,
            inherit_font: true,
            inherit_shell: true,
            inherit_terminal: true,
            inherit_keybindings: true,
            inherit_security: true,
            show_profile_selector: true,
            show_profile_icons: true,
        }
    }
}

impl ProfileSettings {
    pub fn default_profile(&self) -> &str {
        &self.default_profile
    }

    pub fn set_default_profile(&mut self, value: impl Into<String>) {
        let value = value.into();

        if !value.trim().is_empty() {
            self.default_profile = value;
        }
    }

    pub fn startup_behavior(&self) -> ProfileStartupBehavior {
        self.startup_behavior
    }

    pub fn set_startup_behavior(&mut self, value: ProfileStartupBehavior) {
        self.startup_behavior = value;
    }

    pub fn scope(&self) -> ProfileScope {
        self.scope
    }

    pub fn set_scope(&mut self, value: ProfileScope) {
        self.scope = value;
    }

    pub fn allow_profile_inheritance(&self) -> bool {
        self.allow_profile_inheritance
    }

    pub fn set_allow_profile_inheritance(&mut self, value: bool) {
        self.allow_profile_inheritance = value;
    }

    pub fn allow_profile_overrides(&self) -> bool {
        self.allow_profile_overrides
    }

    pub fn set_allow_profile_overrides(&mut self, value: bool) {
        self.allow_profile_overrides = value;
    }

    pub fn remember_last_profile(&self) -> bool {
        self.remember_last_profile
    }

    pub fn set_remember_last_profile(&mut self, value: bool) {
        self.remember_last_profile = value;
    }

    pub fn auto_create_profiles(&self) -> bool {
        self.auto_create_profiles
    }

    pub fn set_auto_create_profiles(&mut self, value: bool) {
        self.auto_create_profiles = value;
    }

    pub fn validate_profiles(&self) -> bool {
        self.validate_profiles
    }

    pub fn set_validate_profiles(&mut self, value: bool) {
        self.validate_profiles = value;
    }

    pub fn inherit_environment(&self) -> bool {
        self.inherit_environment
    }

    pub fn set_inherit_environment(&mut self, value: bool) {
        self.inherit_environment = value;
    }

    pub fn inherit_theme(&self) -> bool {
        self.inherit_theme
    }

    pub fn set_inherit_theme(&mut self, value: bool) {
        self.inherit_theme = value;
    }

    pub fn inherit_font(&self) -> bool {
        self.inherit_font
    }

    pub fn set_inherit_font(&mut self, value: bool) {
        self.inherit_font = value;
    }

    pub fn inherit_shell(&self) -> bool {
        self.inherit_shell
    }

    pub fn set_inherit_shell(&mut self, value: bool) {
        self.inherit_shell = value;
    }

    pub fn inherit_terminal(&self) -> bool {
        self.inherit_terminal
    }

    pub fn set_inherit_terminal(&mut self, value: bool) {
        self.inherit_terminal = value;
    }

    pub fn inherit_keybindings(&self) -> bool {
        self.inherit_keybindings
    }

    pub fn set_inherit_keybindings(&mut self, value: bool) {
        self.inherit_keybindings = value;
    }

    pub fn inherit_security(&self) -> bool {
        self.inherit_security
    }

    pub fn set_inherit_security(&mut self, value: bool) {
        self.inherit_security = value;
    }

    pub fn show_profile_selector(&self) -> bool {
        self.show_profile_selector
    }

    pub fn set_show_profile_selector(&mut self, value: bool) {
        self.show_profile_selector = value;
    }

    pub fn show_profile_icons(&self) -> bool {
        self.show_profile_icons
    }

    pub fn set_show_profile_icons(&mut self, value: bool) {
        self.show_profile_icons = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "default_profile".into(),
            ConfigValue::String(self.default_profile.clone()),
        );
        values.insert(
            "startup_behavior".into(),
            ConfigValue::String(format!("{:?}", self.startup_behavior).to_lowercase()),
        );
        values.insert(
            "scope".into(),
            ConfigValue::String(format!("{:?}", self.scope).to_lowercase()),
        );
        values.insert(
            "allow_profile_inheritance".into(),
            ConfigValue::Boolean(self.allow_profile_inheritance),
        );
        values.insert(
            "allow_profile_overrides".into(),
            ConfigValue::Boolean(self.allow_profile_overrides),
        );
        values.insert(
            "remember_last_profile".into(),
            ConfigValue::Boolean(self.remember_last_profile),
        );
        values.insert(
            "auto_create_profiles".into(),
            ConfigValue::Boolean(self.auto_create_profiles),
        );
        values.insert(
            "validate_profiles".into(),
            ConfigValue::Boolean(self.validate_profiles),
        );
        values.insert(
            "inherit_environment".into(),
            ConfigValue::Boolean(self.inherit_environment),
        );
        values.insert(
            "inherit_theme".into(),
            ConfigValue::Boolean(self.inherit_theme),
        );
        values.insert(
            "inherit_font".into(),
            ConfigValue::Boolean(self.inherit_font),
        );
        values.insert(
            "inherit_shell".into(),
            ConfigValue::Boolean(self.inherit_shell),
        );
        values.insert(
            "inherit_terminal".into(),
            ConfigValue::Boolean(self.inherit_terminal),
        );
        values.insert(
            "inherit_keybindings".into(),
            ConfigValue::Boolean(self.inherit_keybindings),
        );
        values.insert(
            "inherit_security".into(),
            ConfigValue::Boolean(self.inherit_security),
        );
        values.insert(
            "show_profile_selector".into(),
            ConfigValue::Boolean(self.show_profile_selector),
        );
        values.insert(
            "show_profile_icons".into(),
            ConfigValue::Boolean(self.show_profile_icons),
        );

        values
    }
}
