use std::time::{Duration, SystemTime};

use super::configuration::ConfigurationDiagnostics;
use super::gpu::GpuDiagnostics;
use super::memory::MemoryDiagnostics;
use super::performance::PerformanceDiagnostics;
use super::plugins::PluginDiagnostics;
use super::protocols::ProtocolDiagnostics;
use super::pty::PtyDiagnostics;
use super::renderer::RendererDiagnostics;
use super::resources::ResourceDiagnostics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticHealth {
    Healthy,
    Warning,
    Degraded,
    Error,
    Unknown,
}

impl Default for DiagnosticHealth {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticStatus {
    pub name: String,
    pub health: DiagnosticHealth,
    pub message: String,
}

impl DiagnosticStatus {
    pub fn healthy(
        name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            health: DiagnosticHealth::Healthy,
            message: message.into(),
        }
    }

    pub fn warning(
        name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            health: DiagnosticHealth::Warning,
            message: message.into(),
        }
    }

    pub fn error(
        name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            health: DiagnosticHealth::Error,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticsSnapshot {
    pub timestamp: SystemTime,
    pub uptime: Duration,

    pub performance: PerformanceDiagnostics,
    pub renderer: RendererDiagnostics,
    pub protocols: ProtocolDiagnostics,
    pub configuration: ConfigurationDiagnostics,
    pub resources: ResourceDiagnostics,
    pub plugins: PluginDiagnostics,
    pub memory: MemoryDiagnostics,
    pub gpu: GpuDiagnostics,
    pub pty: PtyDiagnostics,

    pub statuses: Vec<DiagnosticStatus>,
}

impl Default for DiagnosticsSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticsSnapshot {
    pub fn new() -> Self {
        Self {
            timestamp: SystemTime::now(),
            uptime: Duration::ZERO,

            performance: PerformanceDiagnostics::default(),
            renderer: RendererDiagnostics::default(),
            protocols: ProtocolDiagnostics::default(),
            configuration: ConfigurationDiagnostics::default(),
            resources: ResourceDiagnostics::default(),
            plugins: PluginDiagnostics::default(),
            memory: MemoryDiagnostics::default(),
            gpu: GpuDiagnostics::default(),
            pty: PtyDiagnostics::default(),

            statuses: Vec::new(),
        }
    }

    pub fn overall_health(&self) -> DiagnosticHealth {
        if self.statuses.iter().any(|status| {
            status.health == DiagnosticHealth::Error
        }) {
            return DiagnosticHealth::Error;
        }

        if self.statuses.iter().any(|status| {
            status.health == DiagnosticHealth::Degraded
        }) {
            return DiagnosticHealth::Degraded;
        }

        if self.statuses.iter().any(|status| {
            status.health == DiagnosticHealth::Warning
        }) {
            return DiagnosticHealth::Warning;
        }

        if self.statuses.is_empty() {
            DiagnosticHealth::Unknown
        } else {
            DiagnosticHealth::Healthy
        }
    }

    pub fn add_status(&mut self, status: DiagnosticStatus) {
        self.statuses.push(status);
    }

    pub fn status(&self, name: &str) -> Option<&DiagnosticStatus> {
        self.statuses
            .iter()
            .find(|status| status.name == name)
    }
}
