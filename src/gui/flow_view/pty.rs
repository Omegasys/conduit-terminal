use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PtyId(u64);

impl PtyId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtyState {
    Created,
    Starting,
    Running,
    Suspended,
    Exited,
    Failed,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtySize {
    pub columns: u16,
    pub rows: u16,
}

impl PtySize {
    pub fn new(columns: u16, rows: u16) -> Self {
        Self { columns, rows }
    }
}

impl Default for PtySize {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

#[derive(Debug, Clone)]
pub struct PtySession {
    id: PtyId,
    state: PtyState,
    size: PtySize,
    shell: Option<String>,
    working_directory: Option<String>,
    process_id: Option<u32>,
    exit_code: Option<i32>,
    started_at: Option<Instant>,
    last_activity: Option<Instant>,
    bytes_read: u64,
    bytes_written: u64,
}

impl PtySession {
    pub fn new(id: PtyId) -> Self {
        Self {
            id,
            state: PtyState::Created,
            size: PtySize::default(),
            shell: None,
            working_directory: None,
            process_id: None,
            exit_code: None,
            started_at: None,
            last_activity: None,
            bytes_read: 0,
            bytes_written: 0,
        }
    }

    pub fn id(&self) -> PtyId {
        self.id
    }

    pub fn state(&self) -> PtyState {
        self.state
    }

    pub fn set_state(&mut self, state: PtyState) {
        if matches!(state, PtyState::Running) && self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }

        self.state = state;
        self.last_activity = Some(Instant::now());
    }

    pub fn size(&self) -> PtySize {
        self.size
    }

    pub fn resize(&mut self, columns: u16, rows: u16) {
        self.size = PtySize::new(columns.max(1), rows.max(1));
        self.touch();
    }

    pub fn shell(&self) -> Option<&str> {
        self.shell.as_deref()
    }

    pub fn set_shell(&mut self, shell: Option<String>) {
        self.shell = shell;
    }

    pub fn working_directory(&self) -> Option<&str> {
        self.working_directory.as_deref()
    }

    pub fn set_working_directory(&mut self, directory: Option<String>) {
        self.working_directory = directory;
    }

    pub fn process_id(&self) -> Option<u32> {
        self.process_id
    }

    pub fn set_process_id(&mut self, process_id: Option<u32>) {
        self.process_id = process_id;
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn set_exit_code(&mut self, exit_code: Option<i32>) {
        self.exit_code = exit_code;
    }

    pub fn started_at(&self) -> Option<Instant> {
        self.started_at
    }

    pub fn last_activity(&self) -> Option<Instant> {
        self.last_activity
    }

    pub fn idle_duration(&self) -> Option<Duration> {
        self.last_activity
            .and_then(|time| Instant::now().checked_duration_since(time))
    }

    pub fn bytes_read(&self) -> u64 {
        self.bytes_read
    }

    pub fn bytes_written(&self) -> u64 {
        self.bytes_written
    }

    pub fn record_read(&mut self, bytes: usize) {
        self.bytes_read = self.bytes_read.saturating_add(bytes as u64);
        self.touch();
    }

    pub fn record_written(&mut self, bytes: usize) {
        self.bytes_written = self.bytes_written.saturating_add(bytes as u64);
        self.touch();
    }

    pub fn touch(&mut self) {
        self.last_activity = Some(Instant::now());
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            PtyState::Starting
                | PtyState::Running
                | PtyState::Suspended
        )
    }

    pub fn close(&mut self) {
        self.state = PtyState::Closed;
        self.touch();
    }
}

#[derive(Debug, Default)]
pub struct PtyManager {
    sessions: Vec<PtySession>,
    next_id: u64,
}

impl PtyManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create(&mut self) -> PtyId {
        let id = PtyId::new(self.next_id);
        self.next_id += 1;

        self.sessions.push(PtySession::new(id));
        id
    }

    pub fn add(&mut self, session: PtySession) {
        self.sessions.push(session);
    }

    pub fn get(&self, id: PtyId) -> Option<&PtySession> {
        self.sessions.iter().find(|session| session.id() == id)
    }

    pub fn get_mut(&mut self, id: PtyId) -> Option<&mut PtySession> {
        self.sessions
            .iter_mut()
            .find(|session| session.id() == id)
    }

    pub fn remove(&mut self, id: PtyId) -> Option<PtySession> {
        let index = self.sessions.iter().position(|session| session.id() == id)?;
        Some(self.sessions.remove(index))
    }

    pub fn sessions(&self) -> impl Iterator<Item = &PtySession> {
        self.sessions.iter()
    }

    pub fn active(&self) -> impl Iterator<Item = &PtySession> {
        self.sessions
            .iter()
            .filter(|session| session.is_active())
    }

    pub fn active_count(&self) -> usize {
        self.active().count()
    }

    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    pub fn clear(&mut self) {
        self.sessions.clear();
    }
}
