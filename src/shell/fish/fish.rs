use super::FishShellAdapter;
use crate::shell::{CapabilityLevel, ShellCapabilities};

/// Fish shell adapter.
#[derive(Debug, Clone, Copy, Default)]
pub struct Fish;

impl Fish {
    pub const fn new() -> Self {
        Self
    }
}

impl FishShellAdapter for Fish {
    fn id(&self) -> &'static str {
        "fish"
    }

    fn name(&self) -> &'static str {
        "Fish"
    }

    fn executable(&self) -> &'static str {
        "fish"
    }

    fn capabilities(&self) -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;

        capabilities.programmable_prompt = CapabilityLevel::Full;
        capabilities.prompt_hooks = CapabilityLevel::Full;
        capabilities.command_hooks = CapabilityLevel::Full;
        capabilities.directory_hooks = CapabilityLevel::Full;

        capabilities.native_history = CapabilityLevel::Full;
        capabilities.completion = CapabilityLevel::Full;
        capabilities.job_control = CapabilityLevel::Full;

        capabilities.aliases = CapabilityLevel::Full;
        capabilities.functions = CapabilityLevel::Full;
        capabilities.environment_modification = CapabilityLevel::Full;

        capabilities.terminal_title = CapabilityLevel::Full;
        capabilities.working_directory_reporting = CapabilityLevel::Full;
        capabilities.command_status_reporting = CapabilityLevel::Full;
        capabilities.command_duration_reporting = CapabilityLevel::Full;

        capabilities.signal_handling = CapabilityLevel::Full;
        capabilities.startup_files = CapabilityLevel::Full;
        capabilities.configurable_rc_file = CapabilityLevel::Full;

        capabilities
    }
}
