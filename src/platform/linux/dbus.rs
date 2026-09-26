use std::collections::BTreeMap;
use std::fmt;

/// D-Bus connection type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbusConnection {
    Session,
    System,
}

impl DbusConnection {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Session => "session",
            Self::System => "system",
        }
    }
}

/// Basic D-Bus manager.
///
/// This provides the platform boundary for D-Bus operations. Actual D-Bus
/// transport can later be implemented using zbus or another D-Bus library.
#[derive(Debug, Default)]
pub struct DbusManager {
    environment: BTreeMap<String, String>,
}

impl DbusManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn session_available() -> bool {
        std::env::var_os("DBUS_SESSION_BUS_ADDRESS").is_some()
    }

    pub fn system_available() -> bool {
        std::path::Path::new("/run/dbus/system_bus_socket").exists()
    }

    pub fn is_available(&self, connection: DbusConnection) -> bool {
        match connection {
            DbusConnection::Session => Self::session_available(),
            DbusConnection::System => Self::system_available(),
        }
    }

    pub fn set_environment(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.environment.insert(key.into(), value.into());
    }

    pub fn environment(&self) -> impl Iterator<Item = (&String, &String)> {
        self.environment.iter()
    }
}

/// D-Bus errors.
#[derive(Debug)]
pub enum DbusError {
    Unavailable(DbusConnection),
    ConnectionFailed(String),
    MethodFailed(String),
    InvalidMessage(String),
}

impl fmt::Display for DbusError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unavailable(connection) => {
                write!(formatter, "{} D-Bus is unavailable", connection.as_str())
            }
            Self::ConnectionFailed(message) => {
                write!(formatter, "D-Bus connection failed: {message}")
            }
            Self::MethodFailed(message) => {
                write!(formatter, "D-Bus method failed: {message}")
            }
            Self::InvalidMessage(message) => {
                write!(formatter, "invalid D-Bus message: {message}")
            }
        }
    }
}

impl std::error::Error for DbusError {}
