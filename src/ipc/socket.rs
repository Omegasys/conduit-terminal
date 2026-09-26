use std::fmt;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// Address used by an IPC transport.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketAddress {
    Unix(PathBuf),
    Tcp {
        host: String,
        port: u16,
    },
}

impl SocketAddress {
    pub fn unix(path: impl Into<PathBuf>) -> Self {
        Self::Unix(path.into())
    }

    pub fn tcp(host: impl Into<String>, port: u16) -> Self {
        Self::Tcp {
            host: host.into(),
            port,
        }
    }

    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Unix(path) => Some(path),
            Self::Tcp { .. } => None,
        }
    }
}

impl Default for SocketAddress {
    fn default() -> Self {
        #[cfg(unix)]
        {
            Self::Unix(default_socket_path())
        }

        #[cfg(not(unix))]
        {
            Self::Tcp {
                host: "127.0.0.1".to_string(),
                port: 49321,
            }
        }
    }
}

fn default_socket_path() -> PathBuf {
    if let Some(runtime) = std::env::var_os("XDG_RUNTIME_DIR") {
        return PathBuf::from(runtime).join("conduit.sock");
    }

    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join(".cache")
            .join("conduit")
            .join("conduit.sock");
    }

    PathBuf::from("/tmp/conduit.sock")
}

/// Errors produced by IPC socket operations.
#[derive(Debug)]
pub enum IpcSocketError {
    Io(io::Error),
    UnsupportedTransport,
    InvalidAddress,
}

impl fmt::Display for IpcSocketError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "IPC socket I/O error: {error}"),
            Self::UnsupportedTransport => {
                formatter.write_str("requested IPC transport is unsupported")
            }
            Self::InvalidAddress => formatter.write_str("invalid IPC socket address"),
        }
    }
}

impl std::error::Error for IpcSocketError {}

impl From<io::Error> for IpcSocketError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Generic framed IPC socket.
///
/// The initial transport implementation uses a length-prefixed byte stream.
/// Serialization is intentionally kept outside this type.
pub struct IpcSocket<S> {
    stream: S,
}

impl<S> IpcSocket<S> {
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    pub fn into_inner(self) -> S {
        self.stream
    }

    pub fn get_ref(&self) -> &S {
        &self.stream
    }

    pub fn get_mut(&mut self) -> &mut S {
        &mut self.stream
    }
}

impl<S: Read + Write> IpcSocket<S> {
    /// Sends one framed payload.
    pub fn send(&mut self, payload: &[u8]) -> Result<(), IpcSocketError> {
        let length = u32::try_from(payload.len())
            .map_err(|_| IpcSocketError::InvalidAddress)?;

        self.stream.write_all(&length.to_be_bytes())?;
        self.stream.write_all(payload)?;
        self.stream.flush()?;

        Ok(())
    }

    /// Receives one framed payload.
    pub fn receive(&mut self) -> Result<Vec<u8>, IpcSocketError> {
        let mut header = [0u8; 4];

        self.stream.read_exact(&mut header)?;

        let length = u32::from_be_bytes(header) as usize;

        // Prevent obviously unreasonable allocations from malformed peers.
        const MAX_FRAME_SIZE: usize = 64 * 1024 * 1024;

        if length > MAX_FRAME_SIZE {
            return Err(IpcSocketError::InvalidAddress);
        }

        let mut payload = vec![0u8; length];

        self.stream.read_exact(&mut payload)?;

        Ok(payload)
    }
}
