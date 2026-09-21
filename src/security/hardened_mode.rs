use super::permissions::{
    SecurityPermission,
    SecurityPermissionSet,
};
use super::sandbox::{SandboxMode, SandboxPolicy};

/// Strong security configuration intended to minimize terminal-originated
/// capabilities while retaining basic interactive terminal functionality.
#[derive(Debug, Clone)]
pub struct HardenedMode {
    sandbox: SandboxPolicy,
    permissions: SecurityPermissionSet,
}

impl HardenedMode {
    pub fn new() -> Self {
        let mut sandbox = SandboxPolicy::new();

        sandbox.set_mode(SandboxMode::Strict);

        let mut permissions = SecurityPermissionSet::new();

        permissions.allow(SecurityPermission::ClipboardRead);

        Self {
            sandbox,
            permissions,
        }
    }

    pub fn sandbox(&self) -> &SandboxPolicy {
        &self.sandbox
    }

    pub fn sandbox_mut(&mut self) -> &mut SandboxPolicy {
        &mut self.sandbox
    }

    pub fn permissions(&self) -> &SecurityPermissionSet {
        &self.permissions
    }

    pub fn permissions_mut(&mut self) -> &mut SecurityPermissionSet {
        &mut self.permissions
    }

    pub fn is_allowed(
        &self,
        permission: SecurityPermission,
    ) -> bool {
        self.permissions.allows(permission)
            && self.sandbox.can(permission).is_ok()
    }

    pub fn apply(
        &self,
        policy: &mut SandboxPolicy,
    ) {
        *policy = self.sandbox.clone();
    }
}

impl Default for HardenedMode {
    fn default() -> Self {
        Self::new()
    }
}
