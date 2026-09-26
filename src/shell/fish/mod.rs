//! Fish shell integration.

pub mod fish;

use crate::shell::{CapabilityLevel, ShellCapabilities, ShellManifest};

/// Common interface for Fish-family adapters.
pub trait FishShellAdapter: Send + Sync {
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

    fn supports_event_hooks(&self) -> bool {
        self.capabilities().command_hooks != CapabilityLevel::Unsupported
    }

    fn supports_completion(&self) -> bool {
        self.capabilities().completion != CapabilityLevel::Unsupported
    }
}

/// Returns the built-in Fish adapter.
pub fn built_in() -> Vec<Box<dyn FishShellAdapter>> {
    vec![Box::new(fish::Fish)]
}
