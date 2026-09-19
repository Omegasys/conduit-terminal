//! Conduit terminal emulator library.
//!
//! The library contains the shared core used by Conduit's GUI, TUI,
//! and CLI interfaces.

pub mod core;

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
