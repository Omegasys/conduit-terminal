use super::ModernShellAdapter;
use crate::shell::{CapabilityLevel, ShellCapabilities};

/// Elvish shell adapter.
#[derive(Debug, Clone, Copy, Default)]
pub struct Elvish;

impl Elvish {
    pub const fn new() -> Self {
        Self
    }
}

impl ModernShellAdapter for Elvish {
    fn id(&self) -> &'static str {
        "elvish"
    }

    fn name(&self) -> &'static str {
        "Elvish"
    }

    fn executable(&self) -> &'static str {
        "elvish"
    }

    fn capabilities(&self) -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;
        capabilities.structured_output = CapabilityLevel::Full;

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
        capabilities.command_duration_reporting = CapabilityLevel::Basic;

        capabilities.signal_handling = CapabilityLevel::Full;
        capabilities.startup_files = CapabilityLevel::Full;
        capabilities.configurable_rc_file = CapabilityLevel::Full;

        capabilities
    }
}
