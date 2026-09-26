//! Experimental shell adapters.
//!
//! This module contains integrations for shells that are less common,
//! experimental, research-oriented, or otherwise outside Conduit's
//! primary shell families.

pub mod es;

pub use es::Es;

use crate::shell::capabilities::ShellCapabilities;
use crate::shell::manifest::ShellManifest;

/// Common interface implemented by experimental shell adapters.
pub trait ExperimentalShellAdapter: Send + Sync {
    /// Stable identifier used by Conduit.
    fn id(&self) -> &'static str;

    /// Human-readable shell name.
    fn name(&self) -> &'static str;

    /// Executable used to launch the shell.
    fn executable(&self) -> &'static str;

    /// Capabilities supported by the shell.
    fn capabilities(&self) -> ShellCapabilities;

    /// Build a shell manifest describing the adapter.
    fn manifest(&self) -> ShellManifest {
        ShellManifest::new(
            self.id(),
            self.name(),
            self.executable(),
            self.capabilities(),
        )
    }

    /// Whether the shell provides useful prompt integration hooks.
    fn supports_prompt_hooks(&self) -> bool {
        false
    }

    /// Whether the shell provides native history integration.
    fn supports_history(&self) -> bool {
        false
    }

    /// Whether the shell provides programmable completion.
    fn supports_completion(&self) -> bool {
        false
    }
}

/// Return the experimental adapters built into Conduit.
pub fn built_in() -> Vec<Box<dyn ExperimentalShellAdapter>> {
    vec![Box::new(Es::default())]
}
