use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use super::sensitive::SensitiveCommandResult;

#[derive(Debug, Clone)]
pub struct CommandDatabaseEntry {
    pub id: u64,
    pub command: String,
    pub working_directory: Option<String>,
    pub timestamp: SystemTime,
    pub exit_code: Option<i32>,
    pub duration: Option<Duration>,
    pub sensitive: bool,
}

impl CommandDatabaseEntry {
    pub fn new(id: u64, command: impl Into<String>) -> Self {
        Self {
            id,
            command: command.into(),
            working_directory: None,
            timestamp: SystemTime::now(),
            exit_code: None,
            duration: None,
            sensitive: false,
        }
    }

    pub fn with_sensitive_result(
        mut self,
        result: &SensitiveCommandResult,
    ) -> Self {
        self.sensitive = result.is_sensitive();

        if self.sensitive {
            self.command = result.redacted_command().to_string();
        }

        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CommandDatabaseStats {
    pub total_entries: usize,
    pub sensitive_entries: usize,
    pub successful_entries: usize,
    pub failed_entries: usize,
}

#[derive(Debug, Default)]
pub struct CommandDatabase {
    entries: HashMap<u64, CommandDatabaseEntry>,
}

impl CommandDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, entry: CommandDatabaseEntry) {
        self.entries.insert(entry.id, entry);
    }

    pub fn get(&self, id: u64) -> Option<&CommandDatabaseEntry> {
        self.entries.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut CommandDatabaseEntry> {
        self.entries.get_mut(&id)
    }

    pub fn remove(&mut self, id: u64) -> Option<CommandDatabaseEntry> {
        self.entries.remove(&id)
    }

    pub fn search(&self, query: &str) -> Vec<&CommandDatabaseEntry> {
        let query = query.to_lowercase();

        self.entries
            .values()
            .filter(|entry| {
                entry.command.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn by_working_directory(
        &self,
        directory: &str,
    ) -> Vec<&CommandDatabaseEntry> {
        self.entries
            .values()
            .filter(|entry| {
                entry.working_directory.as_deref() == Some(directory)
            })
            .collect()
    }

    pub fn stats(&self) -> CommandDatabaseStats {
        let mut stats = CommandDatabaseStats {
            total_entries: self.entries.len(),
            ..Default::default()
        };

        for entry in self.entries.values() {
            if entry.sensitive {
                stats.sensitive_entries += 1;
            }

            match entry.exit_code {
                Some(0) => stats.successful_entries += 1,
                Some(_) => stats.failed_entries += 1,
                None => {}
            }
        }

        stats
    }

    pub fn entries(&self) -> impl Iterator<Item = &CommandDatabaseEntry> {
        self.entries.values()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
