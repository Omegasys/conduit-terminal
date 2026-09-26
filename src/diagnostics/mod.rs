pub mod configuration;
pub mod crash;
pub mod export;
pub mod gpu;
pub mod memory;
pub mod performance;
pub mod plugins;
pub mod protocols;
pub mod pty;
pub mod renderer;
pub mod resources;
pub mod status;

pub use configuration::ConfigurationDiagnostics;
pub use crash::{
    CrashDiagnostics,
    CrashRecord,
};
pub use export::{
    DiagnosticExportFormat,
    DiagnosticExporter,
    DiagnosticReport,
};
pub use gpu::GpuDiagnostics;
pub use memory::MemoryDiagnostics;
pub use performance::PerformanceDiagnostics;
pub use plugins::PluginDiagnostics;
pub use protocols::ProtocolDiagnostics;
pub use pty::PtyDiagnostics;
pub use renderer::RendererDiagnostics;
pub use resources::ResourceDiagnostics;
pub use status::{
    DiagnosticHealth,
    DiagnosticStatus,
    DiagnosticsSnapshot,
};
