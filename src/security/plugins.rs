use super::permissions::SecurityPermission;
use super::sandbox::{SandboxPolicy, SandboxViolation};

/// Security policy for plugin-originated operations.
#[derive(Debug, Clone)]
pub struct PluginSecurityPolicy {
    allow_plugins: bool,
    allow_native_plugins: bool,
    allow_wasm_plugins: bool,
    allow_filesystem: bool,
    allow_network: bool,
    allow_process_execution: bool,
    allow_environment: bool,
    require_confirmation: bool,
}

impl Default for PluginSecurityPolicy {
    fn default() -> Self {
        Self {
            allow_plugins: true,
            allow_native_plugins: false,
            allow_wasm_plugins: true,
            allow_filesystem: false,
            allow_network: false,
            allow_process_execution: false,
            allow_environment: false,
            require_confirmation: true,
        }
    }
}

impl PluginSecurityPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_plugins(&self) -> bool {
        self.allow_plugins
    }

    pub fn allow_native_plugins(&self) -> bool {
        self.allow_native_plugins
    }

    pub fn allow_wasm_plugins(&self) -> bool {
        self.allow_wasm_plugins
    }

    pub fn allow_filesystem(&self) -> bool {
        self.allow_filesystem
    }

    pub fn allow_network(&self) -> bool {
        self.allow_network
    }

    pub fn allow_process_execution(&self) -> bool {
        self.allow_process_execution
    }

    pub fn allow_environment(&self) -> bool {
        self.allow_environment
    }

    pub fn require_confirmation(&self) -> bool {
        self.require_confirmation
    }

    pub fn set_allow_plugins(&mut self, value: bool) {
        self.allow_plugins = value;
    }

    pub fn set_allow_native_plugins(&mut self, value: bool) {
        self.allow_native_plugins = value;
    }

    pub fn set_allow_wasm_plugins(&mut self, value: bool) {
        self.allow_wasm_plugins = value;
    }

    pub fn set_allow_filesystem(&mut self, value: bool) {
        self.allow_filesystem = value;
    }

    pub fn set_allow_network(&mut self, value: bool) {
        self.allow_network = value;
    }

    pub fn set_allow_process_execution(&mut self, value: bool) {
        self.allow_process_execution = value;
    }

    pub fn set_allow_environment(&mut self, value: bool) {
        self.allow_environment = value;
    }

    pub fn set_require_confirmation(&mut self, value: bool) {
        self.require_confirmation = value;
    }

    pub fn check(
        &self,
        permission: SecurityPermission,
        sandbox: &SandboxPolicy,
    ) -> Result<bool, SandboxViolation> {
        if !self.allow_plugins {
            return Ok(false);
        }

        sandbox.can(permission)?;

        let allowed = match permission {
            SecurityPermission::ReadFilesystem
            | SecurityPermission::OpenFile
            | SecurityPermission::OpenDirectory => self.allow_filesystem,

            SecurityPermission::SystemIntegration => {
                self.allow_native_plugins
            }

            SecurityPermission::ExecuteCommands
            | SecurityPermission::SpawnProcess => {
                self.allow_process_execution
            }

            SecurityPermission::AccessEnvironment => {
                self.allow_environment
            }

            SecurityPermission::OpenHyperlink => {
                self.allow_network
            }

            _ => true,
        };

        Ok(allowed)
    }
}
