//! Conduit workspace management.
//!
//! A workspace is a persistent logical collection of windows, tabs, panes,
//! and their associated layout information. Workspaces coordinate these
//! resources without directly owning the terminal processes or PTYs.

pub mod create;
pub mod layouts;
pub mod manager;
pub mod restore;
pub mod save;
pub mod state;
pub mod switching;
pub mod workspace;

pub use create::{
    WorkspaceCreateOptions,
    WorkspaceCreator,
};

pub use layouts::{
    WorkspaceLayout,
    WorkspaceLayoutMode,
};

pub use manager::{
    WorkspaceManager,
    WorkspaceManagerError,
};

pub use restore::{
    WorkspaceRestoreOptions,
    WorkspaceRestorer,
};

pub use save::{
    WorkspaceSaveOptions,
    WorkspaceSaver,
};

pub use state::{
    WorkspaceState,
};

pub use switching::{
    WorkspaceSwitcher,
};

pub use workspace::{
    Workspace,
    WorkspaceId,
    WorkspaceStatus,
};
