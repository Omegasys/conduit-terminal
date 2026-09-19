use std::collections::VecDeque;
use std::time::SystemTime;

/// Severity of a diagnostic message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// A diagnostic message shown or recorded by the GUI.
#[derive(Debug, Clone)]
pub struct DiagnosticMessage {
    level: DiagnosticLevel,
    source: String,
    message: String,
    timestamp: SystemTime,
}

impl DiagnosticMessage {
    pub fn new(
        level: DiagnosticLevel,
        source: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            level,
            source: source.into(),
            message: message.into(),
            timestamp: SystemTime::now(),
        }
    }

    pub fn level(&self) -> DiagnosticLevel {
        self.level
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }
}

/// GUI diagnostic/log viewer.
#[derive(Debug)]
pub struct DiagnosticsManager {
    messages: VecDeque<DiagnosticMessage>,
    max_messages: usize,
    minimum_level: DiagnosticLevel,
    enabled: bool,
}

impl Default for DiagnosticsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticsManager {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages: 1000,
            minimum_level: DiagnosticLevel::Trace,
            enabled: true,
        }
    }

    pub fn record(&mut self, message: DiagnosticMessage) {
        if !self.enabled || message.level() < self.minimum_level {
            return;
        }

        self.messages.push_back(message);

        while self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }

    pub fn trace(&mut self, source: impl Into<String>, message: impl Into<String>) {
        self.record(DiagnosticMessage::new(
            DiagnosticLevel::Trace,
            source,
            message,
        ));
    }

    pub fn debug(&mut self, source: impl Into<String>, message: impl Into<String>) {
        self.record(DiagnosticMessage::new(
            DiagnosticLevel::Debug,
            source,
            message,
        ));
    }

    pub fn info(&mut self, source: impl Into<String>, message: impl Into<String>) {
        self.record(DiagnosticMessage::new(
            DiagnosticLevel::Info,
            source,
            message,
        ));
    }

    pub fn warning(&mut self, source: impl Into<String>, message: impl Into<String>) {
        self.record(DiagnosticMessage::new(
            DiagnosticLevel::Warning,
            source,
            message,
        ));
    }

    pub fn error(&mut self, source: impl Into<String>, message: impl Into<String>) {
        self.record(DiagnosticMessage::new(
            DiagnosticLevel::Error,
            source,
            message,
        ));
    }

    pub fn critical(&mut self, source: impl Into<String>, message: impl Into<String>) {
        self.record(DiagnosticMessage::new(
            DiagnosticLevel::Critical,
            source,
            message,
        ));
    }

    pub fn messages(&self) -> impl Iterator<Item = &DiagnosticMessage> {
        self.messages.iter()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn set_max_messages(&mut self, max: usize) {
        self.max_messages = max;

        while self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }

    pub fn set_minimum_level(&mut self, level: DiagnosticLevel) {
        self.minimum_level = level;
    }

    pub fn minimum_level(&self) -> DiagnosticLevel {
        self.minimum_level
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}
