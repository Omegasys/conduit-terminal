use std::{fmt, io};

use super::protocol::ProtocolId;

#[derive(Debug)]
pub enum CustomProtocolError {
    Io(io::Error),

    InvalidManifest(String),

    InvalidProtocol(String),

    AlreadyRegistered(String),

    NotFound(String),

    SequenceTooLong {
        maximum: usize,
    },

    Parser(String),

    SecurityDenied(String),

    Unsupported(String),
}

impl fmt::Display for CustomProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),

            Self::InvalidManifest(message) => {
                write!(f, "invalid protocol manifest: {message}")
            }

            Self::InvalidProtocol(message) => {
                write!(f, "invalid protocol: {message}")
            }

            Self::AlreadyRegistered(id) => {
                write!(f, "protocol already registered: {id}")
            }

            Self::NotFound(id) => {
                write!(f, "protocol not found: {id}")
            }

            Self::SequenceTooLong { maximum } => {
                write!(
                    f,
                    "protocol sequence exceeds maximum length of {maximum} bytes"
                )
            }

            Self::Parser(message) => {
                write!(f, "protocol parser error: {message}")
            }

            Self::SecurityDenied(message) => {
                write!(f, "protocol security operation denied: {message}")
            }

            Self::Unsupported(message) => {
                write!(f, "unsupported protocol feature: {message}")
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

pub type CustomProtocolResult<T> = Result<T, CustomProtocolError>;

impl From<ProtocolId> for CustomProtocolError {
    fn from(id: ProtocolId) -> Self {
        Self::NotFound(id.to_string())
    }
}
