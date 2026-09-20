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
