pub mod application;
pub mod dashboard;
pub mod panes;
pub mod settings;
pub mod tabs;
pub mod windows;

pub use application::{
    TuiApplication,
    TuiApplicationState,
    TuiExitReason,
};

pub use dashboard::{
    DashboardPanel,
    DashboardState,
    DashboardView,
};

pub use panes::{
    PaneDisplay,
    PaneDisplayState,
    PaneFocusMode,
    TuiPaneView,
};

pub use settings::{
    SettingsCategory,
    SettingsItem,
    SettingsState,
    SettingsView,
};

pub use tabs::{
    TabDisplay,
    TabDisplayState,
    TabList,
    TuiTabView,
};

pub use windows::{
    TuiWindow,
    TuiWindowManager,
    WindowDisplayState,
};
pub mod command_palette;
pub mod diagnostics;
pub mod plugins;
pub mod security_center;
pub mod sessions;
pub mod workspaces;

pub use command_palette::{
    TuiCommand,
    TuiCommandCategory,
    TuiCommandMatch,
    TuiCommandPalette,
    TuiCommandState,
};

pub use diagnostics::{
    DiagnosticEntry,
    DiagnosticLevel,
    DiagnosticSource,
    TuiDiagnosticsView,
};

pub use plugins::{
    PluginDisplay,
    PluginDisplayState,
    PluginPermission,
    PluginTrustLevel,
    TuiPluginView,
};

pub use security_center::{
    PermissionDisplay,
    PermissionDisplayState,
    RestrictionDisplay,
    RestrictionDisplayState,
    SecurityIndicator,
    SecurityLevel,
    SecurityPanel,
    SecurityWarningDisplay,
    SecurityWarningSeverity,
    TuiSecurityCenter,
};

pub use sessions::{
    SessionDisplay,
    SessionDisplayState,
    SessionKind,
    TuiSessionId,
    TuiSessionView,
};

pub use workspaces::{
    TuiWorkspaceView,
    WorkspaceDisplay,
    WorkspaceDisplayState,
};
pub mod configuration_editor;
pub mod connections;
pub mod flow_view;
pub mod history;

pub use configuration_editor::{
    ConfigEditorLine,
    ConfigEditorLineKind,
    ConfigEditorMode,
    EditorCursor,
    EditorSelection,
    TuiConfigurationEditor,
};

pub use connections::{
    ConnectionDisplay,
    ConnectionKind,
    ConnectionState,
    TuiConnectionView,
};

pub use flow_view::{
    FlowDisplayStyle,
    FlowViewport,
    TuiFlowMode,
    TuiFlowView,
};

pub use history::{
    HistoryEntry,
    HistoryEntryKind,
    HistoryFilter,
    TuiHistoryView,
};
