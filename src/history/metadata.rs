use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryMetadata {
    timestamp: SystemTime,
    shell: Option<String>,
    working_directory: Option<PathBuf>,
    session_id: Option<String>,
    workspace_id: Option<String>,
    tab_id: Option<String>,
    pane_id: Option<String>,
    exit_status: Option<i32>,
    duration: Option<Duration>,
    hostname: Option<String>,
}

impl Default for HistoryMetadata {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            shell: None,
            working_directory: None,
            session_id: None,
            workspace_id: None,
            tab_id: None,
            pane_id: None,
            exit_status: None,
            duration: None,
            hostname: None,
        }
    }
}

impl HistoryMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn set_timestamp(&mut self, timestamp: SystemTime) {
        self.timestamp = timestamp;
    }

    pub fn shell(&self) -> Option<&str> {
        self.shell.as_deref()
    }

    pub fn set_shell(&mut self, shell: impl Into<String>) {
        self.shell = Some(shell.into());
    }

    pub fn clear_shell(&mut self) {
        self.shell = None;
    }

    pub fn working_directory(&self) -> Option<&PathBuf> {
        self.working_directory.as_ref()
    }

    pub fn set_working_directory(&mut self, path: impl Into<PathBuf>) {
        self.working_directory = Some(path.into());
    }

    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    pub fn set_session_id(&mut self, value: impl Into<String>) {
        self.session_id = Some(value.into());
    }

    pub fn workspace_id(&self) -> Option<&str> {
        self.workspace_id.as_deref()
    }

    pub fn set_workspace_id(&mut self, value: impl Into<String>) {
        self.workspace_id = Some(value.into());
    }

    pub fn tab_id(&self) -> Option<&str> {
        self.tab_id.as_deref()
    }

    pub fn set_tab_id(&mut self, value: impl Into<String>) {
        self.tab_id = Some(value.into());
    }

    pub fn pane_id(&self) -> Option<&str> {
        self.pane_id.as_deref()
    }

    pub fn set_pane_id(&mut self, value: impl Into<String>) {
        self.pane_id = Some(value.into());
    }

    pub fn exit_status(&self) -> Option<i32> {
        self.exit_status
    }

    pub fn set_exit_status(&mut self, status: i32) {
        self.exit_status = Some(status);
    }

    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = Some(duration);
    }

    pub fn hostname(&self) -> Option<&str> {
        self.hostname.as_deref()
    }

    pub fn set_hostname(&mut self, hostname: impl Into<String>) {
        self.hostname = Some(hostname.into());
    }
}
