use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginLoadMode {
    Disabled,
    Manual,
    Automatic,
    TrustedOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginUpdateMode {
    Disabled,
    Notify,
    Automatic,
}

#[derive(Debug, Clone)]
pub struct PluginsSettings {
    enabled: bool,
    load_mode: PluginLoadMode,
    update_mode: PluginUpdateMode,
    auto_discover: bool,
    hot_reload: bool,
    validate_manifests: bool,
    require_permissions: bool,
    isolate_plugins: bool,
    allow_network: bool,
    allow_filesystem: bool,
    allow_processes: bool,
    allow_clipboard: bool,
    allow_terminal_control: bool,
    allow_configuration_changes: bool,
    allow_ui_extensions: bool,
    plugin_cache_enabled: bool,
    plugin_cache_mb: usize,
    failed_plugin_limit: usize,
}

impl Default for PluginsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            load_mode: PluginLoadMode::TrustedOnly,
            update_mode: PluginUpdateMode::Notify,
            auto_discover: true,
            hot_reload: true,
            validate_manifests: true,
            require_permissions: true,
            isolate_plugins: true,
            allow_network: false,
            allow_filesystem: false,
            allow_processes: false,
            allow_clipboard: false,
            allow_terminal_control: true,
            allow_configuration_changes: false,
            allow_ui_extensions: true,
            plugin_cache_enabled: true,
            plugin_cache_mb: 256,
            failed_plugin_limit: 5,
        }
    }
}

impl PluginsSettings {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn load_mode(&self) -> PluginLoadMode {
        self.load_mode
    }

    pub fn set_load_mode(&mut self, value: PluginLoadMode) {
        self.load_mode = value;
    }

    pub fn update_mode(&self) -> PluginUpdateMode {
        self.update_mode
    }

    pub fn set_update_mode(&mut self, value: PluginUpdateMode) {
        self.update_mode = value;
    }

    pub fn auto_discover(&self) -> bool {
        self.auto_discover
    }

    pub fn set_auto_discover(&mut self, value: bool) {
        self.auto_discover = value;
    }

    pub fn hot_reload(&self) -> bool {
        self.hot_reload
    }

    pub fn set_hot_reload(&mut self, value: bool) {
        self.hot_reload = value;
    }

    pub fn validate_manifests(&self) -> bool {
        self.validate_manifests
    }

    pub fn set_validate_manifests(&mut self, value: bool) {
        self.validate_manifests = value;
    }

    pub fn require_permissions(&self) -> bool {
        self.require_permissions
    }

    pub fn set_require_permissions(&mut self, value: bool) {
        self.require_permissions = value;
    }

    pub fn isolate_plugins(&self) -> bool {
        self.isolate_plugins
    }

    pub fn set_isolate_plugins(&mut self, value: bool) {
        self.isolate_plugins = value;
    }

    pub fn allow_network(&self) -> bool {
        self.allow_network
    }

    pub fn set_allow_network(&mut self, value: bool) {
        self.allow_network = value;
    }

    pub fn allow_filesystem(&self) -> bool {
        self.allow_filesystem
    }

    pub fn set_allow_filesystem(&mut self, value: bool) {
        self.allow_filesystem = value;
    }

    pub fn allow_processes(&self) -> bool {
        self.allow_processes
    }

    pub fn set_allow_processes(&mut self, value: bool) {
        self.allow_processes = value;
    }

    pub fn allow_clipboard(&self) -> bool {
        self.allow_clipboard
    }

    pub fn set_allow_clipboard(&mut self, value: bool) {
        self.allow_clipboard = value;
    }

    pub fn allow_terminal_control(&self) -> bool {
        self.allow_terminal_control
    }

    pub fn set_allow_terminal_control(&mut self, value: bool) {
        self.allow_terminal_control = value;
    }

    pub fn allow_configuration_changes(&self) -> bool {
        self.allow_configuration_changes
    }

    pub fn set_allow_configuration_changes(&mut self, value: bool) {
        self.allow_configuration_changes = value;
    }

    pub fn allow_ui_extensions(&self) -> bool {
        self.allow_ui_extensions
    }

    pub fn set_allow_ui_extensions(&mut self, value: bool) {
        self.allow_ui_extensions = value;
    }

    pub fn plugin_cache_enabled(&self) -> bool {
        self.plugin_cache_enabled
    }

    pub fn set_plugin_cache_enabled(&mut self, value: bool) {
        self.plugin_cache_enabled = value;
    }

    pub fn plugin_cache_mb(&self) -> usize {
        self.plugin_cache_mb
    }

    pub fn set_plugin_cache_mb(&mut self, value: usize) {
        self.plugin_cache_mb = value.clamp(16, 4096);
    }

    pub fn failed_plugin_limit(&self) -> usize {
        self.failed_plugin_limit
    }

    pub fn set_failed_plugin_limit(&mut self, value: usize) {
        self.failed_plugin_limit = value.clamp(1, 100);
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert("enabled".into(), ConfigValue::Boolean(self.enabled));
        values.insert(
            "load_mode".into(),
            ConfigValue::String(format!("{:?}", self.load_mode).to_lowercase()),
        );
        values.insert(
            "update_mode".into(),
            ConfigValue::String(format!("{:?}", self.update_mode).to_lowercase()),
        );
        values.insert(
            "auto_discover".into(),
            ConfigValue::Boolean(self.auto_discover),
        );
        values.insert(
            "hot_reload".into(),
            ConfigValue::Boolean(self.hot_reload),
        );
        values.insert(
            "validate_manifests".into(),
            ConfigValue::Boolean(self.validate_manifests),
        );
        values.insert(
            "require_permissions".into(),
            ConfigValue::Boolean(self.require_permissions),
        );
        values.insert(
            "isolate_plugins".into(),
            ConfigValue::Boolean(self.isolate_plugins),
        );
        values.insert(
            "allow_network".into(),
            ConfigValue::Boolean(self.allow_network),
        );
        values.insert(
            "allow_filesystem".into(),
            ConfigValue::Boolean(self.allow_filesystem),
        );
        values.insert(
            "allow_processes".into(),
            ConfigValue::Boolean(self.allow_processes),
        );
        values.insert(
            "allow_clipboard".into(),
            ConfigValue::Boolean(self.allow_clipboard),
        );
        values.insert(
            "allow_terminal_control".into(),
            ConfigValue::Boolean(self.allow_terminal_control),
        );
        values.insert(
            "allow_configuration_changes".into(),
            ConfigValue::Boolean(self.allow_configuration_changes),
        );
        values.insert(
            "allow_ui_extensions".into(),
            ConfigValue::Boolean(self.allow_ui_extensions),
        );
        values.insert(
            "plugin_cache_enabled".into(),
            ConfigValue::Boolean(self.plugin_cache_enabled),
        );
        values.insert(
            "plugin_cache_mb".into(),
            ConfigValue::Integer(self.plugin_cache_mb as i64),
        );
        values.insert(
            "failed_plugin_limit".into(),
            ConfigValue::Integer(self.failed_plugin_limit as i64),
        );

        values
    }
}
