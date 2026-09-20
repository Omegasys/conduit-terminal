use std::collections::VecDeque;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct CommandHistoryEntry {
    command_id: String,
    label: String,
    executed_at: SystemTime,
    execution_count: u32,
}

impl CommandHistoryEntry {
    pub fn new(
        command_id: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            command_id: command_id.into(),
            label: label.into(),
            executed_at: SystemTime::now(),
            execution_count: 1,
        }
    }

    pub fn command_id(&self) -> &str {
        &self.command_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn executed_at(&self) -> SystemTime {
        self.executed_at
    }

    pub fn execution_count(&self) -> u32 {
        self.execution_count
    }

    fn record_execution(&mut self) {
        self.executed_at = SystemTime::now();
        self.execution_count = self.execution_count.saturating_add(1);
    }
}

#[derive(Debug)]
pub struct CommandHistory {
    entries: VecDeque<CommandHistoryEntry>,
    max_entries: usize,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries: 100,
        }
    }

    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries: max_entries.max(1),
        }
    }

    pub fn record(
        &mut self,
        command_id: impl Into<String>,
        label: impl Into<String>,
    ) {
        let command_id = command_id.into();
        let label = label.into();

        if let Some(index) = self
            .entries
            .iter()
            .position(|entry| entry.command_id() == command_id)
        {
            if let Some(mut entry) = self.entries.remove(index) {
                entry.record_execution();
                self.entries.push_front(entry);
            }

            return;
        }

        self.entries
            .push_front(CommandHistoryEntry::new(command_id, label));

        self.trim();
    }

    pub fn latest(&self) -> Option<&CommandHistoryEntry> {
        self.entries.front()
    }

    pub fn get(&self, command_id: &str) -> Option<&CommandHistoryEntry> {
        self.entries
            .iter()
            .find(|entry| entry.command_id() == command_id)
    }

    pub fn entries(&self) -> impl Iterator<Item = &CommandHistoryEntry> {
        self.entries.iter()
    }

    pub fn recent(&self, count: usize) -> Vec<&CommandHistoryEntry> {
        self.entries.iter().take(count).collect()
    }

    pub fn set_max_entries(&mut self, max_entries: usize) {
        self.max_entries = max_entries.max(1);
        self.trim();
    }

    pub fn max_entries(&self) -> usize {
        self.max_entries
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
        while self.entries.len() > self.max_entries {
            self.entries.pop_back();
        }
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}
