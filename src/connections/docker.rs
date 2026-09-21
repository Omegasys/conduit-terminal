use super::containers::{
    ContainerConnection,
    ContainerId,
    ContainerRuntime,
};

use super::manager::{
    Connection,
    ConnectionId,
    ConnectionState,
    ConnectionType,
};

#[derive(Debug, Clone)]
pub struct DockerContainer {
    pub id: ContainerId,
    pub name: String,
    pub image: String,
    pub running: bool,
}

impl DockerContainer {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        image: impl Into<String>,
    ) -> Self {
        Self {
            id: ContainerId::new(id),
            name: name.into(),
            image: image.into(),
            running: false,
        }
    }
}

pub struct DockerConnection {
    inner: ContainerConnection,
}

impl DockerConnection {
    pub fn new(
        id: ConnectionId,
        container_id: impl Into<String>,
    ) -> Self {
        Self {
            inner: ContainerConnection::new(
                id,
                ContainerId::new(container_id),
                ContainerRuntime::Docker,
            ),
        }
    }

    pub fn container(&self) -> &ContainerId {
        self.inner.container_id()
    }

    pub fn into_inner(self) -> ContainerConnection {
        self.inner
    }
}

impl Connection for DockerConnection {
    fn id(&self) -> ConnectionId {
        self.inner.id()
    }

    fn connection_type(&self) -> ConnectionType {
        ConnectionType::Docker
    }

    fn state(&self) -> ConnectionState {
        self.inner.state()
    }

    fn connect(&mut self) -> Result<(), String> {
        self.inner.connect()
    }

    fn disconnect(&mut self) -> Result<(), String> {
        self.inner.disconnect()
    }
}
