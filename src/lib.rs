//! Conduit terminal emulator library.
//!
//! The library contains the shared core used by Conduit's GUI, TUI,
//! and CLI interfaces.

pub mod config_engine;
pub mod core;
pub mod events;
pub mod gui;
pub mod live;
pub mod panes;
pub mod resources;
pub mod tabs;
pub mod tui;
pub mod windows;
pub mod workspaces;
pub mod colors;
pub use core::{
    Process,
    ProcessConfig,
    ProcessState,
    Pty,
    PtyConfig,
    Screen,
    ScreenCell,
    Session,
    SessionConfig,
    SessionState,
    Terminal,
};
