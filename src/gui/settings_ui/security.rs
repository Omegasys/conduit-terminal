use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityMode {
    Normal,
    Restricted,
    Safe,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationMode {
    Never,
    RiskyActions,
    Always,
}

#[derive(Debug, Clone)]
pub struct SecuritySettings {
    mode: SecurityMode,
    confirmation_mode: ConfirmationMode,
    allow_clipboard_read: bool,
    allow_clipboard_write: bool,
    allow_hyperlinks: bool,
    allow_external_commands: bool,
    allow_file_access: bool,
    allow_network_access: bool,
    allow_plugins: bool,
    allow_plugin_network: bool,
    allow_plugin_filesystem: bool,
    allow_plugin_processes: bool,
    verify_plugin_signatures: bool,
    require_plugin_permissions: bool,
    warn_untrusted_output: bool,
    warn_external_links: bool,
    warn_paste_commands: bool,
    warn_sensitive_operations: bool,
    isolate_plugins: bool,
    safe_mode_on_error: bool,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            mode: SecurityMode::Normal,
            confirmation_mode: ConfirmationMode::RiskyActions,
            allow_clipboard_read: true,
            allow_clipboard_write: true,
            allow_hyperlinks: true,
            allow_external_commands: false,
            allow_file_access: true,
            allow_network_access: true,
            allow_plugins: true,
            allow_plugin_network: false,
            allow_plugin_filesystem: false,
            allow_plugin_processes: false,
            verify_plugin_signatures: false,
            require_plugin_permissions: true,
            warn_untrusted_output: true,
            warn_external_links: true,
            warn_paste_commands: true,
            warn_sensitive_operations: true,
            isolate_plugins: true,
            safe_mode_on_error: true,
        }
    }
}

impl SecuritySettings {
    pub fn mode(&self) -> SecurityMode {
        self.mode
    }

    pub fn set_mode(&mut self, value: SecurityMode) {
        self.mode = value;

        match value {
            SecurityMode::Restricted => {
                self.allow_external_commands = false;
                self.allow_plugin_network = false;
                self.allow_plugin_filesystem = false;
                self.allow_plugin_processes = false;
            }
            SecurityMode::Safe => {
                self.allow_clipboard_read = false;
                self.allow_clipboard_write = false;
                self.allow_hyperlinks = false;
                self.allow_external_commands = false;
                self.allow_network_access = false;
                self.allow_plugins = false;
            }
            SecurityMode::Normal | SecurityMode::Custom => {}
        }
    }

    pub fn confirmation_mode(&self) -> ConfirmationMode {
        self.confirmation_mode
    }

    pub fn set_confirmation_mode(&mut self, value: ConfirmationMode) {
        self.confirmation_mode = value;
    }

    pub fn allow_clipboard_read(&self) -> bool {
        self.allow_clipboard_read
    }

    pub fn set_allow_clipboard_read(&mut self, value: bool) {
        self.allow_clipboard_read = value;
    }

    pub fn allow_clipboard_write(&self) -> bool {
        self.allow_clipboard_write
    }

    pub fn set_allow_clipboard_write(&mut self, value: bool) {
        self.allow_clipboard_write = value;
    }

    pub fn allow_hyperlinks(&self) -> bool {
        self.allow_hyperlinks
    }

    pub fn set_allow_hyperlinks(&mut self, value: bool) {
        self.allow_hyperlinks = value;
    }

    pub fn allow_external_commands(&self) -> bool {
        self.allow_external_commands
    }

    pub fn set_allow_external_commands(&mut self, value: bool) {
        self.allow_external_commands = value;
    }

    pub fn allow_file_access(&self) -> bool {
        self.allow_file_access
    }

    pub fn set_allow_file_access(&mut self, value: bool) {
        self.allow_file_access = value;
    }

    pub fn allow_network_access(&self) -> bool {
        self.allow_network_access
    }

    pub fn set_allow_network_access(&mut self, value: bool) {
        self.allow_network_access = value;
    }

    pub fn allow_plugins(&self) -> bool {
        self.allow_plugins
    }

    pub fn set_allow_plugins(&mut self, value: bool) {
        self.allow_plugins = value;
    }

    pub fn allow_plugin_network(&self) -> bool {
        self.allow_plugin_network
    }

    pub fn set_allow_plugin_network(&mut self, value: bool) {
        self.allow_plugin_network = value;
    }

    pub fn allow_plugin_filesystem(&self) -> bool {
        self.allow_plugin_filesystem
    }

    pub fn set_allow_plugin_filesystem(&mut self, value: bool) {
        self.allow_plugin_filesystem = value;
    }

