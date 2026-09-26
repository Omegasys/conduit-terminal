use std::fmt;
use std::io;
use std::path::PathBuf;

use super::{
    commands::{IpcCommand, IpcCommandResult},
    json::JsonCodec,
    protocol::{IpcEnvelope, IpcMessage, IpcRequest},
    socket::SocketAddress,
};

/// Configuration for the IPC server.
#[derive(Debug, Clone)]
pub struct IpcServerConfig {
    pub address: SocketAddress,
    pub enabled: bool,
    pub max_clients: usize,
    pub max_frame_size: usize,
}

impl Default for IpcServerConfig {
    fn default() -> Self {
        Self {
            address: SocketAddress::default(),
            enabled: true,
            max_clients: 16,
            max_frame_size: 64 * 1024 * 1024,
        }
    }
}

/// Errors produced by the IPC server.
#[derive(Debug)]
pub enum IpcServerError {
    Io(io::Error),
    Protocol(String),
    Disabled,
    UnsupportedTransport,
}

impl fmt::Display for IpcServerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "IPC server I/O error: {error}"),
            Self::Protocol(message) => write!(formatter, "IPC protocol error: {message}"),
            Self::Disabled => formatter.write_str("IPC server is disabled"),
            Self::UnsupportedTransport => {
                formatter.write_str("IPC transport is unsupported")
            }
        }
    }
}

impl std::error::Error for IpcServerError {}

/// Local IPC server.
///
/// The server owns the transport listener but not Conduit's application
/// state. Command execution should be delegated to the application command
/// dispatcher/EventBus.
pub struct IpcServer {
    config: IpcServerConfig,
    running: bool,
    next_event_sequence: u64,
}

impl IpcServer {
    pub fn new(config: IpcServerConfig) -> Self {
        Self {
            config,
            running: false,
            next_event_sequence: 1,
        }
    }

    pub fn default_config() -> IpcServerConfig {
        IpcServerConfig::default()
    }

    pub fn config(&self) -> &IpcServerConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut IpcServerConfig {
        &mut self.config
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Starts the server transport.
    ///
    /// Listener integration is intentionally kept at the platform/runtime
    /// layer. This method establishes the logical running state.
    pub fn start(&mut self) -> Result<(), IpcServerError> {
        if !self.config.enabled {
            return Err(IpcServerError::Disabled);
        }

        self.running = true;
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn next_event_sequence(&mut self) -> u64 {
        let sequence = self.next_event_sequence;
        self.next_event_sequence = self.next_event_sequence.wrapping_add(1);
        sequence
    }

    /// Handles one decoded request.
    ///
    /// This is intentionally a dispatch boundary. The actual application
    /// implementation should replace or wrap this function and forward
    /// commands into Conduit's normal command/EventBus architecture.
    pub fn handle_request(
        &mut self,
        request: IpcRequest,
    ) -> IpcEnvelope {
        let result = self.dispatch(&request.command);

        IpcEnvelope::new(IpcMessage::Response(
            super::protocol::IpcResponse::success(request.id, result),
        ))
    }

    fn dispatch(&self, command: &IpcCommand) -> IpcCommandResult {
        match command {
            IpcCommand::Ping => IpcCommandResult::Pong,

            IpcCommand::GetVersion => IpcCommandResult::Version {
                version: env!("CARGO_PKG_VERSION").to_string(),
                protocol: super::protocol::IpcVersion::CURRENT.to_string(),
            },

            IpcCommand::GetStatus => IpcCommandResult::Status {
                running: self.running,
                windows: 0,
                tabs: 0,
                panes: 0,
            },

            _ => IpcCommandResult::error(
                "not_implemented",
                "IPC command must be connected to the Conduit application dispatcher",
            ),
        }
    }

    /// Encodes a response for transmission.
    pub fn encode_response(
        &self,
        envelope: &IpcEnvelope,
    ) -> Result<Vec<u8>, IpcServerError> {
        JsonCodec::encode(envelope)
            .map_err(|error| IpcServerError::Protocol(error.to_string()))
    }

    pub fn socket_path(&self) -> Option<PathBuf> {
        self.config.address.path().map(PathBuf::from)
    }
}
