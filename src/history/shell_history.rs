use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellHistoryFormat {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    PlainText,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellHistoryEntry {
    command: String,
    timestamp: Option<i64>,
    duration_seconds: Option<u64>,
    working_directory: Option<PathBuf>,
}

impl ShellHistoryEntry {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            timestamp: None,
            duration_seconds: None,
            working_directory: None,
        }
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn timestamp(&self) -> Option<i64> {
        self.timestamp
    }

    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = Some(timestamp);
    }

    pub fn duration_seconds(&self) -> Option<u64> {
        self.duration_seconds
    }

    pub fn set_duration_seconds(&mut self, duration: u64) {
        self.duration_seconds = Some(duration);
    }

    pub fn working_directory(&self) -> Option<&PathBuf> {
        self.working_directory.as_ref()
    }

    pub fn set_working_directory(&mut self, path: impl Into<PathBuf>) {
        self.working_directory = Some(path.into());
    }
}

pub trait ShellHistory {
    fn format(&self) -> ShellHistoryFormat;

    fn entries(&self) -> &[ShellHistoryEntry];

    fn import_text(
        &mut self,
        text: &str,
    ) -> Result<usize, String>;

    fn export_text(&self) -> String;

    fn clear(&mut self);
}
