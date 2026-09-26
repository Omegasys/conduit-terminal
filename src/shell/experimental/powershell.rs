//! PowerShell shell integration.
//!
//! This adapter covers both Windows PowerShell and PowerShell 7+ where
//! the executable is available as `pwsh`.

use crate::shell::capabilities::{CapabilityLevel, ShellCapabilities};
use crate::shell::manifest::ShellManifest;

use super::MicrosoftShellAdapter;

/// Adapter for Microsoft PowerShell.
#[derive(Debug, Clone, Copy, Default)]
pub struct PowerShell;

impl PowerShell {
    /// Stable Conduit identifier.
    pub const ID: &'static str = "powershell";

    /// Human-readable name.
    pub const NAME: &'static str = "PowerShell";

    /// Default Windows PowerShell executable.
    pub const EXECUTABLE: &'static str = "powershell";

    /// PowerShell Core / PowerShell 7 executable.
    pub const CORE_EXECUTABLE: &'static str = "pwsh";

    /// Create a new PowerShell adapter.
    pub const fn new() -> Self {
        Self
    }

    /// Return the capabilities exposed by PowerShell.
    pub fn capabilities() -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;

        // PowerShell has its own language and pipeline model rather than
        // being a POSIX shell.
        capabilities.posix_compatible = CapabilityLevel::Unsupported;

        // Objects can flow through the PowerShell pipeline, making
        // structured output a core feature.
        capabilities.structured_output = CapabilityLevel::Full;

        capabilities.programmable_prompt = CapabilityLevel::Full;
        capabilities.prompt_hooks = CapabilityLevel::Full;
        capabilities.command_hooks = CapabilityLevel::Full;
        capabilities.directory_hooks = CapabilityLevel::Full;

        capabilities.native_history = CapabilityLevel::Full;
        capabilities.completion = CapabilityLevel::Full;

        capabilities.job_control = CapabilityLevel::Basic;

        capabilities.aliases = CapabilityLevel::Full;
        capabilities.functions = CapabilityLevel::Full;
        capabilities.environment_modification = CapabilityLevel::Full;

        capabilities.terminal_title = CapabilityLevel::Full;
        capabilities.working_directory_reporting = CapabilityLevel::Full;
        capabilities.command_status_reporting = CapabilityLevel::Full;
        capabilities.command_duration_reporting = CapabilityLevel::Full;

        capabilities.signal_handling = CapabilityLevel::Basic;

        capabilities.startup_files = CapabilityLevel::Full;
        capabilities.configurable_rc_file = CapabilityLevel::Full;

        capabilities
    }

    /// Build the default manifest for Windows PowerShell.
    pub fn manifest() -> ShellManifest {
        ShellManifest::new(
            Self::ID,
            Self::NAME,
            Self::EXECUTABLE,
            Self::capabilities(),
        )
        .with_description(
            "Microsoft PowerShell integration, including support for \
             PowerShell's object-oriented pipeline and programmable shell.",
        )
    }

    /// Build a manifest targeting PowerShell Core / PowerShell 7+.
    pub fn core_manifest() -> ShellManifest {
        ShellManifest::new(
            Self::ID,
            "PowerShell Core",
            Self::CORE_EXECUTABLE,
            Self::capabilities(),
        )
        .with_description(
            "PowerShell Core / PowerShell 7+ integration using pwsh.",
        )
    }

    /// Return the executable appropriate for the current platform.
    pub fn platform_executable() -> &'static str {
        if cfg!(windows) {
            Self::EXECUTABLE
        } else {
            Self::CORE_EXECUTABLE
        }
    }
}

impl MicrosoftShellAdapter for PowerShell {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn executable(&self) -> &'static str {
        Self::platform_executable()
    }

    fn capabilities(&self) -> ShellCapabilities {
        Self::capabilities()
    }

    fn manifest(&self) -> ShellManifest {
        if cfg!(windows) {
            Self::manifest()
        } else {
            Self::core_manifest()
        }
    }

    fn supports_prompt_hooks(&self) -> bool {
        true
    }

    fn supports_command_hooks(&self) -> bool {
        true
    }

    fn supports_history(&self) -> bool {
        true
    }

    fn supports_completion(&self) -> bool {
        true
    }
}
