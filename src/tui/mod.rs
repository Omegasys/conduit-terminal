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
