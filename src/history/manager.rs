use super::database::{
    HistoryDatabase,
    HistoryDatabaseEntry,
};
use super::filtering::HistoryFilter;
use super::indexing::HistoryIndex;
use super::metadata::HistoryMetadata;
use super::privacy::HistoryPrivacyPolicy;
use super::redaction::HistoryRedactionPolicy;
use super::search::{
    HistorySearch,
    HistorySearchOptions,
    HistorySearchResult,
};
use super::storage::{
    HistoryStorage,
    HistoryStorageError,
};

#[derive(Debug)]
pub struct HistoryManager {
    database: HistoryDatabase,
    index: HistoryIndex,
    search: HistorySearch,
    storage: HistoryStorage,
    privacy: HistoryPrivacyPolicy,
    redaction: HistoryRedactionPolicy,
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            database: HistoryDatabase::new(),
            index: HistoryIndex::new(),
            search: HistorySearch::new(),
            storage: HistoryStorage::default(),
            privacy: HistoryPrivacyPolicy::default(),
            redaction: HistoryRedactionPolicy::default(),
        }
    }

    pub fn database(&self) -> &HistoryDatabase {
        &self.database
    }

    pub fn database_mut(&mut self) -> &mut HistoryDatabase {
        &mut self.database
    }

    pub fn storage(&self) -> &HistoryStorage {
        &self.storage
    }

    pub fn privacy(&self) -> &HistoryPrivacyPolicy {
        &self.privacy
    }

    pub fn privacy_mut(&mut self) -> &mut HistoryPrivacyPolicy {
        &mut self.privacy
    }

    pub fn redaction(&self) -> &HistoryRedactionPolicy {
        &self.redaction
    }

    pub fn redaction_mut(&mut self) -> &mut HistoryRedactionPolicy {
        &mut self.redaction
    }

    pub fn add(
        &mut self,
        command: impl Into<String>,
        mut metadata: HistoryMetadata,
    ) -> Option<u64> {
        if !self.privacy.should_store_commands() {
            return None;
        }

        let command = self.redaction.redact(&command.into());

        if self.privacy.should_store_metadata() {
            if !self.privacy.store_working_directory() {
                metadata = remove_working_directory(metadata);
            }

            if !self.privacy.store_shell() {
                metadata.clear_shell();
            }
        } else {
            metadata = HistoryMetadata::new();
        }

        let id = self.database.insert(command.clone(), metadata);

        self.index
            .index(id, self.database.len() - 1, &command);

        Some(id)
    }

    pub fn get(&self, id: u64) -> Option<&HistoryDatabaseEntry> {
        self.database.get(id)
    }

    pub fn remove(&mut self, id: u64) -> Option<HistoryDatabaseEntry> {
        let result = self.database.remove(id);

        if result.is_some() {
            self.index.remove(id);
            self.rebuild_index();
        }

        result
    }

    pub fn clear(&mut self) {
        self.database.clear();
        self.index.clear();
    }

    pub fn search(
        &self,
        options: &HistorySearchOptions,
    ) -> Vec<HistorySearchResult> {
        self.search.search(
            self.database.entries(),
            options,
        )
    }

    pub fn filter(
        &self,
        filter: &HistoryFilter,
    ) -> Vec<&HistoryDatabaseEntry> {
        self.database
            .entries()
            .iter()
            .filter(|entry| {
                filter.matches(
                    entry.command(),
                    entry.metadata(),
                )
            })
            .collect()
    }

    pub fn rebuild_index(&mut self) {
        let entries = self
            .database
            .entries()
            .iter()
            .map(|entry| {
                (
                    entry.id(),
                    entry.command().to_string(),
                )
            })
            .collect::<Vec<_>>();

        self.index.rebuild(entries);
    }

    pub fn load(&mut self) -> Result<(), HistoryStorageError> {
        let data = self.storage.read()?;

        if data.is_empty() {
            return Ok(());
        }

        // The database format intentionally remains internal here.
        // A future persistence backend can replace this without
        // changing the public history API.
        let text = String::from_utf8(data)
            .map_err(|error| {
                HistoryStorageError::InvalidFormat(
                    error.to_string(),
                )
            })?;

        self.database.clear();

        for line in text.lines() {
            if line.is_empty() {
                continue;
            }

            self.database
                .insert(line, HistoryMetadata::new());
        }

        self.rebuild_index();

        Ok(())
    }

    pub fn save(&self) -> Result<(), HistoryStorageError> {
        let mut output = String::new();

        for entry in self.database.entries() {
            output.push_str(entry.command());
            output.push('\n');
        }

        self.storage.write(output.as_bytes())
    }

    pub fn entry_count(&self) -> usize {
        self.database.len()
    }
}

fn remove_working_directory(
    mut metadata: HistoryMetadata,
) -> HistoryMetadata {
    // HistoryMetadata deliberately does not expose a generic
    // clear method for every field. Rebuild the privacy-safe
    // representation here.
    let mut safe = HistoryMetadata::new();

    safe.set_timestamp(metadata.timestamp());

    if let Some(shell) = metadata.shell() {
        safe.set_shell(shell);
    }

    if let Some(session) = metadata.session_id() {
        safe.set_session_id(session);
    }

    if let Some(workspace) = metadata.workspace_id() {
        safe.set_workspace_id(workspace);
    }

    if let Some(tab) = metadata.tab_id() {
        safe.set_tab_id(tab);
    }

    if let Some(pane) = metadata.pane_id() {
        safe.set_pane_id(pane);
    }

    if let Some(status) = metadata.exit_status() {
        safe.set_exit_status(status);
    }

    if let Some(duration) = metadata.duration() {
        safe.set_duration(duration);
    }

    if let Some(hostname) = metadata.hostname() {
        safe.set_hostname(hostname);
    }

    metadata = safe;
    metadata
}
