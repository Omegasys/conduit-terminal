//! Platform integration for Conduit.
//!
//! Platform-specific functionality lives here rather than inside the core
//! application. This keeps the terminal, shell, configuration, and runtime
//! systems portable while allowing native integrations where appropriate.

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use linux::{
    DesktopEnvironment,
    DisplayServer,
    LinuxDesktop,
    LinuxFilesystem,
    LinuxNotification,
    LinuxNotificationManager,
    LinuxProcess,
    LinuxProcessManager,
    LinuxSession,
    Wayland,
    X11,
};
