use crate::terminal_protocols::custom::{
    CustomProtocolAction,
    CustomProtocolCapabilities,
    CustomProtocolEvent,
    CustomProtocolManifest,
    CustomProtocolMetadata,
    CustomProtocolResult,
    CustomTerminalProtocol,
    ProtocolContext,
    ProtocolId,
    ProtocolVersion,
};

pub struct ExampleProtocol {
    manifest: CustomProtocolManifest,
    capabilities: CustomProtocolCapabilities,
}

impl ExampleProtocol {
    pub fn new() -> Self {
        let metadata = CustomProtocolMetadata::new(
            "Conduit Example Author",
            "Example custom terminal protocol.",
        )
        .license("GPL-3.0-or-later");

        let mut manifest = CustomProtocolManifest::new(
            ProtocolId::new("example-terminal"),
            "Example Terminal Protocol",
            metadata,
        );

        manifest.set_version(ProtocolVersion::new(1, 0, 0));

        let capabilities = CustomProtocolCapabilities::modern_terminal();

        Self {
            manifest,
            capabilities,
        }
    }
}

impl Default for ExampleProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomTerminalProtocol for ExampleProtocol {
    fn id(&self) -> ProtocolId {
        self.manifest.id().clone()
    }

    fn version(&self) -> ProtocolVersion {
        self.manifest.version().clone()
    }

    fn manifest(&self) -> CustomProtocolManifest {
        self.manifest.clone()
    }

    fn capabilities(&self) -> CustomProtocolCapabilities {
        self.capabilities.clone()
    }

    fn reset(&mut self) {
        // Reset protocol-specific state here.
    }

    fn feed(
        &mut self,
        bytes: &[u8],
        _context: &mut ProtocolContext,
    ) -> CustomProtocolResult<Vec<CustomProtocolEvent>> {
        let text = String::from_utf8_lossy(bytes);

        Ok(vec![CustomProtocolEvent::Action(
            CustomProtocolAction::Print(text.into_owned()),
        )])
    }

    fn handle_action(
        &mut self,
        action: CustomProtocolAction,
        _context: &mut ProtocolContext,
    ) -> CustomProtocolResult<Vec<CustomProtocolEvent>> {
        Ok(vec![CustomProtocolEvent::Action(action)])
    }
}
