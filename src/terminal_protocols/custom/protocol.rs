use std::fmt;

use super::{
    capabilities::CustomProtocolCapabilities,
    decoder::CustomProtocolDecoder,
    encoder::CustomProtocolEncoder,
    errors::CustomProtocolResult,
    manifest::CustomProtocolManifest,
    parser::ProtocolParseResult,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CustomProtocolId(String);

impl CustomProtocolId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for CustomProtocolId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl From<&str> for CustomProtocolId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CustomProtocolId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomProtocolVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl CustomProtocolVersion {
    pub fn new(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn initial() -> Self {
        Self::new(1, 0, 0)
    }
}

impl Default for CustomProtocolVersion {
    fn default() -> Self {
        Self::initial()
    }
}

impl fmt::Display for CustomProtocolVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

pub trait CustomProtocol: Send {
    fn id(&self) -> CustomProtocolId;

    fn version(&self) -> CustomProtocolVersion {
        CustomProtocolVersion::initial()
    }

    fn manifest(&self) -> CustomProtocolManifest;

    fn capabilities(&self) -> CustomProtocolCapabilities;

    fn decoder(&self) -> &CustomProtocolDecoder;

    fn decoder_mut(&mut self) -> &mut CustomProtocolDecoder;

    fn encoder(&self) -> &CustomProtocolEncoder;

    fn encoder_mut(&mut self) -> &mut CustomProtocolEncoder;

    fn initialize(&mut self) -> CustomProtocolResult<()>;

    fn reset(&mut self);

    fn parse(
        &mut self,
        input: &[u8],
    ) -> CustomProtocolResult<ProtocolParseResult>;
}
