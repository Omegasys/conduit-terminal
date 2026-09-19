//! Terminal session management.
//!
//! A session represents one interactive terminal environment.
//!
//! Session architecture:
//
//! Session
//! ├── PTY
//! ├── Process
//! ├── Terminal
//! └── Screen

use std::io;

use super::{
    Pty,
    PtyConfig,
    Screen,
    Terminal,
};

/// Session configuration.
#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub shell: String,
    pub working_directory: Option<String>,
    pub environment: Vec<(String, String)>,
    pub rows: u16,
    pub columns: u16,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            shell: default_shell(),
            working_directory: None,
            environment: Vec::new(),
            rows: 24,
            columns: 80,
        }
    }
}

/// Current session state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Created,
    Running,
    Exited,
    Terminated,
}

/// An interactive Conduit terminal session.
pub struct Session {
    config: SessionConfig,
    pty: Option<Pty>,
    terminal: Terminal,
    state: SessionState,
}

impl Session {
    /// Creates a session without starting its shell.
    pub fn new(config: SessionConfig) -> io::Result<Self> {
        let screen = Screen::new(
            config.rows as usize,
            config.columns as usize,
        );

        let terminal = Terminal::new(screen);

        Ok(Self {
            config,
            pty: None,
            terminal,
            state: SessionState::Created,
        })
    }

    /// Starts the configured shell.
    pub fn start(&mut self) -> io::Result<()> {
        if self.state == SessionState::Running {
            return Ok(());
        }

        let pty_config = PtyConfig {
            rows: self.config.rows,
            columns: self.config.columns,
        };

        let pty = Pty::spawn(
            &self.config.shell,
            &[],
            &pty_config,
            self.config.working_directory.as_deref(),
            &self.config.environment,
        )?;

        self.pty = Some(pty);
        self.state = SessionState::Running;

        Ok(())
    }

    /// Processes the interactive session until the shell exits.
    ///
    /// The initial implementation uses a simple polling loop. The
    /// future event-driven implementation can replace this without
    /// changing the public session concept.
    pub fn run(&mut self) -> io::Result<()> {
        if self.state != SessionState::Running {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "session is not running",
            ));
        }

        let mut buffer = [0u8; 8192];

        loop {
            let bytes_read = {
                let pty = self.pty.as_ref().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        "session PTY unavailable",
                    )
                })?;

                pty.read(&mut buffer)?
            };

            if bytes_read > 0 {
                self.terminal
                    .process_output(&buffer[..bytes_read]);
            }

            if bytes_read == 0 {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }

            if self.process_exited()? {
                self.state = SessionState::Exited;
                break;
            }
        }

        Ok(())
    }

    /// Sends input to the shell.
    pub fn write_input(&self, data: &[u8]) -> io::Result<usize> {
        let pty = self.pty.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "session PTY unavailable",
            )
        })?;

        pty.write(data)
    }

    /// Resizes the session.
    pub fn resize(&mut self, rows: u16, columns: u16) -> io::Result<()> {
        self.config.rows = rows;
        self.config.columns = columns;

        self.terminal.resize(rows as usize, columns as usize);

        if let Some(pty) = &self.pty {
            pty.resize(rows, columns)?;
        }

        Ok(())
    }

    /// Returns the session's terminal state.
    pub fn terminal(&self) -> &Terminal {
        &self.terminal
    }

    /// Returns mutable terminal state.
    pub fn terminal_mut(&mut self) -> &mut Terminal {
        &mut self.terminal
    }

    /// Returns the PTY process ID.
    pub fn pid(&self) -> Option<i32> {
        self.pty.as_ref().map(|pty| pty.pid())
    }

    /// Returns the session state.
    pub fn state(&self) -> SessionState {
        self.state
    }

    /// Terminates the session.
    pub fn terminate(&mut self) -> io::Result<()> {
        if let Some(pty) = self.pty.take() {
            #[cfg(unix)]
            unsafe {
                libc::kill(pty.pid(), libc::SIGTERM);
            }
        }

        self.state = SessionState::Terminated;

        Ok(())
    }

    fn process_exited(&self) -> io::Result<bool> {
        let Some(pty) = &self.pty else {
            return Ok(true);
        };

        #[cfg(unix)]
        {
            let mut status: libc::c_int = 0;

            let result = unsafe {
                libc::waitpid(
                    pty.pid(),
                    &mut status,
                    libc::WNOHANG,
                )
            };

            if result == pty.pid() {
                return Ok(true);
            }

            if result == -1 {
                let error = io::Error::last_os_error();

                if error.raw_os_error() == Some(libc::ECHILD) {
                    return Ok(true);
                }

                return Err(error);
            }
        }

        Ok(false)
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.state == SessionState::Running {
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
