pub mod actions;
pub mod capabilities;
pub mod context;
pub mod errors;
pub mod example;
pub mod loader;
pub mod manifest;
pub mod parser;
pub mod protocol;
pub mod registry;
pub mod validation;

pub use actions::{
    CustomProtocolAction,
    CustomProtocolEvent,
    ProtocolResponse,
};

pub use capabilities::{
    CustomProtocolCapabilities,
    ProtocolColorSupport,
    ProtocolImageSupport,
    ProtocolMouseSupport,
};

pub use context::{
    ProtocolContext,
    ProtocolSecurityPolicy,
};

pub use errors::{
    CustomProtocolError,
    CustomProtocolResult,
};

pub use loader::{
    CustomProtocolLoader,
    LoadedProtocol,
};

pub use manifest::{
    CustomProtocolManifest,
    CustomProtocolMetadata,
};

pub use parser::{
    CustomProtocolParser,
    ParserState,
};

pub use protocol::{
    CustomTerminalProtocol,
    ProtocolId,
    ProtocolVersion,
};

pub use registry::{
    CustomProtocolRegistry,
    ProtocolRegistration,
};

pub use validation::{
    CustomProtocolValidator,
    ValidationIssue,
    ValidationSeverity,
};
