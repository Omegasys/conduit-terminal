pub mod application;
pub mod profiles;
pub mod settings;
pub mod window;
pub mod workspaces;

pub use application::{
    GuiApplication,
    GuiApplicationState,
};

pub use profiles::{
    GuiProfile,
    GuiProfileManager,
};

pub use settings::{
    GuiSettings,
    GuiSettingsState,
};

pub use window::{
    GuiWindow,
    GuiWindowId,
    GuiWindowManager,
    GuiWindowState,
    GuiWindowType,
};

pub use workspaces::{
    GuiWorkspace,
    GuiWorkspaceManager,
};
pub mod accessibility;
pub mod diagnostics;
pub mod keyboard;
pub mod mouse;
pub mod security;
pub mod themes;

pub use accessibility::AccessibilitySettings;

pub use diagnostics::{
    DiagnosticLevel,
    DiagnosticMessage,
    DiagnosticsManager,
};

pub use keyboard::{
    Key,
    KeyBinding,
    KeyCombination,
    KeyModifier,
    KeyboardAction,
    KeyboardManager,
};

pub use mouse::{
    MouseAction,
    MouseButton,
    MousePosition,
    MouseSettings,
    MouseState,
};

pub use security::{
    GuiSecurityMode,
    GuiSecuritySettings,
    SecurityDecision,
};

pub use themes::{
    GuiTheme,
    GuiThemeManager,
};
pub mod toolbar;

pub use toolbar::{
    Accelerator,
    AcceleratorManager,
    FileMenu,
    Menu,
    MenuAction,
    MenuBar,
    MenuItem,
    MenuItemKind,
    EditMenu,
    HelpMenu,
    PanesMenu,
    SessionMenu,
    TabsMenu,
    TerminalMenu,
    ToolsMenu,
    ViewMenu,
    Toolbar,
    ToolbarAction,
    ToolbarButton,
    ToolbarButtonGroup,
    ToolbarButtonKind,
    ToolbarLayout,
    ToolbarPosition,
    OverflowItem,
    OverflowMenu,
};
pub mod tab_bar;

pub use tab_bar::{
    TabBar,
    TabBarControl,
    TabBarControlAction,
    TabBarControls,
    TabBarLayout,
    TabBarTab,
    TabBarTabState,
    TabBarOverflowMode,
    TabContextAction,
    TabContextMenu,
};
pub mod status_bar;

pub use status_bar::{
    IndicatorKind,
    StatusBar,
    StatusBarLayout,
    StatusBarPosition,
    StatusIndicator,
    StatusIndicatorManager,
    StatusIndicatorState,
    StatusSegment,
    StatusSegmentAlignment,
    StatusSegmentKind,
    StatusSegmentManager,
};
pub mod sidebar;

pub use sidebar::{
    ConnectionEntry,
    ConnectionKind,
    ConnectionManager,
    ConnectionState,
    NavigationEntry,
    NavigationManager,
    NavigationTarget,
    SessionEntry,
    SessionManager,
    SessionState,
    Sidebar,
    SidebarPanel,
    SidebarPosition,
    SidebarState,
    SidebarStateSnapshot,
    SidebarTool,
    SidebarToolManager,
    SidebarWorkspace,
    SidebarWorkspaceManager,
    ToolCategory,
};
pub mod command_palette;

pub use command_palette::{
    CommandCategory,
    CommandCategoryManager,
    CommandEntry,
    CommandEntryState,
    CommandHistory,
    CommandHistoryEntry,
    CommandManager,
    CommandPalette,
    CommandPaletteMode,
    CommandPaletteState,
    CommandSearch,
    CommandSearchMatch,
    SearchMatchKind,
};
pub mod settings_ui;

pub use settings_ui::{
    ApplicationSettingsUi,
    ApplicationSettingsUiState,
    AppearanceSettings,
    BellStyle,
    ColorScheme,
    ColorSettings,
    CursorBlinkMode,
    CursorStyle,
    FontSettings,
    FontWeight,
    GeneralSettings,
    ScrollbarMode,
    SettingsPage,
    SettingsUiNavigation,
    StartupBehavior,
    TabBarStyle,
    TerminalSettings,
    ThemeMode,
};
