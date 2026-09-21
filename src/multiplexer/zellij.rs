use std::process::Command;

use super::{
    Multiplexer,
    MultiplexerKind,
    MultiplexerSession,
    SessionId,
    SessionState,
};

/// Configuration for Zellij integration.
#[derive(Debug, Clone)]
pub struct ZellijConfig {
    pub executable: String,
    pub default_shell: Option<String>,
    pub layout: Option<String>,
}

impl Default for ZellijConfig {
    fn default() -> Self {
        Self {
            executable: "zellij".to_string(),
            default_shell: None,
            layout: None,
        }
    }
}

impl ZellijConfig {
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

    pub fn with_layout<S: Into<String>>(mut self, layout: S) -> Self {
        self.layout = Some(layout.into());
        self
    }
}

/// Zellij integration.
#[derive(Debug, Clone)]
pub struct ZellijMultiplexer {
    config: ZellijConfig,
}

impl ZellijMultiplexer {
    pub fn new(config: ZellijConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &ZellijConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut ZellijConfig {
        &mut self.config
    }

    fn command(&self) -> Command {
        Command::new(&self.config.executable)
    }
}

impl Default for ZellijMultiplexer {
    fn default() -> Self {
        Self::new(ZellijConfig::default())
    }
}

impl Multiplexer for ZellijMultiplexer {
    fn kind(&self) -> MultiplexerKind {
        MultiplexerKind::Zellij
    }

    fn is_available(&self) -> bool {
        self.command()
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn list_sessions(&self) -> Result<Vec<MultiplexerSession>, String> {
        let output = self
            .command()
            .args(["list-sessions", "--short"])
            .output()
            .map_err(|error| error.to_string())?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let text = String::from_utf8_lossy(&output.stdout);

        let sessions = text
            .lines()
            .filter_map(|line| {
                let name = line.trim();

                if name.is_empty() {
                    return None;
                }

                Some(
                    MultiplexerSession::new(SessionId::new(name))
                        .with_name(name)
                        .with_state(SessionState::Detached),
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
        command.arg("--session");

        let session_name = name.unwrap_or("conduit");
        command.arg(session_name);

        if let Some(layout) = &self.config.layout {
            command.args(["--layout", layout]);
        }

        if let Some(shell) = &self.config.default_shell {
            command.args(["--", shell]);
        }

        let output = command.output().map_err(|error| error.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
        }

        Ok(SessionId::new(session_name))
    }

    fn attach(
        &mut self,
        session: &SessionId,
    ) -> Result<(), String> {
        let output = self
            .command()
            .args(["attach", session.as_str()])
            .output()
            .map_err(|error| error.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }

    fn detach(&mut self) -> Result<(), String> {
        // Zellij normally detaches through its own keybinding/UI.
        // This command is intentionally represented as an integration hook.
        let output = self
            .command()
            .args(["action", "detach"])
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
            .args(["delete-session", session.as_str()])
            .output()
            .map_err(|error| error.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }
}
