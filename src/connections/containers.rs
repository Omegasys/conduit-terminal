use super::manager::{
    Connection,
    ConnectionId,
    ConnectionState,
    ConnectionType,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerId(String);

impl ContainerId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerRuntime {
    Docker,
    Podman,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created,
    Running,
    Paused,
    Stopped,
    Restarting,
    Removing,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ContainerConnection {
    id: ConnectionId,
    container_id: ContainerId,
    runtime: ContainerRuntime,
    state: ConnectionState,
    container_state: ContainerState,
}

impl ContainerConnection {
    pub fn new(
        id: ConnectionId,
        container_id: ContainerId,
        runtime: ContainerRuntime,
    ) -> Self {
        Self {
            id,
            container_id,
            runtime,
            state: ConnectionState::Disconnected,
            container_state: ContainerState::Unknown,
        }
    }

    pub fn container_id(&self) -> &ContainerId {
        &self.container_id
    }

    pub fn runtime(&self) -> ContainerRuntime {
        self.runtime
    }

    pub fn container_state(&self) -> ContainerState {
        self.container_state
    }

    pub fn set_container_state(
        &mut self,
        state: ContainerState,
    ) {
        self.container_state = state;
    }
}

impl Connection for ContainerConnection {
    fn id(&self) -> ConnectionId {
        self.id
    }

    fn connection_type(&self) -> ConnectionType {
        ConnectionType::Container
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
