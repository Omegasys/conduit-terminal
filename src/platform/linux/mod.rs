//! Linux platform integration.
//!
//! This module provides native Linux integration for Conduit while keeping
//! platform details isolated from the rest of the application.

pub mod dbus;
pub mod desktop;
pub mod filesystem;
pub mod notifications;
pub mod process;
pub mod wayland;
pub mod x11;

pub use dbus::{
    DbusConnection,
    DbusError,
    DbusManager,
};
pub use desktop::{
    DesktopEnvironment,
    DisplayServer,
    LinuxDesktop,
    LinuxSession,
};
pub use filesystem::{
    FileMetadata,
    LinuxFilesystem,
    LinuxFilesystemError,
};
pub use notifications::{
    LinuxNotification,
    LinuxNotificationManager,
    NotificationUrgency,
};
pub use process::{
    LinuxProcess,
    LinuxProcessError,
    LinuxProcessManager,
};
pub use wayland::{
    Wayland,
    WaylandError,
    WaylandInfo,
};
pub use x11::{
    X11,
    X11Error,
    X11Info,
};
