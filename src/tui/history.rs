use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryEntryKind {
    Command,
    Session,
    Workspace,
    Search,
    Configuration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryFilter {
    All,
    Commands,
    Sessions,
    Workspaces,
    Searches,
    Configuration,
}

impl HistoryFilter {
    fn matches(&self, kind: HistoryEntryKind) -> bool {
        match self {
            Self::All => true,
            Self::Commands => kind == HistoryEntryKind::Command,
            Self::Sessions => kind == HistoryEntryKind::Session,
            Self::Workspaces => kind == HistoryEntryKind::Workspace,
            Self::Searches => kind == HistoryEntryKind::Search,
            Self::Configuration => kind == HistoryEntryKind::Configuration,
        }
    }
}

pub struct HistoryEntry {
    id: u64,
    kind: HistoryEntryKind,
    text: String,
    cwd: Option<String>,
    shell: Option<String>,
    exit_code: Option<i32>,
    duration_ms: Option<u64>,
    timestamp: Instant,
    pinned: bool,
}

impl HistoryEntry {
    pub fn new(
        id: u64,
        kind: HistoryEntryKind,
        text: impl Into<String>,
    ) -> Self {
        Self {
            id,
            kind,
            text: text.into(),
            cwd: None,
            shell: None,
            exit_code: None,
            duration_ms: None,
            timestamp: Instant::now(),
            pinned: false,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn kind(&self) -> HistoryEntryKind {
        self.kind
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn cwd(&self) -> Option<&str> {
        self.cwd.as_deref()
    }

    pub fn set_cwd(&mut self, cwd: Option<String>) {
        self.cwd = cwd;
    }

    pub fn shell(&self) -> Option<&str> {
        self.shell.as_deref()
    }

    pub fn set_shell(&mut self, shell: Option<String>) {
        self.shell = shell;
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn set_exit_code(&mut self, exit_code: Option<i32>) {
        self.exit_code = exit_code;
    }

    pub fn duration_ms(&self) -> Option<u64> {
        self.duration_ms
    }

    pub fn set_duration_ms(&mut self, duration_ms: Option<u64>) {
        self.duration_ms = duration_ms;
    }

    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }

    pub fn pinned(&self) -> bool {
        self.pinned
    }

    pub fn set_pinned(&mut self, pinned: bool) {
        self.pinned = pinned;
    }

    pub fn toggle_pinned(&mut self) {
        self.pinned = !self.pinned;
    }
}

pub struct TuiHistoryView {
    entries: Vec<HistoryEntry>,
    next_id: u64,
    filter: HistoryFilter,
    search: String,
    selected: Option<u64>,
    max_entries: usize,
    show_pinned_only: bool,
}

impl Default for TuiHistoryView {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiHistoryView {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
            filter: HistoryFilter::All,
            search: String::new(),
            selected: None,
            max_entries: 4096,
            show_pinned_only: false,
        }
    }

    pub fn add(
        &mut self,
        kind: HistoryEntryKind,
        text: impl Into<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        self.entries.push(HistoryEntry::new(id, kind, text));

        while self.entries.len() > self.max_entries {
            if let Some(index) = self.entries.iter().position(|entry| !entry.pinned()) {
                self.entries.remove(index);
            } else {
                break;
            }
        }

        if self.selected.is_none() {
            self.selected = Some(id);
        }

        id
    }

    pub fn add_entry(&mut self, entry: HistoryEntry) {
        let id = entry.id();

        self.entries.push(entry);

        while self.entries.len() > self.max_entries {
            if let Some(index) = self.entries.iter().position(|entry| !entry.pinned()) {
                self.entries.remove(index);
            } else {
                break;
            }
        }

        if self.selected.is_none() {
            self.selected = Some(id);
        }
    }

    pub fn get(&self, id: u64) -> Option<&HistoryEntry> {
        self.entries.iter().find(|entry| entry.id() == id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut HistoryEntry> {
        self.entries.iter_mut().find(|entry| entry.id() == id)
    }

    pub fn remove(&mut self, id: u64) -> Option<HistoryEntry> {
        let index = self.entries.iter().position(|entry| entry.id() == id)?;
        let removed = self.entries.remove(index);

        if self.selected == Some(id) {
            self.selected = self.entries.last().map(|entry| entry.id());
        }

        Some(removed)
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn visible_entries(&self) -> Vec<&HistoryEntry> {
        let query = self.search.trim().to_lowercase();

        self.entries
            .iter()
            .rev()
            .filter(|entry| self.filter.matches(entry.kind()))
            .filter(|entry| !self.show_pinned_only || entry.pinned())
            .filter(|entry| {
                query.is_empty() || entry.text().to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn filter(&self) -> HistoryFilter {
        self.filter
    }

    pub fn set_filter(&mut self, filter: HistoryFilter) {
        self.filter = filter;
        self.selected = None;
    }

    pub fn search(&self) -> &str {
        &self.search
    }

    pub fn set_search(&mut self, search: impl Into<String>) {
        self.search = search.into();
        self.selected = None;
    }

    pub fn clear_search(&mut self) {
        self.search.clear();
        self.selected = None;
    }

    pub fn show_pinned_only(&self) -> bool {
        self.show_pinned_only
    }

    pub fn set_show_pinned_only(&mut self, show: bool) {
        self.show_pinned_only = show;
        self.selected = None;
    }

    pub fn select(&mut self, id: u64) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        self.selected = Some(id);
        true
    }

    pub fn selected(&self) -> Option<&HistoryEntry> {
        self.selected.and_then(|id| self.get(id))
    }

    pub fn selected_id(&self) -> Option<u64> {
        self.selected
    }

    pub fn toggle_pin(&mut self, id: u64) -> bool {
        if let Some(entry) = self.get_mut(id) {
            entry.toggle_pinned();
            true
        } else {
            false
        }
    }

    pub fn set_max_entries(&mut self, max_entries: usize) {
        self.max_entries = max_entries.max(1);

        while self.entries.len() > self.max_entries {
            if let Some(index) = self.entries.iter().position(|entry| !entry.pinned()) {
                self.entries.remove(index);
            } else {
                break;
            }
        }
    }

    pub fn max_entries(&self) -> usize {
        self.max_entries
    }

    pub fn pinned_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.pinned())
            .count()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.retain(|entry| entry.pinned());
        self.selected = None;
    }

    pub fn clear_all(&mut self) {
        self.entries.clear();
        self.selected = None;
    }
}
