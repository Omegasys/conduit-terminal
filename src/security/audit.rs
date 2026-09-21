use std::time::SystemTime;

use super::permissions::SecurityPermission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditResult {
    Allowed,
    Denied,
    ConfirmationRequired,
}

#[derive(Debug, Clone)]
pub struct AuditEntry {
    timestamp: SystemTime,
    permission: SecurityPermission,
    result: AuditResult,
    resource: Option<String>,
    reason: Option<String>,
}

impl AuditEntry {
    pub fn new(
        permission: SecurityPermission,
        result: AuditResult,
    ) -> Self {
        Self {
            timestamp: SystemTime::now(),
            permission,
            result,
            resource: None,
            reason: None,
        }
    }

    pub fn with_resource(
        mut self,
        resource: impl Into<String>,
    ) -> Self {
        self.resource = Some(resource.into());
        self
    }

    pub fn with_reason(
        mut self,
        reason: impl Into<String>,
    ) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn permission(&self) -> SecurityPermission {
        self.permission
    }

    pub fn result(&self) -> AuditResult {
        self.result
    }

    pub fn resource(&self) -> Option<&str> {
        self.resource.as_deref()
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}

#[derive(Debug)]
pub struct SecurityAuditLog {
    entries: Vec<AuditEntry>,
    maximum_entries: usize,
    enabled: bool,
}

impl Default for SecurityAuditLog {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityAuditLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            maximum_entries: 1_000,
            enabled: true,
        }
    }

    pub fn record(
        &mut self,
        entry: AuditEntry,
    ) {
        if !self.enabled {
            return;
        }

        self.entries.push(entry);

        if self.entries.len() > self.maximum_entries {
            let excess = self.entries.len() - self.maximum_entries;

            self.entries.drain(0..excess);
        }
    }

    pub fn record_decision(
        &mut self,
        permission: SecurityPermission,
        result: AuditResult,
        resource: Option<&str>,
        reason: Option<&str>,
    ) {
        let mut entry = AuditEntry::new(permission, result);

        if let Some(resource) = resource {
            entry = entry.with_resource(resource);
        }

        if let Some(reason) = reason {
            entry = entry.with_reason(reason);
        }

        self.record(entry);
    }

    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn set_enabled(
        &mut self,
        enabled: bool,
    ) {
        self.enabled = enabled;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_maximum_entries(
        &mut self,
        maximum: usize,
    ) {
        self.maximum_entries = maximum.max(1);

        if self.entries.len() > self.maximum_entries {
            let excess = self.entries.len() - self.maximum_entries;

            self.entries.drain(0..excess);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
