use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionKind {
    Local,
    Ssh,
    Telnet,
    Serial,
    Container,
    Multiplexer,
    Plugin,
    Custom,
}

impl ConnectionKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Local => "Local",
            Self::Ssh => "SSH",
            Self::Telnet => "Telnet",
            Self::Serial => "Serial",
            Self::Container => "Container",
            Self::Multiplexer => "Multiplexer",
            Self::Plugin => "Plugin",
            Self::Custom => "Custom",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Failed,
    Closing,
}

impl ConnectionState {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Connecting => "Connecting",
            Self::Connected => "Connected",
            Self::Reconnecting => "Reconnecting",
            Self::Failed => "Failed",
            Self::Closing => "Closing",
        }
    }
}

pub struct ConnectionDisplay {
    id: String,
    name: String,
    kind: ConnectionKind,
    state: ConnectionState,
    target: String,
    user: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    error: Option<String>,
    created_at: Instant,
    last_activity: Option<Instant>,
    selected: bool,
}

impl ConnectionDisplay {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        kind: ConnectionKind,
        target: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            state: ConnectionState::Disconnected,
            target: target.into(),
            user: None,
            host: None,
            port: None,
            error: None,
            created_at: Instant::now(),
            last_activity: None,
            selected: false,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn kind(&self) -> ConnectionKind {
        self.kind
    }

    pub fn set_kind(&mut self, kind: ConnectionKind) {
        self.kind = kind;
    }

    pub fn state(&self) -> ConnectionState {
        self.state
    }

    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = state;

        if state == ConnectionState::Connected {
            self.last_activity = Some(Instant::now());
        }
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn set_target(&mut self, target: impl Into<String>) {
        self.target = target.into();
    }

    pub fn user(&self) -> Option<&str> {
        self.user.as_deref()
    }

    pub fn set_user(&mut self, user: Option<String>) {
        self.user = user;
    }

    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }

    pub fn set_host(&mut self, host: Option<String>) {
        self.host = host;
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn set_port(&mut self, port: Option<u16>) {
        self.port = port;
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn created_at(&self) -> Instant {
        self.created_at
    }

    pub fn last_activity(&self) -> Option<Instant> {
        self.last_activity
    }

    pub fn touch(&mut self) {
        self.last_activity = Some(Instant::now());
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

pub struct TuiConnectionView {
    connections: Vec<ConnectionDisplay>,
    selected: Option<String>,
    show_disconnected: bool,
    show_failed: bool,
    compact: bool,
}

impl Default for TuiConnectionView {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiConnectionView {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            selected: None,
            show_disconnected: true,
            show_failed: true,
            compact: false,
        }
    }

    pub fn add(&mut self, connection: ConnectionDisplay) {
        let id = connection.id().to_owned();

        if self
            .connections
            .iter()
            .any(|existing| existing.id() == id)
        {
            return;
        }

        self.connections.push(connection);

        if self.selected.is_none() {
            self.selected = Some(id);
        }

        self.refresh_selection();
    }

    pub fn remove(&mut self, id: &str) -> Option<ConnectionDisplay> {
        let index = self
            .connections
            .iter()
            .position(|connection| connection.id() == id)?;

        let removed = self.connections.remove(index);

        if self.selected.as_deref() == Some(id) {
            self.selected = self
                .connections
                .first()
                .map(|connection| connection.id().to_owned());
        }

        self.refresh_selection();
        Some(removed)
    }

    pub fn get(&self, id: &str) -> Option<&ConnectionDisplay> {
        self.connections
            .iter()
            .find(|connection| connection.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ConnectionDisplay> {
        self.connections
            .iter_mut()
            .find(|connection| connection.id() == id)
    }

    pub fn select(&mut self, id: &str) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        self.selected = Some(id.to_owned());
        self.refresh_selection();
        true
    }

    pub fn selected(&self) -> Option<&ConnectionDisplay> {
        self.selected.as_deref().and_then(|id| self.get(id))
    }

    pub fn selected_id(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    pub fn connections(&self) -> &[ConnectionDisplay] {
        &self.connections
    }

    pub fn visible_connections(&self) -> Vec<&ConnectionDisplay> {
        self.connections
            .iter()
            .filter(|connection| match connection.state() {
                ConnectionState::Disconnected => self.show_disconnected,
                ConnectionState::Failed => self.show_failed,
                _ => true,
            })
            .collect()
    }

    pub fn connected_count(&self) -> usize {
        self.connections
            .iter()
            .filter(|connection| {
                connection.state() == ConnectionState::Connected
            })
            .count()
    }

    pub fn connecting_count(&self) -> usize {
        self.connections
            .iter()
            .filter(|connection| {
                matches!(
                    connection.state(),
                    ConnectionState::Connecting
                        | ConnectionState::Reconnecting
                )
            })
            .count()
    }

    pub fn show_disconnected(&self) -> bool {
        self.show_disconnected
    }

    pub fn set_show_disconnected(&mut self, show: bool) {
        self.show_disconnected = show;
    }

    pub fn show_failed(&self) -> bool {
        self.show_failed
    }

    pub fn set_show_failed(&mut self, show: bool) {
        self.show_failed = show;
    }

    pub fn compact(&self) -> bool {
        self.compact
    }

    pub fn set_compact(&mut self, compact: bool) {
        self.compact = compact;
    }

    pub fn refresh_selection(&mut self) {
        let selected = self.selected.as_deref();

        for connection in &mut self.connections {
            connection.set_selected(selected == Some(connection.id()));
        }
    }

    pub fn clear(&mut self) {
        self.connections.clear();
        self.selected = None;
    }

    pub fn len(&self) -> usize {
        self.connections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.connections.is_empty()
    }
}
