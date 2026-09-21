use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionId(u64);

impl ConnectionId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Local,
    Ssh,
    Serial,
    Remote,
    Container,
    Docker,
    Podman,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Failed,
}

pub trait Connection: Send {
    fn id(&self) -> ConnectionId;
    fn connection_type(&self) -> ConnectionType;
    fn state(&self) -> ConnectionState;

    fn connect(&mut self) -> Result<(), String>;
    fn disconnect(&mut self) -> Result<(), String>;

    fn is_connected(&self) -> bool {
        self.state() == ConnectionState::Connected
    }
}

pub struct ConnectionManager {
    connections: HashMap<ConnectionId, Box<dyn Connection>>,
    next_id: u64,
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn allocate_id(&mut self) -> ConnectionId {
        let id = ConnectionId::new(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);
        id
    }

    pub fn add(
        &mut self,
        connection: Box<dyn Connection>,
    ) -> ConnectionId {
        let id = connection.id();

        self.connections.insert(id, connection);

        id
    }

    pub fn get(
        &self,
        id: ConnectionId,
    ) -> Option<&dyn Connection> {
        self.connections
            .get(&id)
            .map(|connection| connection.as_ref())
    }

    pub fn get_mut(
        &mut self,
        id: ConnectionId,
    ) -> Option<&mut (dyn Connection + '_)> {
        match self.connections.get_mut(&id) {
            Some(connection) => Some(connection.as_mut()),
            None => None,
        }
    }

    pub fn connect(
        &mut self,
        id: ConnectionId,
    ) -> Result<(), String> {
        let connection = self
            .get_mut(id)
            .ok_or_else(|| "connection not found".to_string())?;

        connection.connect()
    }

    pub fn disconnect(
        &mut self,
        id: ConnectionId,
    ) -> Result<(), String> {
        let connection = self
            .get_mut(id)
            .ok_or_else(|| "connection not found".to_string())?;

        connection.disconnect()
    }

    pub fn remove(
        &mut self,
        id: ConnectionId,
    ) -> Option<Box<dyn Connection>> {
        self.connections.remove(&id)
    }

    pub fn contains(&self, id: ConnectionId) -> bool {
        self.connections.contains_key(&id)
    }

    pub fn len(&self) -> usize {
        self.connections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.connections.is_empty()
    }

    pub fn ids(&self) -> Vec<ConnectionId> {
        self.connections.keys().copied().collect()
    }

    pub fn clear(&mut self) {
        self.connections.clear();
    }
}
