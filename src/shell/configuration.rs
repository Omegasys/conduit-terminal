//! Shell launch and integration configuration.

use std::path::PathBuf;

/// How Conduit launches a shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellLaunchMode {
    Interactive,
    Login,
    InteractiveLogin,
    Command,
}

impl Default for ShellLaunchMode {
    fn default() -> Self {
        Self::Interactive
    }
}

/// Per-shell configuration.
#[derive(Debug, Clone)]
pub struct ShellConfiguration {
    pub executable: Option<PathBuf>,
    pub mode: ShellLaunchMode,
    pub arguments: Vec<String>,
    pub environment: Vec<(String, String)>,
    pub working_directory: Option<PathBuf>,
    pub inherit_environment: bool,
    pub integrate_prompt: bool,
    pub integrate_directory: bool,
    pub integrate_command_status: bool,
    pub detect_shell: bool,
}

impl Default for ShellConfiguration {
    fn default() -> Self {
        Self {
            executable: None,
            mode: ShellLaunchMode::Interactive,
            arguments: Vec::new(),
            environment: Vec::new(),
            working_directory: None,
            inherit_environment: true,
            integrate_prompt: true,
            integrate_directory: true,
            integrate_command_status: true,
            detect_shell: true,
        }
    }
}

impl ShellConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_executable(
        mut self,
        executable: impl Into<PathBuf>,
    ) -> Self {
        self.executable = Some(executable.into());
        self
    }

    pub fn with_mode(mut self, mode: ShellLaunchMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_argument(mut self, argument: impl Into<String>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    pub fn with_working_directory(
        mut self,
        directory: impl Into<PathBuf>,
    ) -> Self {
        self.working_directory = Some(directory.into());
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

    pub fn disable_integrations(&mut self) {
        self.integrate_prompt = false;
        self.integrate_directory = false;
        self.integrate_command_status = false;
    }

    pub fn launch_arguments(&self) -> Vec<String> {
        let mut arguments = self.arguments.clone();

        match self.mode {
            ShellLaunchMode::Interactive => {
                if !arguments.iter().any(|arg| arg == "-i") {
                    arguments.push("-i".into());
                }
            }
            ShellLaunchMode::Login => {
                if !arguments.iter().any(|arg| arg == "-l") {
                    arguments.push("-l".into());
                }
            }
            ShellLaunchMode::InteractiveLogin => {
                if !arguments.iter().any(|arg| arg == "-i") {
                    arguments.push("-i".into());
                }

                if !arguments.iter().any(|arg| arg == "-l") {
                    arguments.push("-l".into());
                }
            }
            ShellLaunchMode::Command => {}
        }

        arguments
    }
}
