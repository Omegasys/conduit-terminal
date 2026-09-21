use super::clipboard::{
    ClipboardSecurity,
    ClipboardSecurityPolicy,
};
use super::environment::EnvironmentPolicy;
use super::escape_sequences::{
    EscapeSequenceSecurity,
    EscapeSequenceSecurityPolicy,
};
use super::filesystem::FilesystemPolicy;
use super::hyperlinks::{
    HyperlinkSecurity,
    HyperlinkSecurityPolicy,
};
use super::permissions::{
    SecurityPermission,
    SecurityPermissionSet,
};
use super::plugins::PluginSecurityPolicy;
use super::sandbox::SandboxPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityProfile {
    Normal,
    Safe,
    Hardened,
}

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    profile: SecurityProfile,
    permissions: SecurityPermissionSet,
    sandbox: SandboxPolicy,

    pub clipboard: ClipboardSecurity,
    pub hyperlinks: HyperlinkSecurity,
    pub escape_sequences: EscapeSequenceSecurity,
    pub filesystem: FilesystemPolicy,
    pub environment: EnvironmentPolicy,
    pub plugins: PluginSecurityPolicy,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self::normal()
    }
}

impl SecurityPolicy {
    pub fn normal() -> Self {
        let mut permissions =
            SecurityPermissionSet::terminal_defaults();

        permissions.allow(SecurityPermission::ClipboardWrite);
        permissions.allow(SecurityPermission::ChangeWindowTitle);
        permissions.allow(SecurityPermission::SetTerminalProperty);

        Self {
            profile: SecurityProfile::Normal,
            permissions,
            sandbox: SandboxPolicy::default(),
            clipboard: ClipboardSecurity::new(
                ClipboardSecurityPolicy::default(),
            ),
            hyperlinks: HyperlinkSecurity::new(
                HyperlinkSecurityPolicy::default(),
            ),
            escape_sequences: EscapeSequenceSecurity::new(
                EscapeSequenceSecurityPolicy::default(),
            ),
            filesystem: FilesystemPolicy::default(),
            environment: EnvironmentPolicy::default(),
            plugins: PluginSecurityPolicy::default(),
        }
    }

    pub fn safe() -> Self {
        let mut policy = Self::normal();

        policy.profile = SecurityProfile::Safe;

        policy.permissions =
            SecurityPermissionSet::terminal_defaults();

        policy.clipboard
            .policy_mut()
            .set_allow_sensitive_read(false);

        policy.clipboard
            .policy_mut()
            .set_allow_sensitive_write(false);

        policy.escape_sequences
            .policy_mut()
            .set_allow_clipboard(false);

        policy.escape_sequences
            .policy_mut()
            .set_allow_shell_commands(false);

        policy.escape_sequences
            .policy_mut()
            .set_allow_file_operations(false);

        policy.plugins.set_allow_native_plugins(false);
        policy.plugins.set_allow_process_execution(false);
        policy.plugins.set_allow_network(false);

        policy
    }

    pub fn hardened() -> Self {
        let mut policy = Self::safe();

        policy.profile = SecurityProfile::Hardened;

        policy.permissions = SecurityPermissionSet::new();

        policy.clipboard
            .policy_mut()
            .set_allow_write(false);

        policy.clipboard
            .policy_mut()
            .set_allow_sensitive_read(false);

        policy.clipboard
            .policy_mut()
            .set_allow_sensitive_write(false);

        policy.hyperlinks
            .policy_mut()
            .set_allow_file(false);

        policy.hyperlinks
            .policy_mut()
            .set_allow_custom(false);

        policy.escape_sequences
            .policy_mut()
            .set_allow_clipboard(false);

        policy.escape_sequences
            .policy_mut()
            .set_allow_shell_commands(false);

        policy.escape_sequences
            .policy_mut()
            .set_allow_file_operations(false);

        policy.filesystem.set_allow_write(false);
        policy.filesystem.set_allow_create(false);
        policy.filesystem.set_allow_delete(false);
        policy.filesystem.set_allow_execute(false);

        policy.environment.set_allow_write(false);
        policy.environment.set_allow_path(false);

        policy.plugins.set_allow_plugins(false);
        policy.plugins.set_allow_native_plugins(false);
        policy.plugins.set_allow_network(false);
        policy.plugins.set_allow_process_execution(false);

        policy
    }

    pub fn profile(&self) -> SecurityProfile {
        self.profile
    }

    pub fn permissions(&self) -> &SecurityPermissionSet {
        &self.permissions
    }

    pub fn permissions_mut(&mut self) -> &mut SecurityPermissionSet {
        &mut self.permissions
    }

    pub fn sandbox(&self) -> &SandboxPolicy {
        &self.sandbox
    }

    pub fn sandbox_mut(&mut self) -> &mut SandboxPolicy {
        &mut self.sandbox
    }

    pub fn allows(
        &self,
        permission: SecurityPermission,
    ) -> bool {
        self.permissions.allows(permission)
            && self.sandbox.can(permission).is_ok()
    }

    pub fn set_profile(
        &mut self,
        profile: SecurityProfile,
    ) {
        *self = match profile {
            SecurityProfile::Normal => Self::normal(),
            SecurityProfile::Safe => Self::safe(),
            SecurityProfile::Hardened => Self::hardened(),
        };
    }
}
