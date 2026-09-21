use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CustomProtocolError {
    Io(io::Error),

    InvalidManifest(String),
    InvalidProtocol(String),

    AlreadyRegistered(String),
    NotFound(String),

    BufferTooLarge {
        maximum: usize,
    },

    Parser(String),
    Encoder(String),
    Decoder(String),

    SecurityDenied(String),
    Unsupported(String),

    InitializationFailed(String),
}

impl fmt::Display for CustomProtocolError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Io(error) => {
                write!(formatter, "I/O error: {error}")
            }

            Self::InvalidManifest(message) => {
                write!(
                    formatter,
                    "Invalid protocol manifest: {message}"
                )
            }

            Self::InvalidProtocol(message) => {
                write!(
                    formatter,
                    "Invalid protocol: {message}"
                )
            }

            Self::AlreadyRegistered(id) => {
                write!(
                    formatter,
                    "Protocol already registered: {id}"
                )
            }

            Self::NotFound(id) => {
                write!(
                    formatter,
                    "Protocol not found: {id}"
                )
            }

            Self::BufferTooLarge { maximum } => {
                write!(
                    formatter,
                    "Protocol buffer exceeded maximum size of {maximum} bytes"
                )
            }

            Self::Parser(message) => {
                write!(formatter, "Protocol parser error: {message}")
            }

            Self::Encoder(message) => {
                write!(formatter, "Protocol encoder error: {message}")
            }

            Self::Decoder(message) => {
                write!(formatter, "Protocol decoder error: {message}")
            }

            Self::SecurityDenied(message) => {
                write!(
                    formatter,
                    "Protocol security permission denied: {message}"
                )
            }

            Self::Unsupported(message) => {
                write!(
                    formatter,
                    "Unsupported protocol feature: {message}"
                )
            }

            Self::InitializationFailed(message) => {
                write!(
                    formatter,
                    "Protocol initialization failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for CustomProtocolError {}

impl From<io::Error> for CustomProtocolError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub type CustomProtocolResult<T> =
    Result<T, CustomProtocolError>;
