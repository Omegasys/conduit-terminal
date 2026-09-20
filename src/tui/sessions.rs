use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TuiSessionId(u64);

impl TuiSessionId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionDisplayState {
    Created,
    Starting,
    Running,
    Suspended,
    Exited,
    Failed,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionKind {
    Local,
    Remote,
    Container,
    Multiplexed,
    External,
    Unknown,
}

pub struct SessionDisplay {
    id: TuiSessionId,
    kind: SessionKind,
    state: SessionDisplayState,
    name: String,
    shell: String,
    host: Option<String>,
    cwd: Option<String>,
    pid: Option<u32>,
    exit_code: Option<i32>,
    created_at: Instant,
    last_activity: Option<Instant>,
    bytes_read: u64,
    bytes_written: u64,
    visible: bool,
    selected: bool,
}

impl SessionDisplay {
    pub fn new(
        id: TuiSessionId,
        kind: SessionKind,
        name: impl Into<String>,
        shell: impl Into<String>,
    ) -> Self {
        Self {
            id,
            kind,
            state: SessionDisplayState::Created,
            name: name.into(),
            shell: shell.into(),
            host: None,
            cwd: None,
            pid: None,
            exit_code: None,
            created_at: Instant::now(),
            last_activity: None,
            bytes_read: 0,
            bytes_written: 0,
            visible: true,
            selected: false,
        }
    }

    pub fn id(&self) -> TuiSessionId {
        self.id
    }

    pub fn kind(&self) -> SessionKind {
        self.kind
    }

    pub fn set_kind(&mut self, kind: SessionKind) {
        self.kind = kind;
    }

    pub fn state(&self) -> SessionDisplayState {
        self.state
    }

    pub fn set_state(&mut self, state: SessionDisplayState) {
        self.state = state;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn shell(&self) -> &str {
        &self.shell
    }

    pub fn set_shell(&mut self, shell: impl Into<String>) {
        self.shell = shell.into();
    }

    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }

    pub fn set_host(&mut self, host: Option<String>) {
        self.host = host;
    }

    pub fn cwd(&self) -> Option<&str> {
        self.cwd.as_deref()
    }

    pub fn set_cwd(&mut self, cwd: Option<String>) {
        self.cwd = cwd;
    }

    pub fn pid(&self) -> Option<u32> {
        self.pid
    }

    pub fn set_pid(&mut self, pid: Option<u32>) {
        self.pid = pid;
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn set_exit_code(&mut self, code: Option<i32>) {
        self.exit_code = code;
    }

    pub fn created_at(&self) -> Instant {
        self.created_at
    }

    pub fn last_activity(&self) -> Option<Instant> {
        self.last_activity
    }

    pub fn record_read(&mut self, bytes: u64) {
        self.bytes_read = self.bytes_read.saturating_add(bytes);
        self.last_activity = Some(Instant::now());
    }

    pub fn record_written(&mut self, bytes: u64) {
        self.bytes_written = self.bytes_written.saturating_add(bytes);
        self.last_activity = Some(Instant::now());
    }

    pub fn bytes_read(&self) -> u64 {
        self.bytes_read
    }

    pub fn bytes_written(&self) -> u64 {
        self.bytes_written
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

pub struct TuiSessionView {
    sessions: Vec<SessionDisplay>,
    active: Option<TuiSessionId>,
    show_closed: bool,
    compact: bool,
}

impl Default for TuiSessionView {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiSessionView {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            active: None,
            show_closed: false,
            compact: false,
        }
    }

    pub fn add(&mut self, session: SessionDisplay) -> TuiSessionId {
        let id = session.id();

        self.sessions.push(session);

        if self.active.is_none() {
            self.active = Some(id);
        }

        self.refresh_selection();
        id
    }

    pub fn remove(&mut self, id: TuiSessionId) -> Option<SessionDisplay> {
        let index = self.sessions.iter().position(|session| session.id() == id)?;
        let removed = self.sessions.remove(index);

        if self.active == Some(id) {
            self.active = self
                .sessions
                .iter()
                .find(|session| session.state() != SessionDisplayState::Closed)
                .map(|session| session.id());
        }

        self.refresh_selection();
        Some(removed)
    }

    pub fn get(&self, id: TuiSessionId) -> Option<&SessionDisplay> {
        self.sessions.iter().find(|session| session.id() == id)
    }

    pub fn get_mut(&mut self, id: TuiSessionId) -> Option<&mut SessionDisplay> {
        self.sessions.iter_mut().find(|session| session.id() == id)
    }

    pub fn activate(&mut self, id: TuiSessionId) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        self.active = Some(id);
        self.refresh_selection();
        true
    }

    pub fn active(&self) -> Option<&SessionDisplay> {
        self.active.and_then(|id| self.get(id))
    }

    pub fn active_id(&self) -> Option<TuiSessionId> {
        self.active
    }

    pub fn sessions(&self) -> &[SessionDisplay] {
        &self.sessions
    }

    pub fn visible_sessions(&self) -> Vec<&SessionDisplay> {
        self.sessions
            .iter()
            .filter(|session| {
                session.visible()
                    && (self.show_closed
                        || session.state() != SessionDisplayState::Closed)
            })
            .collect()
    }

    pub fn show_closed(&self) -> bool {
        self.show_closed
    }

    pub fn set_show_closed(&mut self, show: bool) {
        self.show_closed = show;
    }

    pub fn compact(&self) -> bool {
        self.compact
    }

    pub fn set_compact(&mut self, compact: bool) {
        self.compact = compact;
    }

    pub fn refresh_selection(&mut self) {
        let active = self.active;

        for session in &mut self.sessions {
            session.set_selected(active == Some(session.id()));
        }
    }

    pub fn running_count(&self) -> usize {
        self.sessions
            .iter()
            .filter(|session| session.state() == SessionDisplayState::Running)
            .count()
    }

    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    pub fn clear(&mut self) {
        self.sessions.clear();
        self.active = None;
    }
}
