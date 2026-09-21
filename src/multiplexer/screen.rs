use std::process::Command;

use super::{
    Multiplexer,
    MultiplexerKind,
    MultiplexerSession,
    SessionId,
    SessionState,
};

/// Configuration for GNU screen integration.
#[derive(Debug, Clone)]
pub struct ScreenConfig {
    pub executable: String,
    pub default_shell: Option<String>,
}

impl Default for ScreenConfig {
    fn default() -> Self {
        Self {
            executable: "screen".to_string(),
            default_shell: None,
        }
    }
}

impl ScreenConfig {
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
}

/// GNU screen integration.
#[derive(Debug, Clone)]
pub struct ScreenMultiplexer {
    config: ScreenConfig,
}

impl ScreenMultiplexer {
    pub fn new(config: ScreenConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &ScreenConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut ScreenConfig {
        &mut self.config
    }

    fn command(&self) -> Command {
        Command::new(&self.config.executable)
    }
}

impl Default for ScreenMultiplexer {
    fn default() -> Self {
        Self::new(ScreenConfig::default())
    }
}

impl Multiplexer for ScreenMultiplexer {
    fn kind(&self) -> MultiplexerKind {
        MultiplexerKind::Screen
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
            .arg("-ls")
            .output()
            .map_err(|error| error.to_string())?;

        let text = String::from_utf8_lossy(&output.stdout);

        let sessions = text
            .lines()
            .filter_map(|line| {
                let line = line.trim();

                if !line.contains('\t') && !line.contains("Detached")
                    && !line.contains("Attached")
                {
                    return None;
                }

                let first = line.split_whitespace().next()?;

                if !first.contains('.') {
                    return None;
                }

                let state = if line.contains("Attached") {
                    SessionState::Attached
                } else {
                    SessionState::Detached
                };

                Some(
                    MultiplexerSession::new(SessionId::new(first))
                        .with_name(first)
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

        command.arg("-dmS");

        let session_name = name.unwrap_or("conduit");
        command.arg(session_name);

        if let Some(shell) = &self.config.default_shell {
            command.arg(shell);
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
            .args(["-r", session.as_str()])
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
            .args(["-D"])
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
            .args(["-S", session.as_str(), "-X", "quit"])
            .output()
            .map_err(|error| error.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }
}
