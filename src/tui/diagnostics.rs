use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    Trace,
    Debug,
    Information,
    Warning,
    Error,
    Critical,
}

impl DiagnosticLevel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Trace => "Trace",
            Self::Debug => "Debug",
            Self::Information => "Info",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Critical => "Critical",
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error | Self::Critical)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSource {
    Core,
    Gui,
    Tui,
    Cli,
    Terminal,
    Configuration,
    Resource,
    Plugin,
    Security,
    Session,
    Renderer,
    External,
}

pub struct DiagnosticEntry {
    id: u64,
    level: DiagnosticLevel,
    source: DiagnosticSource,
    message: String,
    details: Option<String>,
    timestamp: Instant,
    resolved: bool,
}

impl DiagnosticEntry {
    pub fn new(
        id: u64,
        level: DiagnosticLevel,
        source: DiagnosticSource,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id,
            level,
            source,
            message: message.into(),
            details: None,
            timestamp: Instant::now(),
            resolved: false,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn level(&self) -> DiagnosticLevel {
        self.level
    }

    pub fn source(&self) -> DiagnosticSource {
        self.source
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    pub fn set_details(&mut self, details: Option<String>) {
        self.details = details;
    }

    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }

    pub fn resolved(&self) -> bool {
        self.resolved
    }

    pub fn resolve(&mut self) {
        self.resolved = true;
    }

    pub fn reopen(&mut self) {
        self.resolved = false;
    }
}

pub struct TuiDiagnosticsView {
    entries: Vec<DiagnosticEntry>,
    next_id: u64,
    selected: Option<u64>,
    minimum_level: DiagnosticLevel,
    show_resolved: bool,
    max_entries: usize,
}

impl Default for TuiDiagnosticsView {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiDiagnosticsView {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
            selected: None,
            minimum_level: DiagnosticLevel::Trace,
            show_resolved: true,
            max_entries: 1024,
        }
    }

    pub fn add(
        &mut self,
        level: DiagnosticLevel,
        source: DiagnosticSource,
        message: impl Into<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        self.entries
            .push(DiagnosticEntry::new(id, level, source, message));

        while self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }

        if self.selected.is_none() {
            self.selected = Some(id);
        }

        id
    }

    pub fn add_entry(&mut self, entry: DiagnosticEntry) {
        let id = entry.id();

        self.entries.push(entry);

        while self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }

        if self.selected.is_none() {
            self.selected = Some(id);
        }
    }

    pub fn get(&self, id: u64) -> Option<&DiagnosticEntry> {
        self.entries.iter().find(|entry| entry.id() == id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut DiagnosticEntry> {
        self.entries.iter_mut().find(|entry| entry.id() == id)
    }

    pub fn select(&mut self, id: u64) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        self.selected = Some(id);
        true
    }

    pub fn selected(&self) -> Option<&DiagnosticEntry> {
        self.selected.and_then(|id| self.get(id))
    }

    pub fn entries(&self) -> &[DiagnosticEntry] {
        &self.entries
    }

    pub fn visible_entries(&self) -> Vec<&DiagnosticEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.level() >= self.minimum_level
                    && (self.show_resolved || !entry.resolved())
            })
            .collect()
    }

    pub fn set_minimum_level(&mut self, level: DiagnosticLevel) {
        self.minimum_level = level;
    }

    pub fn minimum_level(&self) -> DiagnosticLevel {
        self.minimum_level
    }

    pub fn show_resolved(&self) -> bool {
        self.show_resolved
    }

    pub fn set_show_resolved(&mut self, show: bool) {
        self.show_resolved = show;
    }

    pub fn resolve(&mut self, id: u64) -> bool {
        if let Some(entry) = self.get_mut(id) {
            entry.resolve();
            true
        } else {
            false
        }
    }

    pub fn unresolved_count(&self) -> usize {
        self.entries.iter().filter(|entry| !entry.resolved()).count()
    }

    pub fn error_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.level().is_error() && !entry.resolved())
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| {
                entry.level() == DiagnosticLevel::Warning && !entry.resolved()
            })
            .count()
    }

    pub fn set_max_entries(&mut self, max_entries: usize) {
        self.max_entries = max_entries.max(1);

        while self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn max_entries(&self) -> usize {
        self.max_entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.selected = None;
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
