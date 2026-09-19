#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Created,
    Starting,
    Running,
    Suspended,
    Stopped,
    Error,
    Closed,
}

#[derive(Debug, Clone)]
pub struct SessionEntry {
    id: String,
    name: String,
    shell: String,
    working_directory: String,
    state: SessionState,
    active: bool,
    attached: bool,
    visible: bool,
    enabled: bool,
}

impl SessionEntry {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            shell: String::new(),
            working_directory: String::new(),
            state: SessionState::Created,
            active: false,
            attached: false,
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

    pub fn shell(&self) -> &str {
        &self.shell
    }

    pub fn working_directory(&self) -> &str {
        &self.working_directory
    }

    pub fn state(&self) -> SessionState {
        self.state
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_attached(&self) -> bool {
        self.attached
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

    pub fn set_shell(&mut self, shell: impl Into<String>) {
        self.shell = shell.into();
    }

    pub fn set_working_directory(&mut self, path: impl Into<String>) {
        self.working_directory = path.into();
    }

    pub fn set_state(&mut self, state: SessionState) {
        self.state = state;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_attached(&mut self, attached: bool) {
        self.attached = attached;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn can_activate(&self) -> bool {
        self.visible && self.enabled && self.state != SessionState::Closed
    }
}

#[derive(Debug, Default)]
pub struct SessionManager {
    sessions: Vec<SessionEntry>,
    active: Option<String>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, session: SessionEntry) {
        if let Some(existing) = self
            .sessions
            .iter_mut()
            .find(|item| item.id() == session.id())
        {
            *existing = session;
            return;
        }

        self.sessions.push(session);
    }

    pub fn remove(&mut self, id: &str) -> Option<SessionEntry> {
        let index = self.sessions.iter().position(|item| item.id() == id)?;

        if self.active.as_deref() == Some(id) {
            self.active = None;
        }

        Some(self.sessions.remove(index))
    }

    pub fn get(&self, id: &str) -> Option<&SessionEntry> {
        self.sessions.iter().find(|item| item.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut SessionEntry> {
        self.sessions.iter_mut().find(|item| item.id() == id)
    }

    pub fn activate(&mut self, id: &str) -> bool {
        if !self
            .sessions
            .iter()
            .any(|item| item.id() == id && item.can_activate())
        {
            return false;
        }

        for session in &mut self.sessions {
            session.set_active(session.id() == id);
        }

        self.active = Some(id.to_string());
        true
    }

    pub fn active(&self) -> Option<&SessionEntry> {
        self.active.as_deref().and_then(|id| self.get(id))
    }

    pub fn active_id(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub fn iter(&self) -> impl Iterator<Item = &SessionEntry> {
        self.sessions.iter()
    }

    pub fn running(&self) -> impl Iterator<Item = &SessionEntry> {
        self.sessions
            .iter()
            .filter(|item| item.state() == SessionState::Running)
    }

    pub fn clear(&mut self) {
        self.sessions.clear();
        self.active = None;
    }

    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }
}
