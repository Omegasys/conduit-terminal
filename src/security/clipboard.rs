use super::permissions::SecurityPermission;
use super::sandbox::{SandboxPolicy, SandboxViolation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardSecurityResult {
    Allowed,
    RequiresConfirmation,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct ClipboardSecurityPolicy {
    allow_read: bool,
    allow_write: bool,
    allow_sensitive_read: bool,
    allow_sensitive_write: bool,
    require_confirmation: bool,
}

impl Default for ClipboardSecurityPolicy {
    fn default() -> Self {
        Self {
            allow_read: true,
            allow_write: true,
            allow_sensitive_read: false,
            allow_sensitive_write: false,
            require_confirmation: false,
        }
    }
}

impl ClipboardSecurityPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_read(&self) -> bool {
        self.allow_read
    }

    pub fn allow_write(&self) -> bool {
        self.allow_write
    }

    pub fn allow_sensitive_read(&self) -> bool {
        self.allow_sensitive_read
    }

    pub fn allow_sensitive_write(&self) -> bool {
        self.allow_sensitive_write
    }

    pub fn require_confirmation(&self) -> bool {
        self.require_confirmation
    }

    pub fn set_allow_read(&mut self, value: bool) {
        self.allow_read = value;
    }

    pub fn set_allow_write(&mut self, value: bool) {
        self.allow_write = value;
    }

    pub fn set_allow_sensitive_read(&mut self, value: bool) {
        self.allow_sensitive_read = value;
    }

    pub fn set_allow_sensitive_write(&mut self, value: bool) {
        self.allow_sensitive_write = value;
    }

    pub fn set_require_confirmation(&mut self, value: bool) {
        self.require_confirmation = value;
    }
}

#[derive(Debug, Clone)]
pub struct ClipboardSecurity {
    policy: ClipboardSecurityPolicy,
}

impl ClipboardSecurity {
    pub fn new(policy: ClipboardSecurityPolicy) -> Self {
        Self { policy }
    }

    pub fn policy(&self) -> &ClipboardSecurityPolicy {
        &self.policy
    }

    pub fn policy_mut(&mut self) -> &mut ClipboardSecurityPolicy {
        &mut self.policy
    }

    pub fn check_read(
        &self,
        sensitive: bool,
        sandbox: &SandboxPolicy,
    ) -> Result<ClipboardSecurityResult, SandboxViolation> {
        sandbox.can(SecurityPermission::ClipboardRead)?;

        if sensitive && !self.policy.allow_sensitive_read {
            return Ok(ClipboardSecurityResult::Blocked);
        }

        if self.policy.require_confirmation {
            return Ok(ClipboardSecurityResult::RequiresConfirmation);
        }

        Ok(ClipboardSecurityResult::Allowed)
    }

    pub fn check_write(
        &self,
        sensitive: bool,
        sandbox: &SandboxPolicy,
    ) -> Result<ClipboardSecurityResult, SandboxViolation> {
        sandbox.can(SecurityPermission::ClipboardWrite)?;

        if !self.policy.allow_write {
            return Ok(ClipboardSecurityResult::Blocked);
        }

        if sensitive && !self.policy.allow_sensitive_write {
            return Ok(ClipboardSecurityResult::Blocked);
        }

        if self.policy.require_confirmation || sensitive {
            return Ok(ClipboardSecurityResult::RequiresConfirmation);
        }

        Ok(ClipboardSecurityResult::Allowed)
    }
}

impl Default for ClipboardSecurity {
    fn default() -> Self {
        Self::new(ClipboardSecurityPolicy::default())
    }
}
