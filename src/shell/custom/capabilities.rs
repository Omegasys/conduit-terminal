//! Capabilities exposed by custom shells.

use crate::shell::capabilities::CapabilityLevel;

/// Capabilities that can be declared by a custom shell.
#[derive(Debug, Clone)]
pub struct CustomCapabilities {
    pub interactive: CapabilityLevel,
    pub scripting: CapabilityLevel,
    pub posix_compatible: CapabilityLevel,
    pub structured_output: CapabilityLevel,
    pub programmable_prompt: CapabilityLevel,
    pub prompt_hooks: CapabilityLevel,
    pub command_hooks: CapabilityLevel,
    pub directory_hooks: CapabilityLevel,
    pub native_history: CapabilityLevel,
    pub completion: CapabilityLevel,
    pub job_control: CapabilityLevel,
    pub aliases: CapabilityLevel,
    pub functions: CapabilityLevel,
    pub environment_modification: CapabilityLevel,
    pub terminal_title: CapabilityLevel,
    pub working_directory_reporting: CapabilityLevel,
    pub command_status_reporting: CapabilityLevel,
    pub command_duration_reporting: CapabilityLevel,
    pub signal_handling: CapabilityLevel,
    pub startup_files: CapabilityLevel,
    pub configurable_rc_file: CapabilityLevel,
}

impl Default for CustomCapabilities {
    fn default() -> Self {
        Self {
            interactive: CapabilityLevel::Basic,
            scripting: CapabilityLevel::Basic,
            posix_compatible: CapabilityLevel::Unsupported,
            structured_output: CapabilityLevel::Unsupported,
            programmable_prompt: CapabilityLevel::Unsupported,
            prompt_hooks: CapabilityLevel::Unsupported,
            command_hooks: CapabilityLevel::Unsupported,
            directory_hooks: CapabilityLevel::Unsupported,
            native_history: CapabilityLevel::Unsupported,
            completion: CapabilityLevel::Unsupported,
            job_control: CapabilityLevel::Basic,
            aliases: CapabilityLevel::Unsupported,
            functions: CapabilityLevel::Unsupported,
            environment_modification: CapabilityLevel::Basic,
            terminal_title: CapabilityLevel::Unsupported,
            working_directory_reporting: CapabilityLevel::Unsupported,
            command_status_reporting: CapabilityLevel::Unsupported,
            command_duration_reporting: CapabilityLevel::Unsupported,
            signal_handling: CapabilityLevel::Basic,
            startup_files: CapabilityLevel::Unsupported,
            configurable_rc_file: CapabilityLevel::Unsupported,
        }
    }
}

impl CustomCapabilities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn supports(&self, level: CapabilityLevel) -> bool {
        self.interactive == level
            || self.scripting == level
            || self.completion == level
    }
}
