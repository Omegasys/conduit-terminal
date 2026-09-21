use std::path::PathBuf;

use super::manager::{
    Connection,
    ConnectionId,
    ConnectionState,
    ConnectionType,
};

#[derive(Debug, Clone)]
pub struct LocalConnectionConfig {
    pub shell: Option<String>,
    pub working_directory: Option<PathBuf>,
    pub environment: Vec<(String, String)>,
}

impl Default for LocalConnectionConfig {
    fn default() -> Self {
        Self {
            shell: None,
            working_directory: None,
            environment: Vec::new(),
        }
    }
}

pub struct LocalConnection {
    id: ConnectionId,
    config: LocalConnectionConfig,
    state: ConnectionState,
}

impl LocalConnection {
    pub fn new(
        id: ConnectionId,
        config: LocalConnectionConfig,
    ) -> Self {
        Self {
            id,
            config,
            state: ConnectionState::Disconnected,
        }
    }

    pub fn config(&self) -> &LocalConnectionConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut LocalConnectionConfig {
        &mut self.config
    }
}

impl Connection for LocalConnection {
    fn id(&self) -> ConnectionId {
        self.id
    }

    fn connection_type(&self) -> ConnectionType {
        ConnectionType::Local
    }

    fn state(&self) -> ConnectionState {
        self.state
    }

    fn connect(&mut self) -> Result<(), String> {
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