    pub fn allow_plugin_processes(&self) -> bool {
        self.allow_plugin_processes
    }

    pub fn set_allow_plugin_processes(&mut self, value: bool) {
        self.allow_plugin_processes = value;
    }

    pub fn verify_plugin_signatures(&self) -> bool {
        self.verify_plugin_signatures
    }

    pub fn set_verify_plugin_signatures(&mut self, value: bool) {
        self.verify_plugin_signatures = value;
    }

    pub fn require_plugin_permissions(&self) -> bool {
        self.require_plugin_permissions
    }

    pub fn set_require_plugin_permissions(&mut self, value: bool) {
        self.require_plugin_permissions = value;
    }

    pub fn warn_untrusted_output(&self) -> bool {
        self.warn_untrusted_output
    }

    pub fn set_warn_untrusted_output(&mut self, value: bool) {
        self.warn_untrusted_output = value;
    }

    pub fn warn_external_links(&self) -> bool {
        self.warn_external_links
    }

    pub fn set_warn_external_links(&mut self, value: bool) {
        self.warn_external_links = value;
    }

    pub fn warn_paste_commands(&self) -> bool {
        self.warn_paste_commands
    }

    pub fn set_warn_paste_commands(&mut self, value: bool) {
        self.warn_paste_commands = value;
    }

    pub fn warn_sensitive_operations(&self) -> bool {
        self.warn_sensitive_operations
    }

    pub fn set_warn_sensitive_operations(&mut self, value: bool) {
        self.warn_sensitive_operations = value;
    }

    pub fn isolate_plugins(&self) -> bool {
        self.isolate_plugins
    }

    pub fn set_isolate_plugins(&mut self, value: bool) {
        self.isolate_plugins = value;
    }

    pub fn safe_mode_on_error(&self) -> bool {
        self.safe_mode_on_error
    }

    pub fn set_safe_mode_on_error(&mut self, value: bool) {
        self.safe_mode_on_error = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "mode".into(),
            ConfigValue::String(format!("{:?}", self.mode).to_lowercase()),
        );
        values.insert(
            "confirmation_mode".into(),
            ConfigValue::String(format!("{:?}", self.confirmation_mode).to_lowercase()),
        );
        values.insert(
            "allow_clipboard_read".into(),
            ConfigValue::Boolean(self.allow_clipboard_read),
        );
        values.insert(
            "allow_clipboard_write".into(),
            ConfigValue::Boolean(self.allow_clipboard_write),
        );
        values.insert(
            "allow_hyperlinks".into(),
            ConfigValue::Boolean(self.allow_hyperlinks),
        );
        values.insert(
            "allow_external_commands".into(),
            ConfigValue::Boolean(self.allow_external_commands),
        );
        values.insert(
            "allow_file_access".into(),
            ConfigValue::Boolean(self.allow_file_access),
        );
        values.insert(
            "allow_network_access".into(),
            ConfigValue::Boolean(self.allow_network_access),
        );
        values.insert(
            "allow_plugins".into(),
            ConfigValue::Boolean(self.allow_plugins),
        );
        values.insert(
            "allow_plugin_network".into(),
            ConfigValue::Boolean(self.allow_plugin_network),
        );
        values.insert(
            "allow_plugin_filesystem".into(),
            ConfigValue::Boolean(self.allow_plugin_filesystem),
        );
        values.insert(
            "allow_plugin_processes".into(),
            ConfigValue::Boolean(self.allow_plugin_processes),
        );
        values.insert(
            "verify_plugin_signatures".into(),
            ConfigValue::Boolean(self.verify_plugin_signatures),
        );
        values.insert(
            "require_plugin_permissions".into(),
            ConfigValue::Boolean(self.require_plugin_permissions),
        );
        values.insert(
            "warn_untrusted_output".into(),
            ConfigValue::Boolean(self.warn_untrusted_output),
        );
        values.insert(
            "warn_external_links".into(),
            ConfigValue::Boolean(self.warn_external_links),
        );
        values.insert(
            "warn_paste_commands".into(),
            ConfigValue::Boolean(self.warn_paste_commands),
        );
        values.insert(
            "warn_sensitive_operations".into(),
            ConfigValue::Boolean(self.warn_sensitive_operations),
        );
        values.insert(
            "isolate_plugins".into(),
            ConfigValue::Boolean(self.isolate_plugins),
        );
        values.insert(
            "safe_mode_on_error".into(),
            ConfigValue::Boolean(self.safe_mode_on_error),
        );

        values
    }
}
