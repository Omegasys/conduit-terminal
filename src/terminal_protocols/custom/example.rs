use super::{
    capabilities::CustomProtocolCapabilities,
    decoder::CustomProtocolDecoder,
    encoder::CustomProtocolEncoder,
    errors::CustomProtocolResult,
    manifest::{
        CustomProtocolManifest,
        CustomProtocolMetadata,
    },
    parser::ProtocolParseResult,
    protocol::{
        CustomProtocol,
        CustomProtocolId,
        CustomProtocolVersion,
    },
};

pub struct ExampleProtocol {
    manifest: CustomProtocolManifest,
    decoder: CustomProtocolDecoder,
    encoder: CustomProtocolEncoder,
}

impl ExampleProtocol {
    pub fn new() -> Self {
        let metadata =
            CustomProtocolMetadata::new(
                "Conduit",
                "Example custom terminal protocol.",
            )
            .license("GPLv3");

        let mut manifest =
            CustomProtocolManifest::new(
                CustomProtocolId::new(
                    "example-protocol",
                ),
                "Example Protocol",
                metadata,
                ".",
            );

        manifest.set_version(
            CustomProtocolVersion::new(
                1,
                0,
                0,
            ),
        );

        manifest.set_capabilities(
            CustomProtocolCapabilities::terminal(),
        );

        manifest.set_entrypoint(
            "src/protocol.rs",
        );

        Self {
            manifest,
            decoder: CustomProtocolDecoder::new(),
            encoder: CustomProtocolEncoder::new(),
        }
    }
}

impl Default for ExampleProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomProtocol for ExampleProtocol {
    fn id(&self) -> CustomProtocolId {
        self.manifest.id().clone()
    }

    fn version(&self) -> CustomProtocolVersion {
        self.manifest.version()
    }

    fn manifest(&self) -> CustomProtocolManifest {
        self.manifest.clone()
    }

    fn capabilities(&self) -> CustomProtocolCapabilities {
        self.manifest.capabilities().clone()
    }

    fn decoder(&self) -> &CustomProtocolDecoder {
        &self.decoder
    }

    fn decoder_mut(
        &mut self,
    ) -> &mut CustomProtocolDecoder {
        &mut self.decoder
    }

    fn encoder(&self) -> &CustomProtocolEncoder {
        &self.encoder
    }

    fn encoder_mut(
        &mut self,
    ) -> &mut CustomProtocolEncoder {
        &mut self.encoder
    }

    fn initialize(
        &mut self,
    ) -> CustomProtocolResult<()> {
        Ok(())
    }

    fn reset(&mut self) {
        self.decoder.reset();
    }

    fn parse(
        &mut self,
        input: &[u8],
    ) -> CustomProtocolResult<ProtocolParseResult> {
        self.decoder.decode(input)
    }
}
