use std::path::{Path, PathBuf};

use super::permissions::{
    SecurityPermission,
    SecurityPermissionSet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxMode {
    Disabled,
    Permissive,
    Restricted,
    Strict,
}

impl Default for SandboxMode {
    fn default() -> Self {
        Self::Restricted
    }
}

#[derive(Debug, Clone)]
pub struct SandboxViolation {
    pub permission: SecurityPermission,
    pub resource: Option<String>,
    pub reason: String,
}

impl SandboxViolation {
    pub fn new(
        permission: SecurityPermission,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            permission,
            resource: None,
            reason: reason.into(),
        }
    }

    pub fn with_resource(
        mut self,
        resource: impl Into<String>,
    ) -> Self {
        self.resource = Some(resource.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct SandboxPolicy {
    mode: SandboxMode,
    permissions: SecurityPermissionSet,
    readable_paths: Vec<PathBuf>,
    writable_paths: Vec<PathBuf>,
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl SandboxPolicy {
    pub fn new() -> Self {
        Self {
            mode: SandboxMode::Restricted,
            permissions: SecurityPermissionSet::terminal_defaults(),
            readable_paths: Vec::new(),
            writable_paths: Vec::new(),
        }
    }

    pub fn mode(&self) -> SandboxMode {
        self.mode
    }

    pub fn set_mode(
        &mut self,
        mode: SandboxMode,
    ) {
        self.mode = mode;
    }

    pub fn permissions(&self) -> &SecurityPermissionSet {
        &self.permissions
    }

    pub fn permissions_mut(&mut self) -> &mut SecurityPermissionSet {
        &mut self.permissions
    }

    pub fn allow_permission(
        &mut self,
        permission: SecurityPermission,
    ) {
        self.permissions.allow(permission);
    }

    pub fn deny_permission(
        &mut self,
        permission: SecurityPermission,
    ) {
        self.permissions.deny(permission);
    }

    pub fn allow_read_path<P: Into<PathBuf>>(
        &mut self,
        path: P,
    ) {
        self.readable_paths.push(path.into());
    }

    pub fn allow_write_path<P: Into<PathBuf>>(
        &mut self,
        path: P,
    ) {
        self.writable_paths.push(path.into());
    }

    pub fn can(
        &self,
        permission: SecurityPermission,
    ) -> Result<(), SandboxViolation> {
        if matches!(self.mode, SandboxMode::Disabled) {
            return Ok(());
        }

        if self.permissions.allows(permission) {
            return Ok(());
        }

        Err(
            SandboxViolation::new(
                permission,
                "operation is not permitted by the terminal sandbox",
            ),
        )
    }

    pub fn can_read_path(
        &self,
        path: &Path,
    ) -> Result<(), SandboxViolation> {
        if matches!(self.mode, SandboxMode::Disabled) {
            return Ok(());
        }

        if self
            .readable_paths
            .iter()
            .any(|allowed| path.starts_with(allowed))
        {
            return Ok(());
        }

        Err(
            SandboxViolation::new(
                SecurityPermission::OpenFile,
                "path is outside the sandbox read policy",
            )
            .with_resource(path.display().to_string()),
        )
    }

    pub fn can_write_path(
        &self,
        path: &Path,
    ) -> Result<(), SandboxViolation> {
        if matches!(self.mode, SandboxMode::Disabled) {
            return Ok(());
        }

        if self
            .writable_paths
            .iter()
            .any(|allowed| path.starts_with(allowed))
        {
            return Ok(());
        }

        Err(
            SandboxViolation::new(
                SecurityPermission::OpenFile,
                "path is outside the sandbox write policy",
            )
            .with_resource(path.display().to_string()),
        )
    }

    pub fn readable_paths(&self) -> &[PathBuf] {
        &self.readable_paths
    }

    pub fn writable_paths(&self) -> &[PathBuf] {
        &self.writable_paths
    }
}
