use std::{
    fmt,
    io,
};

#[derive(Debug)]
pub enum CustomShellError {
    Io(io::Error),

    InvalidManifest(String),

    InvalidShell(String),

    AlreadyRegistered(String),

    NotFound(String),

    BufferTooLarge {
        maximum: usize,
    },

    Parser(String),

    SecurityDenied(String),

    Unsupported(String),

    InitializationFailed(String),

    ExecutionFailed(String),
}

impl fmt::Display for CustomShellError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Io(error) => {
                write!(f, "I/O error: {error}")
            }

            Self::InvalidManifest(message) => {
                write!(
                    f,
                    "invalid shell manifest: {message}"
                )
            }

            Self::InvalidShell(message) => {
                write!(
                    f,
                    "invalid custom shell: {message}"
                )
            }

            Self::AlreadyRegistered(id) => {
                write!(
                    f,
                    "shell already registered: {id}"
                )
            }

            Self::NotFound(id) => {
                write!(
                    f,
                    "custom shell not found: {id}"
                )
            }

            Self::BufferTooLarge {
                maximum,
            } => {
                write!(
                    f,
                    "shell parser buffer exceeds {maximum} bytes"
                )
            }

            Self::Parser(message) => {
                write!(
                    f,
                    "shell parser error: {message}"
                )
            }

            Self::SecurityDenied(message) => {
                write!(
                    f,
                    "shell security operation denied: {message}"
                )
            }

            Self::Unsupported(message) => {
                write!(
                    f,
                    "unsupported shell feature: {message}"
                )
            }

            Self::InitializationFailed(
                message,
            ) => {
                write!(
                    f,
                    "shell initialization failed: {message}"
                )
            }

            Self::ExecutionFailed(message) => {
                write!(
                    f,
                    "shell command execution failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for CustomShellError {}

impl From<io::Error> for CustomShellError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub type CustomShellResult<T> =
    Result<T, CustomShellError>;
