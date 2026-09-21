use std::time::Duration;

use super::manager::{
    Connection,
    ConnectionId,
    ConnectionState,
    ConnectionType,
};

#[derive(Debug, Clone)]
pub struct SshConnectionConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub timeout: Duration,
    pub shell: Option<String>,
}

impl Default for SshConnectionConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 22,
            username: String::new(),
            timeout: Duration::from_secs(10),
            shell: None,
        }
    }
}

impl SshConnectionConfig {
    pub fn new(
        host: impl Into<String>,
        username: impl Into<String>,
    ) -> Self {
        Self {
            host: host.into(),
            username: username.into(),
            ..Self::default()
        }
    }

    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub struct SshConnection {
    id: ConnectionId,
    config: SshConnectionConfig,
    state: ConnectionState,
}

impl SshConnection {
    pub fn new(
        id: ConnectionId,
        config: SshConnectionConfig,
    ) -> Self {
        Self {
            id,
            config,
            state: ConnectionState::Disconnected,
        }
    }

    pub fn config(&self) -> &SshConnectionConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut SshConnectionConfig {
        &mut self.config
    }
}

impl Connection for SshConnection {
    fn id(&self) -> ConnectionId {
        self.id
    }

    fn connection_type(&self) -> ConnectionType {
        ConnectionType::Ssh
    }

    fn state(&self) -> ConnectionState {
        self.state
    }

    fn connect(&mut self) -> Result<(), String> {
        if self.config.host.is_empty() {
            return Err("SSH host has not been configured".into());
        }

        if self.config.username.is_empty() {
            return Err("SSH username has not been configured".into());
        }

        self.state = ConnectionState::Connecting;

        self.state = ConnectionState::Connected;

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), String> {
        self.state = ConnectionState::Disconnecting;
        self.state = ConnectionState::Disconnected;

        Ok(())
    }
}
