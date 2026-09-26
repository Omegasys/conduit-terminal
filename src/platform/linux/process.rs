use std::fmt;
use std::process::{Child, Command, ExitStatus, Stdio};

/// A Linux process managed by Conduit.
#[derive(Debug)]
pub struct LinuxProcess {
    child: Child,
    executable: String,
}

impl LinuxProcess {
    pub fn id(&self) -> u32 {
        self.child.id()
    }

    pub fn executable(&self) -> &str {
        &self.executable
    }

    pub fn try_wait(
        &mut self,
    ) -> Result<Option<ExitStatus>, LinuxProcessError> {
        Ok(self.child.try_wait()?)
    }

    pub fn wait(&mut self) -> Result<ExitStatus, LinuxProcessError> {
        Ok(self.child.wait()?)
    }

    pub fn kill(&mut self) -> Result<(), LinuxProcessError> {
        Ok(self.child.kill()?)
    }

    pub fn into_child(self) -> Child {
        self.child
    }
}

/// Linux process manager.
///
/// This deliberately provides a small abstraction around `std::process`.
/// More advanced process-group, signal, namespace, cgroup, and sandbox
/// functionality should be implemented by the security/platform layers.
#[derive(Debug, Clone, Copy, Default)]
pub struct LinuxProcessManager;

impl LinuxProcessManager {
    pub fn new() -> Self {
        Self
    }

    pub fn spawn(
        &self,
        executable: impl AsRef<str>,
        arguments: &[String],
    ) -> Result<LinuxProcess, LinuxProcessError> {
        let executable = executable.as_ref().to_string();

        let mut command = Command::new(&executable);

        command
            .args(arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = command.spawn()?;

        Ok(LinuxProcess {
            child,
            executable,
        })
    }

    pub fn command(
        &self,
        executable: impl AsRef<str>,
    ) -> Command {
        Command::new(executable.as_ref())
    }
}

/// Linux process errors.
#[derive(Debug)]
pub enum LinuxProcessError {
    Io(std::io::Error),
    SpawnDenied(String),
    InvalidExecutable(String),
}

impl fmt::Display for LinuxProcessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "process error: {error}"),
            Self::SpawnDenied(message) => {
                write!(formatter, "process spawn denied: {message}")
            }
            Self::InvalidExecutable(executable) => {
                write!(formatter, "invalid executable: {executable}")
            }
        }
    }
}

impl std::error::Error for LinuxProcessError {}

impl From<std::io::Error> for LinuxProcessError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
