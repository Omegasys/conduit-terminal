use std::fmt;

use super::protocol::{
    IpcEnvelope, IpcMessage, IpcRequest, IpcResponse, IpcVersion,
};

/// Errors produced by the IPC JSON codec.
#[derive(Debug)]
pub enum JsonError {
    InvalidUtf8(std::str::Utf8Error),
    InvalidFormat(String),
}

impl fmt::Display for JsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUtf8(error) => write!(formatter, "invalid UTF-8: {error}"),
            Self::InvalidFormat(message) => {
                write!(formatter, "invalid IPC JSON: {message}")
            }
        }
    }
}

impl std::error::Error for JsonError {}

/// JSON codec boundary for IPC messages.
///
/// The actual schema is intentionally isolated here so that the rest of the
/// IPC system does not depend on a particular wire serialization format.
///
/// When serde/serde_json are enabled in the project, this module is the
/// appropriate place to replace the lightweight implementation with the
/// complete JSON protocol.
pub struct JsonCodec;

impl JsonCodec {
    pub fn encode(envelope: &IpcEnvelope) -> Result<Vec<u8>, JsonError> {
        let message = match &envelope.message {
            IpcMessage::Request(request) => format!(
                "{{\"type\":\"request\",\"id\":{},\"version\":\"{}\",\"command\":{}}}",
                request.id,
                request.version,
                Self::encode_command(&request.command)
            ),

            IpcMessage::Response(response) => format!(
                "{{\"type\":\"response\",\"id\":{},\"version\":\"{}\",\"result\":{}}}",
                response.id,
                response.version,
                Self::encode_result(&response.result)
            ),

            IpcMessage::Event(event) => format!(
                "{{\"type\":\"event\",\"sequence\":{},\"kind\":\"{:?}\"}}",
                event.sequence,
                event.sequence
            ),
        };

        Ok(message.into_bytes())
    }

    pub fn decode(bytes: &[u8]) -> Result<IpcEnvelope, JsonError> {
        let text = std::str::from_utf8(bytes)
            .map_err(JsonError::InvalidUtf8)?;

        let trimmed = text.trim();

        if trimmed.is_empty() {
            return Err(JsonError::InvalidFormat(
                "empty message".to_string(),
            ));
        }

        // This parser intentionally only recognizes the protocol envelope
        // enough to reject malformed/unknown messages. Full command/event
        // deserialization belongs here once serde is enabled.
        if trimmed.contains("\"type\":\"request\"") {
            return Err(JsonError::InvalidFormat(
                "request decoding requires the full JSON schema implementation"
                    .to_string(),
            ));
        }

        Err(JsonError::InvalidFormat(
            "unsupported IPC message".to_string(),
        ))
    }

    fn encode_command(command: &super::commands::IpcCommand) -> String {
        format!("\"{:?}\"", command)
    }

    fn encode_result(result: &super::commands::IpcCommandResult) -> String {
        format!("\"{:?}\"", result)
    }

    pub fn request(
        id: u64,
        command: super::commands::IpcCommand,
    ) -> IpcEnvelope {
        IpcEnvelope {
            version: IpcVersion::CURRENT,
            message: IpcMessage::Request(IpcRequest::new(id, command)),
        }
    }

    pub fn response(
        id: u64,
        result: super::commands::IpcCommandResult,
    ) -> IpcEnvelope {
        IpcEnvelope {
            version: IpcVersion::CURRENT,
            message: IpcMessage::Response(IpcResponse::success(id, result)),
        }
    }
}
