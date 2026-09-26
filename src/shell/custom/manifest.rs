//! Custom shell manifest definitions.

use super::{
    capabilities::CustomCapabilities,
    errors::{CustomShellError, CustomShellResult},
};

/// Declarative description of a custom shell.
#[derive(Debug, Clone)]
pub struct CustomShellManifest {
    id: String,
    name: String,
    executable: String,
    version: Option<String>,
    description: Option<String>,
    arguments: Vec<String>,
    startup_arguments: Vec<String>,
    rc_file: Option<String>,
    capabilities: CustomCapabilities,
}

impl CustomShellManifest {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        executable: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            executable: executable.into(),
            version: None,
            description: None,
            arguments: Vec::new(),
            startup_arguments: Vec::new(),
            rc_file: None,
            capabilities: CustomCapabilities::default(),
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_argument(mut self, argument: impl Into<String>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    pub fn with_startup_argument(mut self, argument: impl Into<String>) -> Self {
        self.startup_arguments.push(argument.into());
        self
    }

    pub fn with_rc_file(mut self, path: impl Into<String>) -> Self {
        self.rc_file = Some(path.into());
        self
    }

    pub fn with_capabilities(mut self, capabilities: CustomCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn executable(&self) -> &str {
        &self.executable
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn startup_arguments(&self) -> &[String] {
        &self.startup_arguments
    }

    pub fn rc_file(&self) -> Option<&str> {
        self.rc_file.as_deref()
    }

    pub fn capabilities(&self) -> &CustomCapabilities {
        &self.capabilities
    }

    pub fn validate(&self) -> CustomShellResult<()> {
        if self.id.trim().is_empty() {
            return Err(CustomShellError::InvalidManifest(
                "shell id cannot be empty".into(),
            ));
        }

        if self.name.trim().is_empty() {
            return Err(CustomShellError::InvalidManifest(
                "shell name cannot be empty".into(),
            ));
        }

        if self.executable.trim().is_empty() {
            return Err(CustomShellError::InvalidManifest(
                "shell executable cannot be empty".into(),
            ));
        }

        if self.id.contains(char::is_whitespace) {
            return Err(CustomShellError::InvalidManifest(
                "shell id cannot contain whitespace".into(),
            ));
        }

        Ok(())
    }
}
