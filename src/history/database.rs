use super::metadata::HistoryMetadata;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryDatabaseEntry {
    id: u64,
    command: String,
    metadata: HistoryMetadata,
}

impl HistoryDatabaseEntry {
    pub fn new(
        id: u64,
        command: impl Into<String>,
        metadata: HistoryMetadata,
    ) -> Self {
        Self {
            id,
            command: command.into(),
            metadata,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn metadata(&self) -> &HistoryMetadata {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut HistoryMetadata {
        &mut self.metadata
    }
}

#[derive(Debug, Default)]
pub struct HistoryDatabase {
    entries: Vec<HistoryDatabaseEntry>,
    next_id: u64,
}

impl HistoryDatabase {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
        }
    }

    pub fn insert(
        &mut self,
        command: impl Into<String>,
        metadata: HistoryMetadata,
    ) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        self.entries.push(HistoryDatabaseEntry::new(
            id,
            command,
            metadata,
        ));

        id
    }

    pub fn get(&self, id: u64) -> Option<&HistoryDatabaseEntry> {
        self.entries.iter().find(|entry| entry.id() == id)
    }

    pub fn get_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut HistoryDatabaseEntry> {
        self.entries
            .iter_mut()
            .find(|entry| entry.id() == id)
    }

    pub fn remove(&mut self, id: u64) -> Option<HistoryDatabaseEntry> {
        let index = self.entries.iter().position(|entry| entry.id() == id)?;
        Some(self.entries.remove(index))
    }

    pub fn entries(&self) -> &[HistoryDatabaseEntry] {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut [HistoryDatabaseEntry] {
        &mut self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.next_id = 1;
    }

    pub fn search_command(&self, query: &str) -> Vec<&HistoryDatabaseEntry> {
        let query = query.to_lowercase();

        self.entries
            .iter()
            .filter(|entry| entry.command().to_lowercase().contains(&query))
            .collect()
    }
}
