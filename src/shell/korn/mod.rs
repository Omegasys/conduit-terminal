//! Korn-family shell adapters.

pub mod ksh;
pub mod mksh;

use crate::shell::{CapabilityLevel, ShellCapabilities, ShellManifest};

/// Common interface for Korn-family shells.
pub trait KornShellAdapter: Send + Sync {
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

    fn supports_prompt_hooks(&self) -> bool {
        self.capabilities()
            .prompt_hooks
            != CapabilityLevel::Unsupported
    }
}

/// Constructs the built-in Korn-family adapters.
pub fn built_in() -> Vec<Box<dyn KornShellAdapter>> {
    vec![
        Box::new(ksh::Ksh),
        Box::new(mksh::Mksh),
    ]
}
