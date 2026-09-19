//! Child-process management.
//!
//! `Process` owns the operating-system child process while the PTY owns
//! the terminal communication channel.

use std::io;
use std::process::{Child, Command, ExitStatus, Stdio};

/// Configuration used when starting a terminal process.
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    /// Executable to launch.
    pub program: String,

    /// Arguments passed to the executable.
    pub arguments: Vec<String>,

    /// Optional working directory.
    pub working_directory: Option<String>,

    /// Additional environment variables.
    pub environment: Vec<(String, String)>,
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            program: default_shell(),
            arguments: Vec::new(),
            working_directory: None,
            environment: Vec::new(),
        }
    }
}

/// Current state of the child process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// Process has not been started.
    NotStarted,

    /// Process is currently running.
    Running,

    /// Process exited normally or abnormally.
    Exited,

    /// Process was terminated by the parent.
    Terminated,
}

/// Wrapper around an operating-system child process.
pub struct Process {
    config: ProcessConfig,
    child: Option<Child>,
    state: ProcessState,
    exit_status: Option<ExitStatus>,
}

impl Process {
    /// Creates a process without starting it.
    pub fn new(config: ProcessConfig) -> Self {
        Self {
            config,
            child: None,
            state: ProcessState::NotStarted,
            exit_status: None,
        }
    }

    /// Starts the configured process.
    ///
    /// This basic process implementation is intentionally independent
    /// from the PTY implementation. `Pty` is responsible for terminal
    /// attachment on Unix.
    pub fn start(&mut self) -> io::Result<()> {
        if self.state == ProcessState::Running {
            return Ok(());
        }

        let mut command = Command::new(&self.config.program);

        command
            .args(&self.config.arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        if let Some(directory) = &self.config.working_directory {
            command.current_dir(directory);
        }

        for (key, value) in &self.config.environment {
            command.env(key, value);
        }

        let child = command.spawn()?;

        self.child = Some(child);
        self.state = ProcessState::Running;
        self.exit_status = None;

        Ok(())
    }

    /// Returns the configured process.
    pub fn config(&self) -> &ProcessConfig {
        &self.config
    }

    /// Returns the current process state.
    pub fn state(&self) -> ProcessState {
        self.state
    }

    /// Returns the operating-system process ID.
    pub fn pid(&self) -> Option<u32> {
        self.child.as_ref().map(|child| child.id())
    }

    /// Checks whether the process has exited without blocking.
    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        let Some(child) = self.child.as_mut() else {
            return Ok(None);
        };

        match child.try_wait()? {
            Some(status) => {
                self.exit_status = Some(status);
                self.state = ProcessState::Exited;
                Ok(Some(status))
            }

            None => Ok(None),
        }
    }

    /// Waits for the process to exit.
    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        let child = self
            .child
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "process not started"))?;

        let status = child.wait()?;

        self.exit_status = Some(status);
        self.state = ProcessState::Exited;

        Ok(status)
    }

    /// Terminates the process.
    pub fn terminate(&mut self) -> io::Result<()> {
        if let Some(child) = self.child.as_mut() {
            child.kill()?;
            self.state = ProcessState::Terminated;
        }

        Ok(())
    }

    /// Returns the exit status if one is available.
    pub fn exit_status(&self) -> Option<ExitStatus> {
        self.exit_status
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if self.state == ProcessState::Running {
            let _ = self.terminate();
        }
    }
}

fn default_shell() -> String {
    if let Ok(shell) = std::env::var("SHELL") {
        if !shell.trim().is_empty() {
            return shell;
        }
    }

    #[cfg(unix)]
    {
        "/bin/sh".to_string()
    }

    #[cfg(windows)]
    {
        "cmd.exe".to_string()
    }
}
