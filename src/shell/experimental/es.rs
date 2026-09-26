//! Es shell integration.
//!
//! Es is an extensible shell descended from the Plan 9 shell tradition.
//! Conduit treats it as an experimental integration because its syntax,
//! execution model, and ecosystem differ substantially from the more
//! common POSIX and modern shells.

use crate::shell::capabilities::{CapabilityLevel, ShellCapabilities};
use crate::shell::manifest::ShellManifest;

use super::ExperimentalShellAdapter;

/// Adapter for the Es shell.
#[derive(Debug, Clone, Copy, Default)]
pub struct Es;

impl Es {
    /// Stable Conduit identifier.
    pub const ID: &'static str = "es";

    /// Human-readable name.
    pub const NAME: &'static str = "Es";

    /// Default executable name.
    pub const EXECUTABLE: &'static str = "es";

    /// Create a new Es adapter.
    pub const fn new() -> Self {
        Self
    }

    /// Return the capabilities exposed by the Es adapter.
    pub fn capabilities() -> ShellCapabilities {
        let mut capabilities = ShellCapabilities::default();

        capabilities.interactive = CapabilityLevel::Full;
        capabilities.scripting = CapabilityLevel::Full;

        // Es follows the Plan 9 shell tradition rather than POSIX shell
        // syntax, so Conduit should not classify it as POSIX-compatible.
        capabilities.posix_compatible = CapabilityLevel::Unsupported;

        capabilities.structured_output = CapabilityLevel::Basic;

        capabilities.programmable_prompt = CapabilityLevel::Basic;
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
        capabilities.command_status_reporting = CapabilityLevel::Basic;
        capabilities.command_duration_reporting = CapabilityLevel::Basic;

        capabilities.signal_handling = CapabilityLevel::Basic;

        capabilities.startup_files = CapabilityLevel::Basic;
        capabilities.configurable_rc_file = CapabilityLevel::Basic;

        capabilities
    }

    /// Build the default manifest for Es.
    pub fn manifest() -> ShellManifest {
        ShellManifest::new(
            Self::ID,
            Self::NAME,
            Self::EXECUTABLE,
            Self::capabilities(),
        )
        .with_description(
            "Experimental integration for the Es extensible shell \
             descended from the Plan 9 shell tradition.",
        )
    }
}

impl ExperimentalShellAdapter for Es {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn executable(&self) -> &'static str {
        Self::EXECUTABLE
    }

    fn capabilities(&self) -> ShellCapabilities {
        Self::capabilities()
    }

    fn manifest(&self) -> ShellManifest {
        Self::manifest()
    }

    fn supports_prompt_hooks(&self) -> bool {
        true
    }

    fn supports_history(&self) -> bool {
        true
    }

    fn supports_completion(&self) -> bool {
        true
    }
}
