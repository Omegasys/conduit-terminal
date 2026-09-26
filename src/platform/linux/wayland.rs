use std::env;
use std::fmt;

/// Information about the current Wayland environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaylandInfo {
    pub display: Option<String>,
    pub desktop: Option<String>,
    pub compositor: Option<String>,
}

impl WaylandInfo {
    pub fn detect() -> Self {
        Self {
            display: env::var("WAYLAND_DISPLAY").ok(),
            desktop: env::var("XDG_CURRENT_DESKTOP").ok(),
            compositor: env::var("XDG_SESSION_DESKTOP").ok(),
        }
    }

    pub fn is_available(&self) -> bool {
        self.display.is_some()
    }
}

/// Wayland platform integration.
#[derive(Debug, Clone, Default)]
pub struct Wayland;

impl Wayland {
    pub fn new() -> Self {
        Self
    }

    pub fn detect() -> WaylandInfo {
        WaylandInfo::detect()
    }

    pub fn is_available() -> bool {
        env::var_os("WAYLAND_DISPLAY").is_some()
    }

    pub fn display_name() -> Option<String> {
        env::var("WAYLAND_DISPLAY").ok()
    }

    /// Returns the runtime directory used by Wayland.
    pub fn runtime_directory() -> Option<String> {
        env::var("XDG_RUNTIME_DIR").ok()
    }

    /// Returns whether the current session appears to be Wayland.
    pub fn is_current_session() -> bool {
        env::var("XDG_SESSION_TYPE")
            .map(|value| value.eq_ignore_ascii_case("wayland"))
            .unwrap_or_else(|_| Self::is_available())
    }
}

/// Errors associated with Wayland integration.
#[derive(Debug)]
pub enum WaylandError {
    NotAvailable,
    ConnectionFailed(String),
    Unsupported(String),
}

impl fmt::Display for WaylandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAvailable => formatter.write_str("Wayland is not available"),
            Self::ConnectionFailed(message) => {
                write!(formatter, "Wayland connection failed: {message}")
            }
            Self::Unsupported(message) => {
                write!(formatter, "Wayland operation unsupported: {message}")
            }
        }
    }
}

impl std::error::Error for WaylandError {}
