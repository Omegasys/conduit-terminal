//! Window restoration and persistence.
//!
//! This module stores logical window information rather than native
//! toolkit handles. That makes restoration portable across platforms.

use super::{
    placement::WindowGeometry,
    window::{
        Window,
        WindowId,
        WindowState,
        WindowType,
    },
};

/// Controls how window restoration behaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowRestorePolicy {
    Disabled,
    RestoreLastSession,
    RestoreIfAvailable,
    RestoreAlways,
}

impl Default for WindowRestorePolicy {
    fn default() -> Self {
        Self::RestoreIfAvailable
    }
}

/// Serializable logical window state.
#[derive(Debug, Clone)]
pub struct WindowRestoreData {
    pub id: Option<WindowId>,

    pub title: String,
    pub window_type: WindowType,
    pub state: WindowState,

    pub geometry: WindowGeometry,

    pub workspace_id: Option<String>,
    pub active_tab_id: Option<String>,

    pub focused: bool,
    pub always_on_top: bool,
    pub resizable: bool,
    pub decorated: bool,
}

impl WindowRestoreData {
    /// Creates restoration data from a live window.
    pub fn from_window(window: &Window) -> Self {
        Self {
            id: Some(window.id()),

            title: window.title().to_string(),
            window_type: window.window_type(),
            state: window.state(),

            geometry: window.geometry(),

            workspace_id: window
                .workspace_id()
                .map(ToOwned::to_owned),

            active_tab_id: window
                .active_tab_id()
                .map(ToOwned::to_owned),

            focused: window.focused(),
            always_on_top: window.always_on_top(),
            resizable: window.resizable(),
            decorated: window.decorated(),
        }
    }

    /// Applies stored state to an existing window.
    pub fn apply_to_window(
        &self,
        window: &mut Window,
    ) {
        window.set_window_type(self.window_type);
        window.set_state(self.state);

        window.set_title(self.title.clone());
        window.set_geometry(self.geometry);

        window.set_workspace_id(
            self.workspace_id.clone(),
        );

        window.set_active_tab_id(
            self.active_tab_id.clone(),
        );

        window.set_focused(self.focused);
        window.set_always_on_top(self.always_on_top);
        window.set_resizable(self.resizable);
        window.set_decorated(self.decorated);
    }
}
