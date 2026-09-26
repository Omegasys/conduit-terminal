use std::fmt;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

use super::{
    commands::{IpcCommand, IpcCommandResult},
    json::{JsonCodec, JsonError},
    protocol::{IpcEnvelope, IpcMessage},
    socket::{IpcSocket, SocketAddress},
};

/// Errors produced by an IPC client.
#[derive(Debug)]
pub enum IpcClientError {
    Socket(std::io::Error),
    Protocol(String),
    Json(JsonError),
    UnsupportedTransport,
}

impl fmt::Display for IpcClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Socket(error) => write!(formatter, "IPC client socket error: {error}"),
            Self::Protocol(message) => write!(formatter, "IPC protocol error: {message}"),
            Self::Json(error) => write!(formatter, "IPC JSON error: {error}"),
            Self::UnsupportedTransport => {
                formatter.write_str("IPC transport is unsupported on this platform")
            }
        }
    }
}

impl std::error::Error for IpcClientError {}

impl From<JsonError> for IpcClientError {
    fn from(error: JsonError) -> Self {
        Self::Json(error)
    }
}

/// Client used to communicate with a running Conduit instance.
pub struct IpcClient {
    socket: Option<IpcSocket<UnixStream>>,
    address: SocketAddress,
    next_request_id: u64,
}

impl IpcClient {
    pub fn new(address: SocketAddress) -> Self {
        Self {
            socket: None,
            address,
            next_request_id: 1,
        }
    }

    pub fn default_address() -> SocketAddress {
        SocketAddress::default()
    }

    /// Connects to the Conduit IPC server.
    pub fn connect(&mut self) -> Result<(), IpcClientError> {
        #[cfg(unix)]
        {
            let path = self
                .address
                .path()
                .ok_or(IpcClientError::UnsupportedTransport)?;

            let stream = UnixStream::connect(path)
                .map_err(IpcClientError::Socket)?;

            self.socket = Some(IpcSocket::new(stream));

            Ok(())
        }

        #[cfg(not(unix))]
        {
            Err(IpcClientError::UnsupportedTransport)
        }
    }

    pub fn is_connected(&self) -> bool {
        self.socket.is_some()
    }

    pub fn disconnect(&mut self) {
        self.socket = None;
    }

    /// Sends a command and waits for its response.
    pub fn send_command(
        &mut self,
        command: IpcCommand,
    ) -> Result<IpcCommandResult, IpcClientError> {
        let id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);

        let envelope = JsonCodec::request(id, command);
        let bytes = JsonCodec::encode(&envelope)?;

        let socket = self.socket.as_mut().ok_or_else(|| {
            IpcClientError::Protocol("client is not connected".to_string())
        })?;

        socket
            .send(&bytes)
            .map_err(|error| IpcClientError::Protocol(error.to_string()))?;

        let response = socket
            .receive()
            .map_err(|error| IpcClientError::Protocol(error.to_string()))?;

        let envelope = JsonCodec::decode(&response)?;

        match envelope.message {
            IpcMessage::Response(response) if response.id == id => {
                Ok(response.result)
            }

            IpcMessage::Response(_) => Err(IpcClientError::Protocol(
                "response ID does not match request".to_string(),
            )),

            _ => Err(IpcClientError::Protocol(
                "expected IPC response".to_string(),
            )),
        }
    }
}
