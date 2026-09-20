pub mod application;
pub mod appearance;
pub mod colors;
pub mod fonts;
pub mod general;
pub mod navigation;
pub mod terminal;

pub use application::{
    ApplicationSettingsUi,
    ApplicationSettingsUiState,
};

pub use appearance::{
    AppearanceSettings,
    CursorStyle,
    TabBarStyle,
    ThemeMode,
};

pub use colors::{
    ColorSettings,
    ColorScheme,
};

pub use fonts::{
    FontSettings,
    FontWeight,
};

pub use general::{
    GeneralSettings,
    StartupBehavior,
};

pub use navigation::{
    SettingsPage,
    SettingsUiNavigation,
};

pub use terminal::{
    BellStyle,
    CursorBlinkMode,
    ScrollbarMode,
    TerminalSettings,
};
pub mod advanced;
pub mod application;
pub mod appearance;
pub mod colors;
pub mod developer;
pub mod fonts;
pub mod general;
pub mod history;
pub mod navigation;
pub mod recording;
pub mod terminal;
pub mod themes;

pub use advanced::{AdvancedSettings, ReloadMode};

pub use application::{
    ApplicationSettingsUi,
    ApplicationSettingsUiState,
};

pub use appearance::{
    AppearanceSettings,
    CursorStyle,
    TabBarStyle,
    ThemeMode,
};

pub use colors::{
    ColorScheme,
    ColorSettings,
};

pub use developer::{
    DeveloperLogLevel,
    DeveloperSettings,
};

pub use fonts::{
    FontSettings,
    FontWeight,
};

pub use general::{
    GeneralSettings,
    StartupBehavior,
};

pub use history::{
    DuplicateHistoryMode,
    HistoryMode,
    HistorySettings,
};

pub use navigation::{
    SettingsPage,
    SettingsUiNavigation,
};

pub use recording::{
    RecordingFormat,
    RecordingMode,
    RecordingSettings,
};

pub use terminal::{
    BellStyle,
    CursorBlinkMode,
    ScrollbarMode,
    TerminalSettings,
};

pub use themes::{
    ThemeScheduleMode,
    ThemeSelectionMode,
    ThemeSettings,
};
