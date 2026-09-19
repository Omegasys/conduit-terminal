#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionKind {
    Local,
    Ssh,
    Telnet,
    Serial,
    Container,
    VirtualMachine,
    Remote,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct ConnectionEntry {
    id: String,
    name: String,
    address: String,
    username: Option<String>,
    kind: ConnectionKind,
    state: ConnectionState,
    active: bool,
    favorite: bool,
    visible: bool,
    enabled: bool,
}

impl ConnectionEntry {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        kind: ConnectionKind,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            address: String::new(),
            username: None,
            kind,
            state: ConnectionState::Disconnected,
            active: false,
            favorite: false,
            visible: true,
            enabled: true,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn kind(&self) -> ConnectionKind {
        self.kind
    }

    pub fn state(&self) -> ConnectionState {
        self.state
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_favorite(&self) -> bool {
        self.favorite
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn set_address(&mut self, address: impl Into<String>) {
        self.address = address.into();
    }

    pub fn set_username(&mut self, username: Option<String>) {
        self.username = username;
    }

    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = state;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_favorite(&mut self, favorite: bool) {
        self.favorite = favorite;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !enabled {
            self.state = ConnectionState::Disabled;
        }
    }

    pub fn can_activate(&self) -> bool {
        self.visible && self.enabled
    }
}

#[derive(Debug, Default)]
pub struct ConnectionManager {
    connections: Vec<ConnectionEntry>,
    active: Option<String>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, connection: ConnectionEntry) {
        if let Some(existing) = self
            .connections
            .iter_mut()
            .find(|item| item.id() == connection.id())
        {
            *existing = connection;
            return;
        }

        self.connections.push(connection);
    }

    pub fn remove(&mut self, id: &str) -> Option<ConnectionEntry> {
        let index = self
            .connections
            .iter()
            .position(|item| item.id() == id)?;

        if self.active.as_deref() == Some(id) {
            self.active = None;
        }

        Some(self.connections.remove(index))
    }

    pub fn get(&self, id: &str) -> Option<&ConnectionEntry> {
        self.connections.iter().find(|item| item.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ConnectionEntry> {
        self.connections
            .iter_mut()
            .find(|item| item.id() == id)
    }

    pub fn activate(&mut self, id: &str) -> bool {
        if !self
            .connections
            .iter()
            .any(|item| item.id() == id && item.can_activate())
        {
            return false;
        }

        for connection in &mut self.connections {
            connection.set_active(connection.id() == id);
        }

        self.active = Some(id.to_string());
        true
    }

    pub fn active(&self) -> Option<&ConnectionEntry> {
        self.active.as_deref().and_then(|id| self.get(id))
    }

    pub fn active_id(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ConnectionEntry> {
        self.connections.iter()
    }

    pub fn connected(&self) -> impl Iterator<Item = &ConnectionEntry> {
        self.connections
            .iter()
            .filter(|item| item.state() == ConnectionState::Connected)
    }

    pub fn favorites(&self) -> impl Iterator<Item = &ConnectionEntry> {
        self.connections
            .iter()
            .filter(|item| item.is_favorite())
    }

    pub fn clear(&mut self) {
        self.connections.clear();
        self.active = None;
    }

    pub fn len(&self) -> usize {
        self.connections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.connections.is_empty()
    }
}
