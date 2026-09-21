use std::path::{Path, PathBuf};

use super::permissions::SecurityPermission;
use super::sandbox::{SandboxPolicy, SandboxViolation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemAction {
    Read,
    Write,
    Create,
    Delete,
    Execute,
}

#[derive(Debug, Clone)]
pub struct FilesystemPolicy {
    allow_read: bool,
    allow_write: bool,
    allow_create: bool,
    allow_delete: bool,
    allow_execute: bool,
    allowed_read_paths: Vec<PathBuf>,
    allowed_write_paths: Vec<PathBuf>,
}

impl Default for FilesystemPolicy {
    fn default() -> Self {
        Self {
            allow_read: true,
            allow_write: false,
            allow_create: false,
            allow_delete: false,
            allow_execute: false,
            allowed_read_paths: Vec::new(),
            allowed_write_paths: Vec::new(),
        }
    }
}

impl FilesystemPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_allow_read(&mut self, value: bool) {
        self.allow_read = value;
    }

    pub fn set_allow_write(&mut self, value: bool) {
        self.allow_write = value;
    }

    pub fn set_allow_create(&mut self, value: bool) {
        self.allow_create = value;
    }

    pub fn set_allow_delete(&mut self, value: bool) {
        self.allow_delete = value;
    }

    pub fn set_allow_execute(&mut self, value: bool) {
        self.allow_execute = value;
    }

    pub fn allow_read_path<P: Into<PathBuf>>(&mut self, path: P) {
        self.allowed_read_paths.push(path.into());
    }

    pub fn allow_write_path<P: Into<PathBuf>>(&mut self, path: P) {
        self.allowed_write_paths.push(path.into());
    }

    pub fn can_read(
        &self,
        path: &Path,
        sandbox: &SandboxPolicy,
    ) -> Result<(), SandboxViolation> {
        if !self.allow_read {
            return Err(
                SandboxViolation::new(
                    SecurityPermission::OpenFile,
                    "filesystem reads are disabled",
                )
                .with_resource(path.display().to_string()),
            );
        }

        sandbox.can(SecurityPermission::OpenFile)?;

        if self.allowed_read_paths.is_empty() {
            return sandbox.can_read_path(path);
        }

        if self
            .allowed_read_paths
            .iter()
            .any(|allowed| path.starts_with(allowed))
        {
            Ok(())
        } else {
            Err(
                SandboxViolation::new(
                    SecurityPermission::OpenFile,
                    "path is outside the allowed read paths",
                )
                .with_resource(path.display().to_string()),
            )
        }
    }

    pub fn can_write(
        &self,
        path: &Path,
        sandbox: &SandboxPolicy,
    ) -> Result<(), SandboxViolation> {
        if !self.allow_write {
            return Err(
                SandboxViolation::new(
                    SecurityPermission::OpenFile,
                    "filesystem writes are disabled",
                )
                .with_resource(path.display().to_string()),
            );
        }

        sandbox.can(SecurityPermission::OpenFile)?;

        if self.allowed_write_paths.is_empty() {
            return sandbox.can_write_path(path);
        }

        if self
            .allowed_write_paths
            .iter()
            .any(|allowed| path.starts_with(allowed))
        {
            Ok(())
        } else {
            Err(
                SandboxViolation::new(
                    SecurityPermission::OpenFile,
                    "path is outside the allowed write paths",
                )
                .with_resource(path.display().to_string()),
            )
        }
    }

    pub fn can_create(
        &self,
        path: &Path,
        sandbox: &SandboxPolicy,
    ) -> Result<(), SandboxViolation> {
        if !self.allow_create {
            return Err(
                SandboxViolation::new(
                    SecurityPermission::OpenFile,
                    "filesystem creation is disabled",
                )
                .with_resource(path.display().to_string()),
            );
        }

        self.can_write(path, sandbox)
    }

    pub fn can_delete(
        &self,
        path: &Path,
        sandbox: &SandboxPolicy,
    ) -> Result<(), SandboxViolation> {
        if !self.allow_delete {
            return Err(
                SandboxViolation::new(
                    SecurityPermission::OpenFile,
                    "filesystem deletion is disabled",
                )
                .with_resource(path.display().to_string()),
            );
        }

        self.can_write(path, sandbox)
    }

    pub fn can_execute(
        &self,
        path: &Path,
        sandbox: &SandboxPolicy,
    ) -> Result<(), SandboxViolation> {
        if !self.allow_execute {
            return Err(
                SandboxViolation::new(
                    SecurityPermission::ExecuteCommands,
                    "filesystem execution is disabled",
                )
                .with_resource(path.display().to_string()),
            );
        }

        sandbox.can(SecurityPermission::ExecuteCommands)?;

        Ok(())
    }

    pub fn allowed_read_paths(&self) -> &[PathBuf] {
        &self.allowed_read_paths
    }

    pub fn allowed_write_paths(&self) -> &[PathBuf] {
        &self.allowed_write_paths
    }
}
