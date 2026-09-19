//! Conduit tab management.
//!
//! A tab belongs to a top-level window and provides a logical container
//! for one or more panes. The tab system intentionally does not depend
//! on a specific GUI, TUI, or rendering backend.

pub mod icons;
pub mod manager;
pub mod reordering;
pub mod restoration;
pub mod state;
pub mod tab;
pub mod title;

pub use icons::{
    TabIcon,
    TabIconSource,
};

pub use manager::{
    TabManager,
    TabManagerError,
};

pub use reordering::{
    TabDropPosition,
    TabReorder,
};

pub use restoration::{
    TabRestoreData,
    TabRestorePolicy,
};

pub use state::{
    TabActivity,
    TabState,
};

pub use tab::{
    Tab,
    TabId,
};
