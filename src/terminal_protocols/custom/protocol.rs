use std::fmt;

use super::{
    actions::{CustomProtocolAction, CustomProtocolEvent},
    capabilities::CustomProtocolCapabilities,
    context::ProtocolContext,
    errors::CustomProtocolResult,
    manifest::CustomProtocolManifest,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProtocolId(String);

impl ProtocolId {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ProtocolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for ProtocolId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ProtocolId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl ProtocolVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub const fn initial() -> Self {
        Self::new(1, 0, 0)
    }
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::initial()
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Trait implemented by a custom terminal protocol.
///
/// A protocol implementation is responsible for interpreting its own
/// byte stream and producing Conduit-level actions/events.
pub trait CustomTerminalProtocol: Send {
    fn id(&self) -> ProtocolId;

    fn version(&self) -> ProtocolVersion {
        ProtocolVersion::initial()
    }

    fn manifest(&self) -> CustomProtocolManifest;

    fn capabilities(&self) -> CustomProtocolCapabilities;

    fn reset(&mut self);

    fn feed(
        &mut self,
        bytes: &[u8],
        context: &mut ProtocolContext,
    ) -> CustomProtocolResult<Vec<CustomProtocolEvent>>;

    fn handle_action(
        &mut self,
        action: CustomProtocolAction,
        context: &mut ProtocolContext,
    ) -> CustomProtocolResult<Vec<CustomProtocolEvent>>;
}
