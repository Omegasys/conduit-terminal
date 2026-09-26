use std::process::{Child, Command, Stdio};

use super::{
    ShellConfiguration, ShellIntegration, ShellManifest, ShellResult,
};

/// Result of launching a shell process.
#[derive(Debug)]
pub struct ShellProcess {
    child: Child,
}

impl ShellProcess {
    pub fn new(child: Child) -> Self {
        Self { child }
    }

    pub fn id(&self) -> u32 {
        self.child.id()
    }

    pub fn child(&mut self) -> &mut Child {
        &mut self.child
    }

    pub fn into_child(self) -> Child {
        self.child
    }

    pub fn try_wait(&mut self) -> std::io::Result<Option<std::process::ExitStatus>> {
        self.child.try_wait()
    }

    pub fn wait(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.child.wait()
    }

    pub fn kill(&mut self) -> std::io::Result<()> {
        self.child.kill()
    }
}

/// Builds and launches shell processes.
///
/// PTY attachment is intentionally left to Conduit's PTY subsystem.
#[derive(Debug, Default, Clone, Copy)]
pub struct ShellLauncher;

impl ShellLauncher {
    pub fn new() -> Self {
        Self
    }

    /// Builds a command from a shell integration.
    pub fn build(&self, integration: &ShellIntegration) -> ShellResult<Command> {
        integration.launch_command()
    }

    /// Launches a shell using normal stdio.
    ///
    /// A PTY-aware launcher should configure stdin/stdout/stderr separately
    /// before spawning the process.
    pub fn spawn(&self, integration: &ShellIntegration) -> ShellResult<ShellProcess> {
        let mut command = self.build(integration)?;

        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let child = command.spawn()?;

        Ok(ShellProcess::new(child))
    }

    /// Builds a command directly from a manifest and configuration.
    pub fn build_from(
        &self,
        manifest: &ShellManifest,
        configuration: &ShellConfiguration,
    ) -> ShellResult<Command> {
        let executable = if let Some(executable) = &configuration.executable {
            executable.clone()
        } else {
            manifest.executable.clone()
        };

        let mut command = Command::new(executable);

        for argument in configuration.launch_arguments() {
            command.arg(argument);
        }

        for argument in &manifest.startup_arguments {
            command.arg(argument);
        }

        if let Some(directory) = &configuration.working_directory {
            command.current_dir(directory);
        }

        if configuration.inherit_environment {
            for (key, value) in &manifest.environment {
                command.env(key, value);
            }
        } else {
            command.env_clear();

            for (key, value) in &manifest.environment {
                command.env(key, value);
            }
        }

        for (key, value) in &configuration.environment {
            command.env(key, value);
        }

        Ok(command)
    }
}
