//! C-shell family integration.
//!
//! Provides adapters for traditional C-shell implementations such as
//! `csh` and `tcsh`.

pub mod csh;
pub mod tcsh;

use crate::shell::{CapabilityLevel, ShellCapabilities, ShellManifest};

/// Common interface for C-shell-family adapters.
pub trait CShellAdapter: Send + Sync {
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

    fn supports_prompt(&self) -> bool {
        self.capabilities().programmable_prompt != CapabilityLevel::Unsupported
    }

    fn supports_history(&self) -> bool {
        self.capabilities().native_history != CapabilityLevel::Unsupported
    }
}

/// Returns all built-in C-shell adapters.
pub fn built_in() -> Vec<Box<dyn CShellAdapter>> {
    vec![
        Box::new(csh::Csh),
        Box::new(tcsh::Tcsh),
    ]
}
