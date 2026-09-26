use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Default)]
pub struct RecordingMetadata {
    pub created_at: Option<u64>,
    pub application: String,
    pub application_version: Option<String>,
    pub shell: Option<String>,
    pub working_directory: Option<PathBuf>,
    pub hostname: Option<String>,
    pub session_id: Option<String>,
    pub workspace_id: Option<String>,
    pub tab_id: Option<String>,
    pub pane_id: Option<String>,
    pub columns: u16,
    pub rows: u16,
    pub description: Option<String>,
}

impl RecordingMetadata {
    pub fn new() -> Self {
        Self {
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|duration| duration.as_secs()),
            application: "Conduit".to_owned(),
            application_version: None,
            shell: None,
            working_directory: None,
            hostname: None,
            session_id: None,
            workspace_id: None,
            tab_id: None,
            pane_id: None,
            columns: 80,
            rows: 24,
            description: None,
        }
    }

    pub fn set_shell(&mut self, shell: impl Into<String>) {
        self.shell = Some(shell.into());
    }

    pub fn set_working_directory(
        &mut self,
        path: impl Into<PathBuf>,
    ) {
        self.working_directory = Some(path.into());
    }

    pub fn set_hostname(&mut self, hostname: impl Into<String>) {
        self.hostname = Some(hostname.into());
    }

    pub fn set_session_id(&mut self, id: impl Into<String>) {
        self.session_id = Some(id.into());
    }

    pub fn set_workspace_id(&mut self, id: impl Into<String>) {
        self.workspace_id = Some(id.into());
    }

    pub fn set_tab_id(&mut self, id: impl Into<String>) {
        self.tab_id = Some(id.into());
    }

    pub fn set_pane_id(&mut self, id: impl Into<String>) {
        self.pane_id = Some(id.into());
    }

    pub fn set_size(&mut self, columns: u16, rows: u16) {
        self.columns = columns;
        self.rows = rows;
    }

    pub fn set_description(
        &mut self,
        description: impl Into<String>,
    ) {
        self.description = Some(description.into());
    }
}
