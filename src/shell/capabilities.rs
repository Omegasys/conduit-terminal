//! Shell capability descriptions.
//!
//! Different shells expose different features. Conduit uses this information
//! instead of assuming that every shell supports Bash-style integration.

/// Degree of support for a shell capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CapabilityLevel {
    Unsupported,
    Basic,
    Full,
}

impl Default for CapabilityLevel {
    fn default() -> Self {
        Self::Unsupported
    }
}

impl CapabilityLevel {
    pub fn supported(self) -> bool {
        self != Self::Unsupported
    }

    pub fn full(self) -> bool {
        self == Self::Full
    }
}

/// Individual shell capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellCapability {
    Interactive,
    Scripting,
    PosixCompatible,
    StructuredOutput,
    ProgrammablePrompt,
    PromptHooks,
    CommandHooks,
    DirectoryHooks,
    NativeHistory,
    Completion,
    JobControl,
    Aliases,
    Functions,
    EnvironmentModification,
    TerminalTitle,
    WorkingDirectoryReporting,
    CommandStatusReporting,
    CommandDurationReporting,
    SignalHandling,
    StartupFiles,
    ConfigurableRcFile,
}

/// Capabilities advertised by a shell.
#[derive(Debug, Clone)]
pub struct ShellCapabilities {
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

impl Default for ShellCapabilities {
    fn default() -> Self {
        Self {
            interactive: CapabilityLevel::Full,
            scripting: CapabilityLevel::Full,
            posix_compatible: CapabilityLevel::Unsupported,
            structured_output: CapabilityLevel::Unsupported,
            programmable_prompt: CapabilityLevel::Basic,
            prompt_hooks: CapabilityLevel::Unsupported,
            command_hooks: CapabilityLevel::Unsupported,
            directory_hooks: CapabilityLevel::Unsupported,
            native_history: CapabilityLevel::Basic,
            completion: CapabilityLevel::Basic,
            job_control: CapabilityLevel::Full,
            aliases: CapabilityLevel::Full,
            functions: CapabilityLevel::Full,
            environment_modification: CapabilityLevel::Full,
            terminal_title: CapabilityLevel::Basic,
            working_directory_reporting: CapabilityLevel::Basic,
            command_status_reporting: CapabilityLevel::Basic,
            command_duration_reporting: CapabilityLevel::Unsupported,
            signal_handling: CapabilityLevel::Full,
            startup_files: CapabilityLevel::Full,
            configurable_rc_file: CapabilityLevel::Basic,
        }
    }
}

impl ShellCapabilities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn supports(&self, capability: ShellCapability) -> bool {
        self.level(capability).supported()
    }

    pub fn fully_supports(&self, capability: ShellCapability) -> bool {
        self.level(capability).full()
    }

    pub fn level(&self, capability: ShellCapability) -> CapabilityLevel {
        match capability {
            ShellCapability::Interactive => self.interactive,
            ShellCapability::Scripting => self.scripting,
            ShellCapability::PosixCompatible => self.posix_compatible,
            ShellCapability::StructuredOutput => self.structured_output,
            ShellCapability::ProgrammablePrompt => self.programmable_prompt,
            ShellCapability::PromptHooks => self.prompt_hooks,
            ShellCapability::CommandHooks => self.command_hooks,
            ShellCapability::DirectoryHooks => self.directory_hooks,
            ShellCapability::NativeHistory => self.native_history,
            ShellCapability::Completion => self.completion,
            ShellCapability::JobControl => self.job_control,
            ShellCapability::Aliases => self.aliases,
            ShellCapability::Functions => self.functions,
            ShellCapability::EnvironmentModification => {
                self.environment_modification
            }
            ShellCapability::TerminalTitle => self.terminal_title,
            ShellCapability::WorkingDirectoryReporting => {
                self.working_directory_reporting
            }
            ShellCapability::CommandStatusReporting => {
                self.command_status_reporting
            }
            ShellCapability::CommandDurationReporting => {
                self.command_duration_reporting
            }
            ShellCapability::SignalHandling => self.signal_handling,
            ShellCapability::StartupFiles => self.startup_files,
            ShellCapability::ConfigurableRcFile => self.configurable_rc_file,
        }
    }

    pub fn set(
        &mut self,
        capability: ShellCapability,
        level: CapabilityLevel,
    ) {
        match capability {
            ShellCapability::Interactive => self.interactive = level,
            ShellCapability::Scripting => self.scripting = level,
            ShellCapability::PosixCompatible => {
                self.posix_compatible = level
            }
            ShellCapability::StructuredOutput => {
                self.structured_output = level
            }
            ShellCapability::ProgrammablePrompt => {
                self.programmable_prompt = level
            }
            ShellCapability::PromptHooks => self.prompt_hooks = level,
            ShellCapability::CommandHooks => self.command_hooks = level,
            ShellCapability::DirectoryHooks => self.directory_hooks = level,
            ShellCapability::NativeHistory => self.native_history = level,
            ShellCapability::Completion => self.completion = level,
            ShellCapability::JobControl => self.job_control = level,
            ShellCapability::Aliases => self.aliases = level,
            ShellCapability::Functions => self.functions = level,
            ShellCapability::EnvironmentModification => {
                self.environment_modification = level
            }
            ShellCapability::TerminalTitle => self.terminal_title = level,
            ShellCapability::WorkingDirectoryReporting => {
                self.working_directory_reporting = level
            }
            ShellCapability::CommandStatusReporting => {
                self.command_status_reporting = level
            }
            ShellCapability::CommandDurationReporting => {
                self.command_duration_reporting = level
            }
            ShellCapability::SignalHandling => self.signal_handling = level,
            ShellCapability::StartupFiles => self.startup_files = level,
            ShellCapability::ConfigurableRcFile => {
                self.configurable_rc_file = level
            }
        }
    }

    /// Returns capabilities for a conventional POSIX shell.
    pub fn posix() -> Self {
        let mut capabilities = Self::default();

        capabilities.posix_compatible = CapabilityLevel::Full;
        capabilities.structured_output = CapabilityLevel::Unsupported;
        capabilities.command_hooks = CapabilityLevel::Basic;
        capabilities.directory_hooks = CapabilityLevel::Basic;
        capabilities.command_status_reporting = CapabilityLevel::Full;
        capabilities.working_directory_reporting = CapabilityLevel::Full;

        capabilities
    }

    /// Returns capabilities appropriate for a structured-data shell.
    pub fn structured() -> Self {
        let mut capabilities = Self::default();

        capabilities.structured_output = CapabilityLevel::Full;
        capabilities.posix_compatible = CapabilityLevel::Unsupported;
        capabilities.command_hooks = CapabilityLevel::Full;
        capabilities.directory_hooks = CapabilityLevel::Full;
        capabilities.working_directory_reporting = CapabilityLevel::Full;

        capabilities
    }
}
