use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandHistorySource {
    Shell,
    Conduit,
    Imported,
    Recorded,
}

#[derive(Debug, Clone)]
pub struct CommandHistoryEntry {
    id: u64,
    command: String,
    working_directory: Option<String>,
    timestamp: SystemTime,
    source: CommandHistorySource,
    exit_code: Option<i32>,
    duration: Option<Duration>,
    sensitive: bool,
}

impl CommandHistoryEntry {
    pub fn new(
        id: u64,
        command: impl Into<String>,
        source: CommandHistorySource,
    ) -> Self {
        Self {
            id,
            command: command.into(),
            working_directory: None,
            timestamp: SystemTime::now(),
            source,
            exit_code: None,
            duration: None,
            sensitive: false,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn working_directory(&self) -> Option<&str> {
        self.working_directory.as_deref()
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn source(&self) -> CommandHistorySource {
        self.source
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

    pub fn sensitive(&self) -> bool {
        self.sensitive
    }

    pub fn set_working_directory(&mut self, directory: impl Into<String>) {
        self.working_directory = Some(directory.into());
    }

    pub fn set_exit_code(&mut self, exit_code: i32) {
        self.exit_code = Some(exit_code);
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = Some(duration);
    }

    pub fn set_sensitive(&mut self, sensitive: bool) {
        self.sensitive = sensitive;
    }
}

#[derive(Debug)]
pub struct CommandHistory {
    entries: VecDeque<CommandHistoryEntry>,
    maximum_entries: usize,
    next_id: u64,
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new(10_000)
    }
}

impl CommandHistory {
    pub fn new(maximum_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            maximum_entries,
            next_id: 1,
        }
    }

    pub fn add(
        &mut self,
        command: impl Into<String>,
        source: CommandHistorySource,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let entry = CommandHistoryEntry::new(id, command, source);
        self.push(entry);

        id
    }

    pub fn push(&mut self, entry: CommandHistoryEntry) {
        if self.maximum_entries == 0 {
            return;
        }

        self.entries.push_back(entry);

        while self.entries.len() > self.maximum_entries {
            self.entries.pop_front();
        }
    }

    pub fn get(&self, id: u64) -> Option<&CommandHistoryEntry> {
        self.entries.iter().find(|entry| entry.id() == id)
    }

    pub fn latest(&self) -> Option<&CommandHistoryEntry> {
        self.entries.back()
    }

    pub fn entries(&self) -> impl DoubleEndedIterator<Item = &CommandHistoryEntry> {
        self.entries.iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn maximum_entries(&self) -> usize {
        self.maximum_entries
    }

    pub fn set_maximum_entries(&mut self, maximum_entries: usize) {
        self.maximum_entries = maximum_entries;

        while self.entries.len() > self.maximum_entries {
            self.entries.pop_front();
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn search(&self, query: &str) -> Vec<&CommandHistoryEntry> {
        let query = query.to_lowercase();

        self.entries
            .iter()
            .rev()
            .filter(|entry| entry.command().to_lowercase().contains(&query))
            .collect()
    }
}
