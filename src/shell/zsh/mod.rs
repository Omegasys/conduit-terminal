pub mod zsh;

use crate::shell::{CapabilityLevel, ShellCapabilities, ShellManifest};

/// Common interface for Zsh adapters.
pub trait ZshShellAdapter: Send + Sync {
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

    fn supports_hooks(&self) -> bool {
        self.capabilities().prompt_hooks != CapabilityLevel::Unsupported
            || self.capabilities().command_hooks
                != CapabilityLevel::Unsupported
    }
}

/// Constructs the built-in Zsh adapter.
pub fn built_in() -> Vec<Box<dyn ZshShellAdapter>> {
    vec![Box::new(zsh::Zsh)]
}
