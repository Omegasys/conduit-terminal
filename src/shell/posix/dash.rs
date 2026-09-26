use super::PosixShellAdapter;
use crate::shell::{CapabilityLevel, ShellCapabilities};

/// Debian Almquist Shell (`dash`) adapter.
#[derive(Debug, Clone, Copy, Default)]
pub struct Dash;

impl Dash {
    pub const fn new() -> Self {
        Self
    }
}

impl PosixShellAdapter for Dash {
    fn id(&self) -> &'static str {
        "dash"
    }

    fn name(&self) -> &'static str {
        "Dash"
    }

    fn executable(&self) -> &'static str {
        "dash"
    }

    fn capabilities(&self) -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;
        capabilities.posix_compatible = CapabilityLevel::Full;
        capabilities.aliases = CapabilityLevel::Full;
        capabilities.functions = CapabilityLevel::Full;
        capabilities.environment_modification = CapabilityLevel::Full;
        capabilities.job_control = CapabilityLevel::Full;
        capabilities.native_history = CapabilityLevel::Basic;
        capabilities.startup_files = CapabilityLevel::Basic;
        capabilities.working_directory_reporting = CapabilityLevel::Basic;
        capabilities.command_status_reporting = CapabilityLevel::Basic;
        capabilities.signal_handling = CapabilityLevel::Full;

        capabilities
    }
}
