//! Conduit pane management.
//!
//! A pane is a logical terminal viewport inside a tab. Panes own layout
//! relationships and references to terminal sessions, while the terminal
//! core owns the actual PTY and process lifecycle.

pub mod layout;
pub mod manager;
pub mod pane;
pub mod split;

pub use layout::{
    LayoutNode,
    PaneLayout,
};

pub use manager::{
    PaneManager,
    PaneManagerError,
};

pub use pane::{
    Pane,
    PaneId,
    PaneOrientation,
};

pub use split::{
    SplitDirection,
    SplitRatio,
};
