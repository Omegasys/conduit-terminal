//! Custom shell launch configuration.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::errors::{CustomShellError, CustomShellResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomLaunchMode {
    Interactive,
    Login,
    InteractiveLogin,
    Command,
}

impl Default for CustomLaunchMode {
    fn default() -> Self {
        Self::Interactive
    }
}

#[derive(Debug, Clone)]
pub struct CustomShellConfiguration {
    mode: CustomLaunchMode,
    arguments: Vec<String>,
    environment: BTreeMap<String, String>,
    working_directory: Option<PathBuf>,
    inherit_environment: bool,
}

impl Default for CustomShellConfiguration {
    fn default() -> Self {
        Self {
            mode: CustomLaunchMode::Interactive,
            arguments: Vec::new(),
            environment: BTreeMap::new(),
            working_directory: None,
            inherit_environment: true,
        }
    }
}

impl CustomShellConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mode(&self) -> CustomLaunchMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: CustomLaunchMode) {
        self.mode = mode;
    }

    pub fn with_argument(mut self, argument: impl Into<String>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn set_environment(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.environment.insert(key.into(), value.into());
    }

    pub fn environment(&self) -> &BTreeMap<String, String> {
        &self.environment
    }

    pub fn with_working_directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.working_directory = Some(path.into());
        self
    }

    pub fn working_directory(&self) -> Option<&Path> {
        self.working_directory.as_deref()
    }

    pub fn set_inherit_environment(&mut self, inherit: bool) {
        self.inherit_environment = inherit;
    }

    pub fn inherit_environment(&self) -> bool {
        self.inherit_environment
    }

    pub fn validate(&self) -> CustomShellResult<()> {
        if let Some(path) = &self.working_directory {
            if !path.exists() {
                return Err(CustomShellError::InvalidConfiguration(format!(
                    "working directory does not exist: {}",
                    path.display()
                )));
            }

            if !path.is_dir() {
                return Err(CustomShellError::InvalidConfiguration(format!(
                    "working directory is not a directory: {}",
                    path.display()
                )));
            }
        }

        Ok(())
    }
}
