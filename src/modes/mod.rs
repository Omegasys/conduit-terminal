//! Conduit application modes.
//!
//! Modes control how much of the Conduit interface is visible and which
//! features are available. The mode system is shared by GUI and TUI layers.
//!
//! Modes are intentionally descriptive rather than directly manipulating
//! windows or widgets. The GUI/TUI layers translate a mode's presentation
//! policy into their own UI operations.

pub mod distraction_free;
pub mod experimental;
pub mod fullscreen;
pub mod full;
pub mod manager;
pub mod minimal;
pub mod safe;
pub mod terminal_only;

pub use distraction_free::DistractionFreeMode;
pub use experimental::ExperimentalMode;
pub use fullscreen::FullscreenMode;
pub use full::FullMode;
pub use manager::{ModeError, ModeId, ModeManager, ModeState};
pub use minimal::MinimalMode;
pub use safe::SafeMode;
pub use terminal_only::TerminalOnlyMode;

/// Describes the visual and functional policy of an application mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModePolicy {
    /// Whether normal application chrome is visible.
    pub show_chrome: bool,

    /// Whether menus should be available.
    pub show_menus: bool,

    /// Whether the toolbar should be visible.
    pub show_toolbar: bool,

    /// Whether the tab bar should be visible.
    pub show_tab_bar: bool,

    /// Whether the sidebar should be visible.
    pub show_sidebar: bool,

    /// Whether the status bar should be visible.
    pub show_status_bar: bool,

    /// Whether the terminal remains the primary interface.
    pub terminal_primary: bool,

    /// Whether the application should occupy the entire display.
    pub fullscreen: bool,

    /// Whether non-essential UI should be hidden.
    pub distraction_free: bool,

    /// Whether potentially dangerous features should be restricted.
    pub safe_mode: bool,

    /// Whether experimental features are enabled.
    pub experimental: bool,

    /// Whether plugins may be loaded.
    pub allow_plugins: bool,

    /// Whether external integrations may be used.
    pub allow_integrations: bool,
}

impl ModePolicy {
    pub const fn full() -> Self {
        Self {
            show_chrome: true,
            show_menus: true,
            show_toolbar: true,
            show_tab_bar: true,
            show_sidebar: true,
            show_status_bar: true,
            terminal_primary: true,
            fullscreen: false,
            distraction_free: false,
            safe_mode: false,
            experimental: false,
            allow_plugins: true,
            allow_integrations: true,
        }
    }

    pub const fn minimal() -> Self {
        Self {
            show_chrome: false,
            show_menus: false,
            show_toolbar: false,
            show_tab_bar: true,
            show_sidebar: false,
            show_status_bar: false,
            terminal_primary: true,
            fullscreen: false,
            distraction_free: true,
            safe_mode: false,
            experimental: false,
            allow_plugins: true,
            allow_integrations: true,
        }
    }

    pub const fn terminal_only() -> Self {
        Self {
            show_chrome: false,
            show_menus: false,
            show_toolbar: false,
            show_tab_bar: false,
            show_sidebar: false,
            show_status_bar: false,
            terminal_primary: true,
            fullscreen: true,
            distraction_free: true,
            safe_mode: false,
            experimental: false,
            allow_plugins: true,
            allow_integrations: true,
        }
    }

    pub const fn fullscreen() -> Self {
        Self {
            show_chrome: true,
            show_menus: true,
            show_toolbar: true,
            show_tab_bar: true,
            show_sidebar: true,
            show_status_bar: true,
            terminal_primary: true,
            fullscreen: true,
            distraction_free: false,
            safe_mode: false,
            experimental: false,
            allow_plugins: true,
            allow_integrations: true,
        }
    }

    pub const fn distraction_free() -> Self {
        Self {
            show_chrome: false,
            show_menus: false,
            show_toolbar: false,
            show_tab_bar: true,
            show_sidebar: false,
            show_status_bar: false,
            terminal_primary: true,
            fullscreen: false,
            distraction_free: true,
            safe_mode: false,
            experimental: false,
            allow_plugins: true,
            allow_integrations: true,
        }
    }

    pub const fn safe() -> Self {
        Self {
            show_chrome: true,
            show_menus: true,
            show_toolbar: true,
            show_tab_bar: true,
            show_sidebar: true,
            show_status_bar: true,
            terminal_primary: true,
            fullscreen: false,
            distraction_free: false,
            safe_mode: true,
            experimental: false,
            allow_plugins: false,
            allow_integrations: false,
        }
    }

    pub const fn experimental() -> Self {
        Self {
            show_chrome: true,
            show_menus: true,
            show_toolbar: true,
            show_tab_bar: true,
            show_sidebar: true,
            show_status_bar: true,
            terminal_primary: true,
            fullscreen: false,
            distraction_free: false,
            safe_mode: false,
            experimental: true,
            allow_plugins: true,
            allow_integrations: true,
        }
    }
}

/// Common interface implemented by every built-in mode.
pub trait Mode: Send + Sync {
    /// Stable mode identifier.
    fn id(&self) -> ModeId;

    /// Human-readable name.
    fn name(&self) -> &'static str;

    /// Description of the mode.
    fn description(&self) -> &'static str;

    /// Presentation and security policy.
    fn policy(&self) -> ModePolicy;

    /// Called when the mode becomes active.
    fn enter(&self) {}

    /// Called immediately before the mode is replaced.
    fn exit(&self) {}
}
