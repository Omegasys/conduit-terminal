use super::permissions::{
    SecurityPermission,
    SecurityPermissionSet,
};
use super::sandbox::{SandboxMode, SandboxPolicy};

/// Conservative configuration intended for untrusted terminal content,
/// recovery sessions, or troubleshooting.
#[derive(Debug, Clone)]
pub struct SafeMode {
    sandbox: SandboxPolicy,
    permissions: SecurityPermissionSet,
}

impl SafeMode {
    pub fn new() -> Self {
        let mut sandbox = SandboxPolicy::new();

        sandbox.set_mode(SandboxMode::Strict);

        let mut permissions = SecurityPermissionSet::new();

        permissions.allow(SecurityPermission::ClipboardRead);
        permissions.allow(SecurityPermission::OpenHyperlink);

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

impl Default for SafeMode {
    fn default() -> Self {
        Self::new()
    }
}
