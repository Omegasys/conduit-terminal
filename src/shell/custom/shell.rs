//! Runtime representation of a custom shell.

use std::path::PathBuf;

use super::{
    capabilities::CustomCapabilities,
    configuration::CustomShellConfiguration,
    environment::CustomEnvironment,
    errors::{CustomShellError, CustomShellResult},
    manifest::CustomShellManifest,
    prompt::CustomPromptIntegration,
    security::CustomShellSecurityPolicy,
};

/// A user-defined shell known to Conduit.
#[derive(Debug, Clone)]
pub struct CustomShell {
    manifest: CustomShellManifest,
    configuration: CustomShellConfiguration,
    environment: CustomEnvironment,
    prompt: CustomPromptIntegration,
    security: CustomShellSecurityPolicy,
}

impl CustomShell {
    /// Create a custom shell from a manifest.
    pub fn new(manifest: CustomShellManifest) -> CustomShellResult<Self> {
        manifest.validate()?;

        Ok(Self {
            manifest,
            configuration: CustomShellConfiguration::default(),
            environment: CustomEnvironment::default(),
            prompt: CustomPromptIntegration::default(),
            security: CustomShellSecurityPolicy::default(),
        })
    }

    pub fn manifest(&self) -> &CustomShellManifest {
        &self.manifest
    }

    pub fn configuration(&self) -> &CustomShellConfiguration {
        &self.configuration
    }

    pub fn configuration_mut(&mut self) -> &mut CustomShellConfiguration {
        &mut self.configuration
    }

    pub fn environment(&self) -> &CustomEnvironment {
        &self.environment
    }

    pub fn environment_mut(&mut self) -> &mut CustomEnvironment {
        &mut self.environment
    }

    pub fn prompt(&self) -> &CustomPromptIntegration {
        &self.prompt
    }

    pub fn prompt_mut(&mut self) -> &mut CustomPromptIntegration {
        &mut self.prompt
    }

    pub fn security(&self) -> &CustomShellSecurityPolicy {
        &self.security
    }

    pub fn security_mut(&mut self) -> &mut CustomShellSecurityPolicy {
        &mut self.security
    }

    pub fn id(&self) -> &str {
        self.manifest.id()
    }

    pub fn name(&self) -> &str {
        self.manifest.name()
    }

    pub fn executable(&self) -> &str {
        self.manifest.executable()
    }

    pub fn capabilities(&self) -> &CustomCapabilities {
        self.manifest.capabilities()
    }

    /// Build the command used to launch this shell.
    pub fn launch_command(&self) -> CustomShellResult<std::process::Command> {
        let executable = if self.executable().contains('/') {
            PathBuf::from(self.executable())
        } else {
            PathBuf::from(self.executable())
        };

        let mut command = std::process::Command::new(executable);

        for argument in self.configuration.arguments() {
            command.arg(argument);
        }

        for argument in self.manifest.arguments() {
            command.arg(argument);
        }

        if let Some(directory) = self.configuration.working_directory() {
            command.current_dir(directory);
        }

        if self.environment.inherit_parent() {
            command.envs(std::env::vars());
        }

        for (key, value) in self.environment.variables() {
            command.env(key, value);
        }

        Ok(command)
    }

    /// Validate the complete runtime configuration.
    pub fn validate(&self) -> CustomShellResult<()> {
        self.manifest.validate()?;
        self.configuration.validate()?;
        self.prompt.validate()?;
        self.security.validate()?;
        Ok(())
    }

    /// Replace the shell's security policy.
    pub fn set_security_policy(&mut self, policy: CustomShellSecurityPolicy) {
        self.security = policy;
    }
}
