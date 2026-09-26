//! Modern and non-POSIX shell adapters.
//!
//! These shells have command languages and runtime models that differ
//! substantially from traditional POSIX shells.

pub mod elvish;
pub mod nushell;
pub mod oil;
pub mod rc;
pub mod xonsh;
pub mod ysh;

use crate::shell::{CapabilityLevel, ShellCapabilities, ShellManifest};

/// Common interface for modern shell adapters.
pub trait ModernShellAdapter: Send + Sync {
    fn id(&self) -> &'static str;

    fn name(&self) -> &'static str;

    fn executable(&self) -> &'static str;

    fn capabilities(&self) -> ShellCapabilities;

    fn manifest(&self) -> ShellManifest {
        let mut manifest = ShellManifest::new(
            self.id(),
            self.name(),
            self.executable(),
        );

        manifest.capabilities = self.capabilities();
        manifest
    }

    /// Whether the shell natively works with structured data.
    fn supports_structured_output(&self) -> bool {
        self.capabilities().structured_output != CapabilityLevel::Unsupported
    }

    /// Whether the shell provides programmable completion.
    fn supports_completion(&self) -> bool {
        self.capabilities().completion != CapabilityLevel::Unsupported
    }

    /// Whether Conduit can integrate with the shell's prompt.
    fn supports_prompt_hooks(&self) -> bool {
        self.capabilities().prompt_hooks != CapabilityLevel::Unsupported
    }
}

/// Returns all built-in modern shell adapters.
pub fn built_in() -> Vec<Box<dyn ModernShellAdapter>> {
    vec![
        Box::new(nushell::Nushell),
        Box::new(elvish::Elvish),
        Box::new(xonsh::Xonsh),
        Box::new(oil::Oil),
        Box::new(ysh::Ysh),
        Box::new(rc::Rc),
    ]
}
