use super::KornShellAdapter;
use crate::shell::{CapabilityLevel, ShellCapabilities};

/// MirBSD Korn Shell (`mksh`) adapter.
#[derive(Debug, Clone, Copy, Default)]
pub struct Mksh;

impl Mksh {
    pub const fn new() -> Self {
        Self
    }
}

impl KornShellAdapter for Mksh {
    fn id(&self) -> &'static str {
        "mksh"
    }

    fn name(&self) -> &'static str {
        "MirBSD Korn Shell"
    }

    fn executable(&self) -> &'static str {
        "mksh"
    }

    fn capabilities(&self) -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;
        capabilities.posix_compatible = CapabilityLevel::Full;
        capabilities.programmable_prompt = CapabilityLevel::Basic;
        capabilities.prompt_hooks = CapabilityLevel::Basic;
        capabilities.command_hooks = CapabilityLevel::Unsupported;
        capabilities.directory_hooks = CapabilityLevel::Basic;
        capabilities.native_history = CapabilityLevel::Basic;
        capabilities.completion = CapabilityLevel::Basic;
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
