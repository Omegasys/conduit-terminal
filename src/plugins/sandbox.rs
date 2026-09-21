use std::path::PathBuf;

use super::permissions::{Permission, PermissionSet};

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub enabled: bool,
    pub filesystem_read: Vec<PathBuf>,
    pub filesystem_write: Vec<PathBuf>,
    pub network: bool,
    pub process_execution: bool,
    pub environment_access: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            filesystem_read: Vec::new(),
            filesystem_write: Vec::new(),
            network: false,
            process_execution: false,
            environment_access: false,
        }
    }
}

impl SandboxConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_read_path<P: Into<PathBuf>>(
        &mut self,
        path: P,
    ) {
        self.filesystem_read.push(path.into());
    }

    pub fn allow_write_path<P: Into<PathBuf>>(
        &mut self,
        path: P,
    ) {
        self.filesystem_write.push(path.into());
    }
}

#[derive(Debug, Clone)]
pub struct SandboxPolicy {
    config: SandboxConfig,
}

impl SandboxPolicy {
    pub fn new(config: SandboxConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut SandboxConfig {
        &mut self.config
    }

    pub fn allows(
        &self,
        permission: Permission,
    ) -> bool {
        if !self.config.enabled {
            return true;
        }

        match permission {
            Permission::Network | Permission::LocalNetwork => {
                self.config.network
            }

            Permission::SpawnProcess | Permission::ExecuteCommands => {
                self.config.process_execution
            }

            Permission::AccessEnvironment => {
                self.config.environment_access
            }

            _ => true,
        }
    }

    pub fn filter_permissions(
        &self,
        requested: &PermissionSet,
    ) -> PermissionSet {
        PermissionSet::from_permissions(
            requested
                .all()
                .iter()
                .copied()
                .filter(|permission| self.allows(*permission)),
        )
    }

    pub fn allows_read_path(
        &self,
        path: &std::path::Path,
    ) -> bool {
        if !self.config.enabled {
            return true;
        }

        self.config
            .filesystem_read
            .iter()
            .any(|allowed| path.starts_with(allowed))
    }

    pub fn allows_write_path(
        &self,
        path: &std::path::Path,
    ) -> bool {
        if !self.config.enabled {
            return true;
        }

        self.config
            .filesystem_write
            .iter()
            .any(|allowed| path.starts_with(allowed))
    }
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self::new(SandboxConfig::default())
    }
}
