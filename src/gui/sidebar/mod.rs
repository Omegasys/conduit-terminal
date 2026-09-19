pub mod connections;
pub mod navigation;
pub mod sessions;
pub mod sidebar;
pub mod state;
pub mod tools;
pub mod workspaces;

pub use connections::{
    ConnectionEntry,
    ConnectionKind,
    ConnectionManager,
    ConnectionState,
};

pub use navigation::{
    NavigationEntry,
    NavigationManager,
    NavigationTarget,
};

pub use sessions::{
    SessionEntry,
    SessionManager,
    SessionState,
};

pub use sidebar::{
    Sidebar,
    SidebarPanel,
    SidebarPosition,
};

pub use state::{
    SidebarState,
    SidebarStateSnapshot,
};

pub use tools::{
    SidebarTool,
    SidebarToolManager,
    ToolCategory,
};

pub use workspaces::{
    SidebarWorkspace,
    SidebarWorkspaceManager,
};
