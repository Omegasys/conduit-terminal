use super::CShellAdapter;
use crate::shell::{CapabilityLevel, ShellCapabilities};

/// Traditional C shell (`csh`) adapter.
#[derive(Debug, Clone, Copy, Default)]
pub struct Csh;

impl Csh {
    pub const fn new() -> Self {
        Self
    }
}

impl CShellAdapter for Csh {
    fn id(&self) -> &'static str {
        "csh"
    }

    fn name(&self) -> &'static str {
        "C Shell"
    }

    fn executable(&self) -> &'static str {
        "csh"
    }

    fn capabilities(&self) -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;

        capabilities.programmable_prompt = CapabilityLevel::Full;
        capabilities.prompt_hooks = CapabilityLevel::Basic;
        capabilities.command_hooks = CapabilityLevel::Basic;
        capabilities.directory_hooks = CapabilityLevel::Basic;

        capabilities.native_history = CapabilityLevel::Full;
        capabilities.completion = CapabilityLevel::Basic;
        capabilities.job_control = CapabilityLevel::Full;

        capabilities.aliases = CapabilityLevel::Full;
        capabilities.functions = CapabilityLevel::Basic;
        capabilities.environment_modification = CapabilityLevel::Full;

        capabilities.terminal_title = CapabilityLevel::Basic;
        capabilities.working_directory_reporting = CapabilityLevel::Basic;
        capabilities.command_status_reporting = CapabilityLevel::Full;
        capabilities.command_duration_reporting = CapabilityLevel::Basic;

        capabilities.signal_handling = CapabilityLevel::Full;
        capabilities.startup_files = CapabilityLevel::Full;
        capabilities.configurable_rc_file = CapabilityLevel::Full;

        capabilities
    }
}
