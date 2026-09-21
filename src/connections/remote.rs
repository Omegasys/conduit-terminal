use std::time::Duration;

use super::manager::{
    Connection,
    ConnectionId,
    ConnectionState,
    ConnectionType,
};

#[derive(Debug, Clone)]
pub struct RemoteConnectionConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub protocol: RemoteProtocol,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteProtocol {
    Ssh,
    Telnet,
    Serial,
    Custom,
}

impl Default for RemoteConnectionConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 22,
            username: None,
            protocol: RemoteProtocol::Ssh,
            timeout: Duration::from_secs(10),
        }
    }
}

pub struct RemoteConnection {
    id: ConnectionId,
    config: RemoteConnectionConfig,
    state: ConnectionState,
}

impl RemoteConnection {
    pub fn new(
        id: ConnectionId,
        config: RemoteConnectionConfig,
    ) -> Self {
        Self {
            id,
            config,
            state: ConnectionState::Disconnected,
        }
    }

    pub fn config(&self) -> &RemoteConnectionConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut RemoteConnectionConfig {
        &mut self.config
    }

    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }
}

impl Connection for RemoteConnection {
    fn id(&self) -> ConnectionId {
        self.id
    }

    fn connection_type(&self) -> ConnectionType {
        ConnectionType::Remote
    }

    fn state(&self) -> ConnectionState {
        self.state
    }

    fn connect(&mut self) -> Result<(), String> {
        if self.config.host.is_empty() {
            return Err("remote host has not been configured".into());
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
