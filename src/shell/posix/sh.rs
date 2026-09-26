use super::PosixShellAdapter;
use crate::shell::{CapabilityLevel, ShellCapabilities};

/// Generic POSIX `sh` adapter.
///
/// This represents the POSIX shell interface rather than a particular
/// implementation. `/bin/sh` may resolve to Bash, Dash, Ash, or another
/// implementation.
#[derive(Debug, Clone, Copy, Default)]
pub struct Sh;

impl Sh {
    pub const fn new() -> Self {
        Self
    }
}

impl PosixShellAdapter for Sh {
    fn id(&self) -> &'static str {
        "sh"
    }

    fn name(&self) -> &'static str {
        "POSIX sh"
    }

    fn executable(&self) -> &'static str {
        "sh"
    }

    fn capabilities(&self) -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;
        capabilities.posix_compatible = CapabilityLevel::Full;
        capabilities.environment_modification = CapabilityLevel::Full;
        capabilities.aliases = CapabilityLevel::Full;
        capabilities.functions = CapabilityLevel::Full;
        capabilities.job_control = CapabilityLevel::Basic;
        capabilities.startup_files = CapabilityLevel::Basic;
        capabilities.working_directory_reporting = CapabilityLevel::Basic;
        capabilities.command_status_reporting = CapabilityLevel::Basic;
        capabilities.signal_handling = CapabilityLevel::Basic;

        capabilities
    }
}
