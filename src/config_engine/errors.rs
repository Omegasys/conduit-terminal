use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),

    Parse(String),

    Serialization(String),

    Validation(String),

    Schema(String),

    ProfileNotFound(String),

    ProfileAlreadyExists(String),

    WorkspaceNotFound(String),

    WorkspaceAlreadyExists(String),

    ReadOnlyWorkspace(String),

    TransactionAlreadyCommitted,

    SnapshotNotFound(usize),

    FutureConfigVersion {
        found: u32,
        supported: u32,
    },

    MissingMigration {
        from: u32,
        to: u32,
    },

    Migration(String),

    InvalidConfigPath(String),

    InvalidValue {
        key: String,
        reason: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Io(error) => {
                write!(formatter, "configuration I/O error: {error}")
            }

            Self::Parse(error) => {
                write!(formatter, "configuration parse error: {error}")
            }

            Self::Serialization(error) => {
                write!(formatter, "configuration serialization error: {error}")
            }

            Self::Validation(error) => {
                write!(formatter, "configuration validation error: {error}")
            }

            Self::Schema(error) => {
                write!(formatter, "configuration schema error: {error}")
            }

            Self::ProfileNotFound(name) => {
                write!(formatter, "configuration profile not found: {name}")
            }

            Self::ProfileAlreadyExists(name) => {
                write!(formatter, "configuration profile already exists: {name}")
            }

            Self::WorkspaceNotFound(name) => {
                write!(formatter, "configuration workspace not found: {name}")
            }

            Self::WorkspaceAlreadyExists(name) => {
                write!(
                    formatter,
                    "configuration workspace already exists: {name}"
                )
            }

            Self::ReadOnlyWorkspace(name) => {
                write!(
                    formatter,
                    "configuration workspace is read-only: {name}"
                )
            }

            Self::TransactionAlreadyCommitted => {
                write!(formatter, "configuration transaction was already committed")
            }

            Self::SnapshotNotFound(index) => {
                write!(
                    formatter,
                    "configuration snapshot not found: {index}"
                )
            }

            Self::FutureConfigVersion {
                found,
                supported,
            } => {
                write!(
                    formatter,
                    "configuration version {found} is newer than supported version {supported}"
                )
            }

            Self::MissingMigration { from, to } => {
                write!(
                    formatter,
                    "no configuration migration registered from version {from} toward version {to}"
                )
            }

            Self::Migration(error) => {
                write!(formatter, "configuration migration error: {error}")
            }

            Self::InvalidConfigPath(path) => {
                write!(formatter, "invalid configuration path: {path}")
            }

            Self::InvalidValue { key, reason } => {
                write!(
                    formatter,
                    "invalid configuration value for '{key}': {reason}"
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
