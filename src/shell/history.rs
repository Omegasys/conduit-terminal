use std::{
    path::PathBuf,
    time::SystemTime,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryEntry {
    id: u64,
    command: String,
    directory: Option<PathBuf>,
    exit_code: Option<i32>,
    timestamp: SystemTime,
}

impl HistoryEntry {
    pub fn new<S: Into<String>>(id: u64, command: S) -> Self {
        Self {
            id,
            command: command.into(),
            directory: None,
            exit_code: None,
            timestamp: SystemTime::now(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn directory(&self) -> Option<&PathBuf> {
        self.directory.as_ref()
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn set_directory(&mut self, directory: PathBuf) {
        self.directory = Some(directory);
    }

    pub fn set_exit_code(&mut self, exit_code: i32) {
        self.exit_code = Some(exit_code);
    }
}

#[derive(Clone, Debug)]
pub struct ShellHistory {
    entries: Vec<HistoryEntry>,
    next_id: u64,
    maximum_entries: usize,
}

impl ShellHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
            maximum_entries: 10_000,
        }
    }

    pub fn add<S: Into<String>>(
        &mut self,
        command: S,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.entries.push(HistoryEntry::new(id, command));

        self.trim();

        id
    }

    pub fn add_entry(
        &mut self,
        entry: HistoryEntry,
    ) {
        self.next_id = self.next_id.max(entry.id() + 1);

        self.entries.push(entry);

        self.trim();
    }

    pub fn get(&self, id: u64) -> Option<&HistoryEntry> {
        self.entries.iter().find(|entry| entry.id() == id)
    }

    pub fn get_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut HistoryEntry> {
        self.entries
            .iter_mut()
            .find(|entry| entry.id() == id)
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn latest(&self) -> Option<&HistoryEntry> {
        self.entries.last()
    }

    pub fn search<'a>(
        &'a self,
        query: &str,
    ) -> Vec<&'a HistoryEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry
                    .command()
                    .contains(query)
            })
            .collect()
    }

    pub fn set_maximum_entries(
        &mut self,
        maximum: usize,
    ) {
        self.maximum_entries = maximum;
        self.trim();
    }

    pub fn maximum_entries(&self) -> usize {
        self.maximum_entries
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

    fn trim(&mut self) {
        if self.entries.len() > self.maximum_entries {
            let excess =
                self.entries.len() - self.maximum_entries;

            self.entries.drain(0..excess);
        }
    }
}

impl Default for ShellHistory {
    fn default() -> Self {
        Self::new()
    }
}
