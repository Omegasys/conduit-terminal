use super::KornShellAdapter;
use crate::shell::{CapabilityLevel, ShellCapabilities};

/// KornShell adapter.
#[derive(Debug, Clone, Copy, Default)]
pub struct Ksh;

impl Ksh {
    pub const fn new() -> Self {
        Self
    }
}

impl KornShellAdapter for Ksh {
    fn id(&self) -> &'static str {
        "ksh"
    }

    fn name(&self) -> &'static str {
        "KornShell"
    }

    fn executable(&self) -> &'static str {
        "ksh"
    }

    fn capabilities(&self) -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;
        capabilities.posix_compatible = CapabilityLevel::Full;
        capabilities.programmable_prompt = CapabilityLevel::Full;
        capabilities.prompt_hooks = CapabilityLevel::Full;
        capabilities.command_hooks = CapabilityLevel::Basic;
        capabilities.directory_hooks = CapabilityLevel::Basic;
        capabilities.native_history = CapabilityLevel::Full;
        capabilities.completion = CapabilityLevel::Full;
        capabilities.job_control = CapabilityLevel::Full;
        capabilities.aliases = CapabilityLevel::Full;
        capabilities.functions = CapabilityLevel::Full;
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
