//! POSIX-compatible shell adapters.
//!
//! These adapters provide shell-specific metadata and behavior while
//! delegating common process/session management to the parent shell module.

pub mod ash;
pub mod bash;
pub mod dash;
pub mod heirloom;
pub mod sh;
pub mod yash;

use super::{CapabilityLevel, ShellCapabilities, ShellManifest};

/// Common interface implemented by POSIX shell adapters.
pub trait PosixShellAdapter: Send + Sync {
    /// Canonical shell identifier.
    fn id(&self) -> &'static str;

    /// Human-readable shell name.
    fn name(&self) -> &'static str;

    /// Executable normally used to launch the shell.
    fn executable(&self) -> &'static str;

    /// Returns the shell's capabilities.
    fn capabilities(&self) -> ShellCapabilities;

    /// Creates a shell manifest.
    fn manifest(&self) -> ShellManifest {
        let mut manifest = ShellManifest::new(
            self.id(),
            self.name(),
            self.executable(),
        );

        manifest.capabilities = self.capabilities();
        manifest
    }

    /// Whether this shell provides programmable prompt hooks.
    fn supports_prompt_hooks(&self) -> bool {
        self.capabilities()
            .prompt_hooks
            != CapabilityLevel::Unsupported
    }

    /// Whether this shell provides command hooks.
    fn supports_command_hooks(&self) -> bool {
        self.capabilities()
            .command_hooks
            != CapabilityLevel::Unsupported
    }

    /// Whether this shell provides native history.
    fn supports_history(&self) -> bool {
        self.capabilities()
            .native_history
            != CapabilityLevel::Unsupported
    }

    /// Whether this shell provides completion.
    fn supports_completion(&self) -> bool {
        self.capabilities()
            .completion
            != CapabilityLevel::Unsupported
    }
}

/// Constructs all built-in POSIX adapters.
pub fn built_in() -> Vec<Box<dyn PosixShellAdapter>> {
    vec![
        Box::new(sh::Sh),
        Box::new(bash::Bash),
        Box::new(dash::Dash),
        Box::new(ash::Ash),
        Box::new(yash::Yash),
        Box::new(heirloom::HeirloomSh),
    ]
}
