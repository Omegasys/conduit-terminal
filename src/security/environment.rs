use std::collections::HashSet;
use std::env;

use super::permissions::SecurityPermission;
use super::sandbox::{SandboxPolicy, SandboxViolation};

#[derive(Debug, Clone)]
pub struct EnvironmentPolicy {
    allow_read: bool,
    allow_write: bool,
    allow_path: bool,
    allowed_variables: HashSet<String>,
}

impl Default for EnvironmentPolicy {
    fn default() -> Self {
        Self {
            allow_read: true,
            allow_write: false,
            allow_path: false,
            allowed_variables: HashSet::new(),
        }
    }
}

impl EnvironmentPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_allow_read(&mut self, value: bool) {
        self.allow_read = value;
    }

    pub fn set_allow_write(&mut self, value: bool) {
        self.allow_write = value;
    }

    pub fn set_allow_path(&mut self, value: bool) {
        self.allow_path = value;
    }

    pub fn allow_variable<S: Into<String>>(&mut self, name: S) {
        self.allowed_variables.insert(name.into());
    }

    pub fn deny_variable(&mut self, name: &str) {
        self.allowed_variables.remove(name);
    }

    pub fn can_read(
        &self,
        name: &str,
        sandbox: &SandboxPolicy,
    ) -> Result<(), SandboxViolation> {
        if !self.allow_read {
            return Err(
                SandboxViolation::new(
                    SecurityPermission::AccessEnvironment,
                    "environment reads are disabled",
                )
                .with_resource(name),
            );
        }

        sandbox.can(SecurityPermission::AccessEnvironment)?;

        if self.allowed_variables.is_empty()
            || self.allowed_variables.contains(name)
        {
            Ok(())
        } else {
            Err(
                SandboxViolation::new(
                    SecurityPermission::AccessEnvironment,
                    "environment variable is not allowed",
                )
                .with_resource(name),
            )
        }
    }

    pub fn can_write(
        &self,
        name: &str,
        sandbox: &SandboxPolicy,
    ) -> Result<(), SandboxViolation> {
        if !self.allow_write {
            return Err(
                SandboxViolation::new(
                    SecurityPermission::AccessEnvironment,
                    "environment writes are disabled",
                )
                .with_resource(name),
            );
        }

        sandbox.can(SecurityPermission::AccessEnvironment)?;

        Ok(())
    }

    pub fn read(
        &self,
        name: &str,
        sandbox: &SandboxPolicy,
    ) -> Result<Option<String>, SandboxViolation> {
        self.can_read(name, sandbox)?;

        Ok(env::var(name).ok())
    }

    pub fn path_allowed(
        &self,
        sandbox: &SandboxPolicy,
    ) -> Result<(), SandboxViolation> {
        if !self.allow_path {
            return Err(
                SandboxViolation::new(
                    SecurityPermission::AccessEnvironment,
                    "PATH access is disabled",
                )
                .with_resource("PATH"),
            );
        }

        sandbox.can(SecurityPermission::AccessEnvironment)
    }

    pub fn allowed_variables(&self) -> impl Iterator<Item = &String> {
        self.allowed_variables.iter()
    }
}
