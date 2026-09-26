use super::PosixShellAdapter;
use crate::shell::{CapabilityLevel, ShellCapabilities};

/// Heirloom Toolchest Bourne shell adapter.
#[derive(Debug, Clone, Copy, Default)]
pub struct HeirloomSh;

impl HeirloomSh {
    pub const fn new() -> Self {
        Self
    }
}

impl PosixShellAdapter for HeirloomSh {
    fn id(&self) -> &'static str {
        "heirloom-sh"
    }

    fn name(&self) -> &'static str {
        "Heirloom Bourne Shell"
    }

    fn executable(&self) -> &'static str {
        "heirloom-sh"
    }

    fn capabilities(&self) -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;
        capabilities.posix_compatible = CapabilityLevel::Basic;
        capabilities.aliases = CapabilityLevel::Basic;
        capabilities.functions = CapabilityLevel::Basic;
        capabilities.environment_modification = CapabilityLevel::Full;
        capabilities.job_control = CapabilityLevel::Basic;
        capabilities.startup_files = CapabilityLevel::Basic;
        capabilities.working_directory_reporting = CapabilityLevel::Basic;
        capabilities.command_status_reporting = CapabilityLevel::Basic;
        capabilities.signal_handling = CapabilityLevel::Basic;

        capabilities
    }
}
