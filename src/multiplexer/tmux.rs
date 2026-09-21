use std::process::Command;

use super::{
    Multiplexer,
    MultiplexerKind,
    MultiplexerSession,
    SessionId,
    SessionState,
};

/// Configuration for tmux integration.
#[derive(Debug, Clone)]
pub struct TmuxConfig {
    pub executable: String,
    pub default_shell: Option<String>,
    pub socket: Option<String>,
}

impl Default for TmuxConfig {
    fn default() -> Self {
        Self {
            executable: "tmux".to_string(),
            default_shell: None,
            socket: None,
        }
    }
}

impl TmuxConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_executable<S: Into<String>>(mut self, executable: S) -> Self {
        self.executable = executable.into();
        self
    }

    pub fn with_shell<S: Into<String>>(mut self, shell: S) -> Self {
        self.default_shell = Some(shell.into());
        self
    }

    pub fn with_socket<S: Into<String>>(mut self, socket: S) -> Self {
        self.socket = Some(socket.into());
        self
    }
}

/// tmux integration.
#[derive(Debug, Clone)]
pub struct TmuxMultiplexer {
    config: TmuxConfig,
}

impl TmuxMultiplexer {
    pub fn new(config: TmuxConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &TmuxConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut TmuxConfig {
        &mut self.config
    }

    fn command(&self) -> Command {
        let mut command = Command::new(&self.config.executable);

        if let Some(socket) = &self.config.socket {
            command.arg("-L").arg(socket);
        }

        command
    }

    fn session_argument(&self, session: &SessionId) -> String {
        session.as_str().to_string()
    }
}

impl Default for TmuxMultiplexer {
    fn default() -> Self {
        Self::new(TmuxConfig::default())
    }
}

impl Multiplexer for TmuxMultiplexer {
    fn kind(&self) -> MultiplexerKind {
        MultiplexerKind::Tmux
    }

    fn is_available(&self) -> bool {
        self.command()
            .arg("-V")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn list_sessions(&self) -> Result<Vec<MultiplexerSession>, String> {
        let output = self
            .command()
            .args([
                "list-sessions",
                "-F",
                "#{session_name}\t#{session_windows}\t#{session_attached}",
            ])
            .output()
            .map_err(|error| error.to_string())?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let text = String::from_utf8_lossy(&output.stdout);

        let sessions = text
            .lines()
            .filter_map(|line| {
                let mut fields = line.split('\t');

                let name = fields.next()?.trim();
                let windows = fields
                    .next()
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(0);

                let attached = fields
                    .next()
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(0);

                let state = if attached > 0 {
                    SessionState::Attached
                } else {
                    SessionState::Detached
                };

                Some(
                    MultiplexerSession::new(SessionId::new(name))
                        .with_name(name)
                        .with_windows(windows)
                        .with_state(state),
                )
            })
            .collect();

        Ok(sessions)
    }

    fn create_session(
        &mut self,
        name: Option<&str>,
    ) -> Result<SessionId, String> {
        let mut command = self.command();
        command.arg("new-session").arg("-d");

        if let Some(name) = name {
            command.args(["-s", name]);
        }

        if let Some(shell) = &self.config.default_shell {
            command.args(["-c", shell]);
        }

        let output = command.output().map_err(|error| error.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
        }

        let id = name
            .map(SessionId::from)
            .unwrap_or_else(|| SessionId::new("conduit"));

        Ok(id)
    }

    fn attach(
        &mut self,
        session: &SessionId,
    ) -> Result<(), String> {
        let output = self
            .command()
            .args(["attach-session", "-t", &self.session_argument(session)])
            .output()
            .map_err(|error| error.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }

    fn detach(&mut self) -> Result<(), String> {
        let output = self
            .command()
            .args(["detach-client"])
            .output()
            .map_err(|error| error.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }

    fn kill_session(
        &mut self,
        session: &SessionId,
    ) -> Result<(), String> {
        let output = self
            .command()
            .args(["kill-session", "-t", &self.session_argument(session)])
            .output()
            .map_err(|error| error.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }
}
