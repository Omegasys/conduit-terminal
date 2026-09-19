//! Conduit's shared terminal core.
//!
//! The core intentionally does not depend on a GUI, TUI, or specific
//! rendering backend. Higher-level interfaces communicate with these
//! components through the public core API and, later, through Conduit's
//! event bus.

pub mod process;
pub mod pty;
pub mod screen;
pub mod session;
pub mod terminal;

pub use process::{
    Process,
    ProcessConfig,
    ProcessState,
};

pub use pty::{
    Pty,
    PtyConfig,
};

pub use screen::{
    Screen,
    ScreenCell,
};

pub use session::{
    Session,
    SessionConfig,
    SessionState,
};

pub use terminal::Terminal;
