use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct HistoryFilter {
    shell: Option<String>,
    working_directory: Option<PathBuf>,
    exit_status: Option<i32>,
    contains: Option<String>,
    session_id: Option<String>,
}

impl HistoryFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shell(&self) -> Option<&str> {
        self.shell.as_deref()
    }

    pub fn set_shell(&mut self, shell: impl Into<String>) {
        self.shell = Some(shell.into());
    }

    pub fn working_directory(&self) -> Option<&PathBuf> {
        self.working_directory.as_ref()
    }

    pub fn set_working_directory(&mut self, path: impl Into<PathBuf>) {
        self.working_directory = Some(path.into());
    }

    pub fn exit_status(&self) -> Option<i32> {
        self.exit_status
    }

    pub fn set_exit_status(&mut self, status: i32) {
        self.exit_status = Some(status);
    }

    pub fn contains(&self) -> Option<&str> {
        self.contains.as_deref()
    }

    pub fn set_contains(&mut self, query: impl Into<String>) {
        self.contains = Some(query.into());
    }

    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    pub fn set_session_id(&mut self, value: impl Into<String>) {
        self.session_id = Some(value.into());
    }

    pub fn matches(
        &self,
        command: &str,
        metadata: &crate::history::HistoryMetadata,
    ) -> bool {
        if let Some(shell) = &self.shell {
            if metadata.shell() != Some(shell.as_str()) {
                return false;
            }
        }

        if let Some(path) = &self.working_directory {
            if metadata.working_directory() != Some(path) {
                return false;
            }
        }

        if let Some(status) = self.exit_status {
            if metadata.exit_status() != Some(status) {
                return false;
            }
        }

        if let Some(query) = &self.contains {
            if !command
                .to_lowercase()
                .contains(&query.to_lowercase())
            {
                return false;
            }
        }

        if let Some(session) = &self.session_id {
            if metadata.session_id() != Some(session.as_str()) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Default)]
pub struct HistoryFilterBuilder {
    filter: HistoryFilter,
}

impl HistoryFilterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shell(mut self, value: impl Into<String>) -> Self {
        self.filter.set_shell(value);
        self
    }

    pub fn working_directory(
        mut self,
        value: impl Into<PathBuf>,
    ) -> Self {
        self.filter.set_working_directory(value);
        self
    }

    pub fn exit_status(mut self, value: i32) -> Self {
        self.filter.set_exit_status(value);
        self
    }

    pub fn contains(mut self, value: impl Into<String>) -> Self {
        self.filter.set_contains(value);
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.filter.set_session_id(value);
        self
    }

    pub fn build(self) -> HistoryFilter {
        self.filter
    }
}
