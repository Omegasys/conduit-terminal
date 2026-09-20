use std::collections::VecDeque;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditSeverity {
    Information,
    Notice,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventKind {
    PermissionRequested,
    PermissionGranted,
    PermissionDenied,
    RestrictionEnabled,
    RestrictionDisabled,
    ProfileActivated,
    ProfileDeactivated,
    WarningRaised,
    WarningDismissed,
    SafeModeEnabled,
    SafeModeDisabled,
    SecuritySettingChanged,
    PluginBlocked,
    ExternalCommandBlocked,
    NetworkAccessBlocked,
    FileAccessBlocked,
    SecurityError,
}

#[derive(Debug, Clone)]
pub struct AuditEntry {
    id: u64,
    timestamp: SystemTime,
    severity: AuditSeverity,
    kind: AuditEventKind,
    source: String,
    message: String,
}

impl AuditEntry {
    pub fn new(
        id: u64,
        severity: AuditSeverity,
        kind: AuditEventKind,
        source: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id,
            timestamp: SystemTime::now(),
            severity,
            kind,
            source: source.into(),
            message: message.into(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn severity(&self) -> AuditSeverity {
        self.severity
    }

    pub fn kind(&self) -> AuditEventKind {
        self.kind
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
pub struct SecurityAuditLog {
    entries: VecDeque<AuditEntry>,
    next_id: u64,
    maximum: usize,
}

impl Default for SecurityAuditLog {
    fn default() -> Self {
        Self {
            entries: VecDeque::new(),
            next_id: 1,
            maximum: 10_000,
        }
    }
}

impl SecurityAuditLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(
        &mut self,
        severity: AuditSeverity,
        kind: AuditEventKind,
        source: impl Into<String>,
        message: impl Into<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.entries.push_back(AuditEntry::new(
            id,
            severity,
            kind,
            source,
            message,
        ));

        while self.entries.len() > self.maximum {
            self.entries.pop_front();
        }

        id
    }

    pub fn get(&self, id: u64) -> Option<&AuditEntry> {
        self.entries.iter().find(|entry| entry.id() == id)
    }

    pub fn entries(&self) -> impl Iterator<Item = &AuditEntry> {
        self.entries.iter()
    }

    pub fn critical(&self) -> impl Iterator<Item = &AuditEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.severity() == AuditSeverity::Critical)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &AuditEntry> {
        self.entries.iter().filter(|entry| {
            matches!(
                entry.severity(),
                AuditSeverity::Warning | AuditSeverity::Critical
            )
        })
    }

    pub fn by_kind(
        &self,
        kind: AuditEventKind,
    ) -> impl Iterator<Item = &AuditEntry> {
        self.entries
            .iter()
            .filter(move |entry| entry.kind() == kind)
    }

    pub fn by_source<'a>(
        &'a self,
        source: &'a str,
    ) -> impl Iterator<Item = &'a AuditEntry> {
        self.entries
            .iter()
            .filter(move |entry| entry.source() == source)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn set_maximum(&mut self, maximum: usize) {
        self.maximum = maximum.max(1);

        while self.entries.len() > self.maximum {
            self.entries.pop_front();
        }
    }

    pub fn maximum(&self) -> usize {
        self.maximum
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
