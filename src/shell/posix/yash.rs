use super::PosixShellAdapter;
use crate::shell::{CapabilityLevel, ShellCapabilities};

/// Yet another shell (`yash`) adapter.
#[derive(Debug, Clone, Copy, Default)]
pub struct Yash;

impl Yash {
    pub const fn new() -> Self {
        Self
    }
}

impl PosixShellAdapter for Yash {
    fn id(&self) -> &'static str {
        "yash"
    }

    fn name(&self) -> &'static str {
        "Yash"
    }

    fn executable(&self) -> &'static str {
        "yash"
    }

    fn capabilities(&self) -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;
        capabilities.posix_compatible = CapabilityLevel::Full;
        capabilities.programmable_prompt = CapabilityLevel::Basic;
        capabilities.prompt_hooks = CapabilityLevel::Basic;
        capabilities.command_hooks = CapabilityLevel::Basic;
        capabilities.directory_hooks = CapabilityLevel::Basic;
        capabilities.native_history = CapabilityLevel::Full;
        capabilities.completion = CapabilityLevel::Full;
        capabilities.job_control = CapabilityLevel::Full;
        capabilities.aliases = CapabilityLevel::Full;
        capabilities.functions = CapabilityLevel::Full;
        capabilities.environment_modification = CapabilityLevel::Full;
        capabilities.working_directory_reporting = CapabilityLevel::Basic;
        capabilities.command_status_reporting = CapabilityLevel::Full;
        capabilities.command_duration_reporting = CapabilityLevel::Basic;
        capabilities.signal_handling = CapabilityLevel::Full;
        capabilities.startup_files = CapabilityLevel::Full;
        capabilities.configurable_rc_file = CapabilityLevel::Full;

        capabilities
    }
}
