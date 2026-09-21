use std::collections::HashMap;

/// Identifier for a multiplexer session.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for SessionId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for SessionId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Current state of a multiplexer session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Unknown,
    Created,
    Detached,
    Attached,
    Dead,
}

/// A session exposed by a terminal multiplexer.
#[derive(Debug, Clone)]
pub struct MultiplexerSession {
    id: SessionId,
    name: Option<String>,
    state: SessionState,
    windows: usize,
}

impl MultiplexerSession {
    pub fn new(id: SessionId) -> Self {
        Self {
            id,
            name: None,
            state: SessionState::Unknown,
            windows: 0,
        }
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_state(mut self, state: SessionState) -> Self {
        self.state = state;
        self
    }

    pub fn with_windows(mut self, windows: usize) -> Self {
        self.windows = windows;
        self
    }

    pub fn id(&self) -> &SessionId {
        &self.id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn state(&self) -> SessionState {
        self.state
    }

    pub fn windows(&self) -> usize {
        self.windows
    }

    pub fn is_alive(&self) -> bool {
        !matches!(self.state, SessionState::Dead)
    }

    pub fn is_attached(&self) -> bool {
        matches!(self.state, SessionState::Attached)
    }
}

/// Stores sessions known to Conduit.
#[derive(Debug, Default)]
pub struct SessionManager {
    sessions: HashMap<SessionId, MultiplexerSession>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, session: MultiplexerSession) {
        self.sessions.insert(session.id().clone(), session);
    }

    pub fn get(&self, id: &SessionId) -> Option<&MultiplexerSession> {
        self.sessions.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &SessionId,
    ) -> Option<&mut MultiplexerSession> {
        self.sessions.get_mut(id)
    }

    pub fn remove(&mut self, id: &SessionId) -> Option<MultiplexerSession> {
        self.sessions.remove(id)
    }

    pub fn contains(&self, id: &SessionId) -> bool {
        self.sessions.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    pub fn sessions(&self) -> impl Iterator<Item = &MultiplexerSession> {
        self.sessions.values()
    }

    pub fn clear(&mut self) {
        self.sessions.clear();
    }
}
