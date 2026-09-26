//! Inter-process communication for Conduit.
//!
//! IPC provides a local control and event channel between Conduit instances,
//! the CLI, GUI/TUI frontends, plugins, and external automation tools.
//!
//! The IPC architecture is split into:
//!
//! - `protocol` - protocol-level messages and envelopes.
//! - `commands` - commands that can be sent to Conduit.
//! - `events` - events emitted by Conduit.
//! - `socket` - local transport abstraction.
//! - `json` - JSON serialization.
//! - `server` - IPC server.
//! - `client` - IPC client.
//!
//! The IPC layer does not own Conduit's runtime state. Commands should be
//! translated into the normal EventBus/application command system.

pub mod client;
pub mod commands;
pub mod events;
pub mod json;
pub mod protocol;
pub mod server;
pub mod socket;

pub use client::{IpcClient, IpcClientError};
pub use commands::{IpcCommand, IpcCommandResult};
pub use events::{IpcEvent, IpcEventKind};
pub use protocol::{
    IpcEnvelope, IpcMessage, IpcRequest, IpcResponse, IpcVersion,
};
pub use server::{IpcServer, IpcServerConfig, IpcServerError};
pub use socket::{IpcSocket, IpcSocketError, SocketAddress};
