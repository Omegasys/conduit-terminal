 //! History integration for custom shells.

#[derive(Debug, Clone)]
pub struct CustomHistoryEntry {
    pub command: String,
    pub timestamp: Option<i64>,
    pub working_directory: Option<String>,
    pub exit_status: Option<i32>,
}

impl CustomHistoryEntry {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            timestamp: None,
            working_directory: None,
            exit_status: None,
        }
    }
}

pub trait CustomHistory: Send + Sync {
    fn load(&self, source: &str) -> Result<Vec<CustomHistoryEntry>, String>;

    fn serialize(
        &self,
        entries: &[CustomHistoryEntry],
    ) -> Result<String, String>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PlainTextHistory;

impl CustomHistory for PlainTextHistory {
    fn load(&self, source: &str) -> Result<Vec<CustomHistoryEntry>, String> {
        Ok(source
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(CustomHistoryEntry::new)
            .collect())
    }

    fn serialize(
        &self,
        entries: &[CustomHistoryEntry],
    ) -> Result<String, String> {
        Ok(entries
            .iter()
            .map(|entry| entry.command.as_str())
            .collect::<Vec<_>>()
            .join("\n"))
    }
}
