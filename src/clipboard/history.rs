use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct ClipboardHistoryEntry {
    pub id: u64,
    pub text: String,
    pub timestamp: u64,
}

impl ClipboardHistoryEntry {
    pub fn new(id: u64, text: impl Into<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or_default();

        Self {
            id,
            text: text.into(),
            timestamp,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct ClipboardHistory {
    entries: Vec<ClipboardHistoryEntry>,
    maximum_entries: usize,
    next_id: u64,
}

impl Default for ClipboardHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            maximum_entries: 100,
            next_id: 1,
        }
    }

    pub fn with_capacity(maximum_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            maximum_entries,
            next_id: 1,
        }
    }

    pub fn push(&mut self, text: impl Into<String>) -> u64 {
        let text = text.into();

        if text.is_empty() {
            return 0;
        }

        if self
            .entries
            .first()
            .map(|entry| entry.text == text)
            .unwrap_or(false)
        {
            return self.entries[0].id;
        }

        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);

        self.entries
            .insert(0, ClipboardHistoryEntry::new(id, text));

        while self.entries.len() > self.maximum_entries {
            self.entries.pop();
        }

        id
    }

    pub fn get(&self, id: u64) -> Option<&ClipboardHistoryEntry> {
        self.entries.iter().find(|entry| entry.id == id)
    }

    pub fn latest(&self) -> Option<&ClipboardHistoryEntry> {
        self.entries.first()
    }

    pub fn entries(&self) -> &[ClipboardHistoryEntry] {
        &self.entries
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
            self.entries.pop();
        }
    }

    pub fn remove(&mut self, id: u64) -> bool {
        if let Some(index) = self.entries.iter().position(|entry| entry.id == id) {
            self.entries.remove(index);
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
