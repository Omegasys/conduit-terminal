use std::fmt;

use crate::events::EventSource;

/// Diagnostic severity used by the CLI diagnostic interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    Trace,
    Debug,
    Information,
    Warning,
    Error,
    Critical,
}

impl DiagnosticSeverity {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Information => "information",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
        }
    }

    pub fn is_problem(&self) -> bool {
        matches!(self, Self::Warning | Self::Error | Self::Critical)
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// Actions supported by the `conduit diagnostics` command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticsCommandAction {
    Status,
    List,
    Show,
    Search,
    Clear,
    Export,
    Enable,
    Disable,
    CollectReport,
}

impl DiagnosticsCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::List => "list",
            Self::Show => "show",
            Self::Search => "search",
            Self::Clear => "clear",
            Self::Export => "export",
            Self::Enable => "enable",
            Self::Disable => "disable",
            Self::CollectReport => "collect-report",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "status" => Some(Self::Status),
            "list" => Some(Self::List),
            "show" => Some(Self::Show),
            "search" => Some(Self::Search),
            "clear" => Some(Self::Clear),
            "export" => Some(Self::Export),
            "enable" => Some(Self::Enable),
            "disable" => Some(Self::Disable),
            "collect-report" => Some(Self::CollectReport),
            _ => None,
        }
    }
}

impl fmt::Display for DiagnosticsCommandAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A diagnostic record suitable for CLI display or filtering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticRecord {
    id: u64,
    severity: DiagnosticSeverity,
    source: EventSource,
    component: String,
    message: String,
    details: Option<String>,
    resolved: bool,
}

impl DiagnosticRecord {
    pub fn new<S1, S2>(
        id: u64,
        severity: DiagnosticSeverity,
        source: EventSource,
        component: S1,
        message: S2,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id,
            severity,
            source,
            component: component.into(),
            message: message.into(),
            details: None,
            resolved: false,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    pub fn source(&self) -> &EventSource {
        &self.source
    }

    pub fn component(&self) -> &str {
        &self.component
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    pub fn resolved(&self) -> bool {
        self.resolved
    }

    pub fn set_details<S>(&mut self, details: S)
    where
        S: Into<String>,
    {
        self.details = Some(details.into());
    }

    pub fn mark_resolved(&mut self) {
        self.resolved = true;
    }

    pub fn mark_unresolved(&mut self) {
        self.resolved = false;
    }
}

/// A parsed diagnostics command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticsCommand {
    action: DiagnosticsCommandAction,
    diagnostic_id: Option<u64>,
    severity: Option<DiagnosticSeverity>,
    component: Option<String>,
    search: Option<String>,
    include_resolved: bool,
}

impl DiagnosticsCommand {
    pub fn new(action: DiagnosticsCommandAction) -> Self {
        Self {
            action,
            diagnostic_id: None,
            severity: None,
            component: None,
            search: None,
            include_resolved: false,
        }
    }

    pub fn action(&self) -> &DiagnosticsCommandAction {
        &self.action
    }

    pub fn diagnostic_id(&self) -> Option<u64> {
        self.diagnostic_id
    }

    pub fn severity(&self) -> Option<DiagnosticSeverity> {
        self.severity
    }

    pub fn component(&self) -> Option<&str> {
        self.component.as_deref()
    }

    pub fn search(&self) -> Option<&str> {
        self.search.as_deref()
    }

    pub fn include_resolved(&self) -> bool {
        self.include_resolved
    }

    pub fn set_diagnostic_id(&mut self, diagnostic_id: u64) {
        self.diagnostic_id = Some(diagnostic_id);
    }

    pub fn set_severity(&mut self, severity: DiagnosticSeverity) {
        self.severity = Some(severity);
    }

    pub fn set_component<S>(&mut self, component: S)
    where
        S: Into<String>,
    {
        self.component = Some(component.into());
    }

    pub fn set_search<S>(&mut self, search: S)
    where
        S: Into<String>,
    {
        self.search = Some(search.into());
    }

    pub fn set_include_resolved(&mut self, include_resolved: bool) {
        self.include_resolved = include_resolved;
    }

    pub fn requires_diagnostic_id(&self) -> bool {
        matches!(self.action, DiagnosticsCommandAction::Show)
    }

    pub fn applies_to(&self, diagnostic: &DiagnosticRecord) -> bool {
        if let Some(id) = self.diagnostic_id {
            if diagnostic.id() != id {
                return false;
            }
        }

        if let Some(severity) = self.severity {
            if diagnostic.severity() != severity {
                return false;
            }
        }

        if let Some(component) = self.component() {
            if diagnostic.component() != component {
                return false;
            }
        }

        if !self.include_resolved && diagnostic.resolved() {
            return false;
        }

        if let Some(search) = self.search() {
            let search = search.to_lowercase();
            let haystack = format!(
                "{} {} {}",
                diagnostic.component(),
                diagnostic.message(),
                diagnostic.details().unwrap_or_default()
            )
            .to_lowercase();

            if !haystack.contains(&search) {
                return false;
            }
        }

        true
    }
}
