//! Shell manifest definitions.
//!
//! A manifest allows built-in and custom shells to describe themselves to
//! Conduit without requiring the shell registry to know implementation
//! details.

use std::path::PathBuf;

use super::capabilities::ShellCapabilities;

/// Description of a shell understood by Conduit.
#[derive(Debug, Clone)]
pub struct ShellManifest {
    pub id: String,
    pub name: String,
    pub executable: PathBuf,
    pub version: Option<String>,
    pub description: Option<String>,
    pub capabilities: ShellCapabilities,
    pub arguments: Vec<String>,
    pub startup_arguments: Vec<String>,
    pub rc_file: Option<PathBuf>,
    pub environment: Vec<(String, String)>,
}

impl ShellManifest {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        executable: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            executable: executable.into(),
            version: None,
            description: None,
            capabilities: ShellCapabilities::default(),
            arguments: Vec::new(),
            startup_arguments: Vec::new(),
            rc_file: None,
            environment: Vec::new(),
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_argument(mut self, argument: impl Into<String>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    pub fn with_startup_argument(
        mut self,
        argument: impl Into<String>,
    ) -> Self {
        self.startup_arguments.push(argument.into());
        self
    }

    pub fn with_rc_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.rc_file = Some(path.into());
        self
    }

    pub fn set_environment(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        let key = key.into();

        if let Some(existing) =
            self.environment.iter_mut().find(|item| item.0 == key)
        {
            existing.1 = value.into();
        } else {
            self.environment.push((key, value.into()));
        }
    }

    pub fn validate(&self) -> Result<(), ShellManifestError> {
        if self.id.trim().is_empty() {
            return Err(ShellManifestError::Invalid(
                "shell id cannot be empty".into(),
            ));
        }

        if self.name.trim().is_empty() {
            return Err(ShellManifestError::Invalid(
                "shell name cannot be empty".into(),
            ));
        }

        if self.executable.as_os_str().is_empty() {
            return Err(ShellManifestError::Invalid(
                "shell executable cannot be empty".into(),
            ));
        }

        Ok(())
    }

    pub fn command_line(&self) -> Vec<String> {
        let mut command = Vec::with_capacity(
            1 + self.arguments.len() + self.startup_arguments.len(),
        );

        command.push(self.executable.to_string_lossy().into_owned());
        command.extend(self.arguments.iter().cloned());
        command.extend(self.startup_arguments.iter().cloned());

        command
    }
}

/// Errors produced while validating shell manifests.
#[derive(Debug, Clone)]
pub enum ShellManifestError {
    Invalid(String),
}

impl std::fmt::Display for ShellManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for ShellManifestError {}
