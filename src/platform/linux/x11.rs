use std::env;
use std::fmt;

/// Information about the current X11 environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X11Info {
    pub display: Option<String>,
    pub desktop: Option<String>,
    pub session_type: Option<String>,
}

impl X11Info {
    pub fn detect() -> Self {
        Self {
            display: env::var("DISPLAY").ok(),
            desktop: env::var("XDG_CURRENT_DESKTOP").ok(),
            session_type: env::var("XDG_SESSION_TYPE").ok(),
        }
    }

    pub fn is_available(&self) -> bool {
        self.display.is_some()
    }
}

/// X11 platform integration.
#[derive(Debug, Clone, Default)]
pub struct X11;

impl X11 {
    pub fn new() -> Self {
        Self
    }

    pub fn detect() -> X11Info {
        X11Info::detect()
    }

    pub fn is_available() -> bool {
        env::var_os("DISPLAY").is_some()
    }

    pub fn display_name() -> Option<String> {
        env::var("DISPLAY").ok()
    }

    pub fn is_current_session() -> bool {
        env::var("XDG_SESSION_TYPE")
            .map(|value| value.eq_ignore_ascii_case("x11"))
            .unwrap_or_else(|_| Self::is_available())
    }
}

/// Errors associated with X11 integration.
#[derive(Debug)]
pub enum X11Error {
    NotAvailable,
    ConnectionFailed(String),
    Unsupported(String),
}

impl fmt::Display for X11Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAvailable => formatter.write_str("X11 is not available"),
            Self::ConnectionFailed(message) => {
                write!(formatter, "X11 connection failed: {message}")
            }
            Self::Unsupported(message) => {
                write!(formatter, "X11 operation unsupported: {message}")
            }
        }
    }
}

impl std::error::Error for X11Error {}
