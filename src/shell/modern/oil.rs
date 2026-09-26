use super::ModernShellAdapter;
use crate::shell::{CapabilityLevel, ShellCapabilities};

/// Oil shell adapter.
///
/// Oil/YSH support is kept behind a dedicated adapter because its language
/// model differs from traditional POSIX shells.
#[derive(Debug, Clone, Copy, Default)]
pub struct Oil;

impl Oil {
    pub const fn new() -> Self {
        Self
    }
}

impl ModernShellAdapter for Oil {
    fn id(&self) -> &'static str {
        "oil"
    }

    fn name(&self) -> &'static str {
        "Oil Shell"
    }

    fn executable(&self) -> &'static str {
        "oil"
    }

    fn capabilities(&self) -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;
        capabilities.posix_compatible = CapabilityLevel::Basic;
        capabilities.structured_output = CapabilityLevel::Basic;

        capabilities.programmable_prompt = CapabilityLevel::Full;
        capabilities.prompt_hooks = CapabilityLevel::Basic;
        capabilities.command_hooks = CapabilityLevel::Basic;
        capabilities.directory_hooks = CapabilityLevel::Basic;

        capabilities.native_history = CapabilityLevel::Basic;
        capabilities.completion = CapabilityLevel::Basic;
        capabilities.job_control = CapabilityLevel::Basic;

        capabilities.aliases = CapabilityLevel::Full;
        capabilities.functions = CapabilityLevel::Full;
        capabilities.environment_modification = CapabilityLevel::Full;

        capabilities.terminal_title = CapabilityLevel::Basic;
        capabilities.working_directory_reporting = CapabilityLevel::Basic;
        capabilities.command_status_reporting = CapabilityLevel::Full;
        capabilities.command_duration_reporting = CapabilityLevel::Basic;

        capabilities.signal_handling = CapabilityLevel::Full;
        capabilities.startup_files = CapabilityLevel::Basic;
        capabilities.configurable_rc_file = CapabilityLevel::Basic;

        capabilities
    }
}
