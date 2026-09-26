use std::fmt;

/// Current IPC protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IpcVersion {
    pub major: u16,
    pub minor: u16,
}

impl IpcVersion {
    pub const CURRENT: Self = Self {
        major: 1,
        minor: 0,
    };

    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub const fn compatible_with(self, other: Self) -> bool {
        self.major == other.major
    }
}

impl Default for IpcVersion {
    fn default() -> Self {
        Self::CURRENT
    }
}

impl fmt::Display for IpcVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)
    }
}

/// Top-level IPC message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcMessage {
    Request(IpcRequest),
    Response(IpcResponse),
    Event(crate::ipc::events::IpcEvent),
}

/// A request sent to the Conduit IPC server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcRequest {
    pub id: u64,
    pub version: IpcVersion,
    pub command: crate::ipc::commands::IpcCommand,
}

impl IpcRequest {
    pub fn new(id: u64, command: crate::ipc::commands::IpcCommand) -> Self {
        Self {
            id,
            version: IpcVersion::CURRENT,
            command,
        }
    }
}

/// Response to an IPC request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcResponse {
    pub id: u64,
    pub version: IpcVersion,
    pub result: crate::ipc::commands::IpcCommandResult,
}

impl IpcResponse {
    pub fn success(id: u64, result: crate::ipc::commands::IpcCommandResult) -> Self {
        Self {
            id,
            version: IpcVersion::CURRENT,
            result,
        }
    }
}

/// Protocol envelope used when a message needs explicit framing metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcEnvelope {
    pub version: IpcVersion,
    pub message: IpcMessage,
}

impl IpcEnvelope {
    pub fn new(message: IpcMessage) -> Self {
        Self {
            version: IpcVersion::CURRENT,
            message,
        }
    }

    pub fn compatible_with(&self, version: IpcVersion) -> bool {
        self.version.compatible_with(version)
    }
}
