//! Errors produced by the custom shell subsystem.

use std::fmt;

#[derive(Debug)]
pub enum CustomShellError {
    InvalidManifest(String),
    InvalidConfiguration(String),
    InvalidPrompt(String),
    InvalidSecurityPolicy(String),
    Validation(String),
    AlreadyRegistered(String),
    NotFound(String),
    ExecutableNotFound(String),
    Io(std::io::Error),
}

impl fmt::Display for CustomShellError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidManifest(message) => {
                write!(formatter, "invalid custom shell manifest: {message}")
            }
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid custom shell configuration: {message}")
            }
            Self::InvalidPrompt(message) => {
                write!(formatter, "invalid custom shell prompt: {message}")
            }
            Self::InvalidSecurityPolicy(message) => {
                write!(formatter, "invalid custom shell security policy: {message}")
            }
            Self::Validation(message) => {
                write!(formatter, "custom shell validation failed: {message}")
            }
            Self::AlreadyRegistered(id) => {
                write!(formatter, "custom shell already registered: {id}")
            }
            Self::NotFound(id) => {
                write!(formatter, "custom shell not found: {id}")
            }
            Self::ExecutableNotFound(executable) => {
                write!(formatter, "custom shell executable not found: {executable}")
            }
            Self::Io(error) => {
                write!(formatter, "custom shell I/O error: {error}")
            }
        }
    }
}

impl std::error::Error for CustomShellError {}

impl From<std::io::Error> for CustomShellError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub type CustomShellResult<T> = Result<T, CustomShellError>;
