//! Conduit window model.
//!
//! A Window represents a top-level application window. It does not own
//! tabs, panes, rendering backends, or workspaces directly. Those systems
//! can be attached through identifiers and managed independently.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::lifecycle::{
    Lifecycle,
    LifecyclePhase,
};

use super::placement::WindowGeometry;

/// Globally unique window identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WindowId(u64);

impl WindowId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);

        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl Default for WindowId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WindowId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "window-{}", self.0)
    }
}

/// Type of top-level Conduit window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    Normal,
    Dialog,
    Settings,
    About,
    DetachedTerminal,
}

/// Current window state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Created,
    Initializing,
    Visible,
    Hidden,
    Minimized,
    Maximized,
    Fullscreen,
    Closing,
    Closed,
}

/// A top-level Conduit window.
#[derive(Debug, Clone)]
pub struct Window {
    id: WindowId,
    window_type: WindowType,
    state: WindowState,

    title: String,

    geometry: WindowGeometry,

    focused: bool,
    always_on_top: bool,
    resizable: bool,
    decorated: bool,

    workspace_id: Option<String>,
    active_tab_id: Option<String>,

    lifecycle: Lifecycle,
}

impl Window {
    pub fn new(
        title: impl Into<String>,
        geometry: WindowGeometry,
    ) -> Self {
        Self {
            id: WindowId::new(),
            window_type: WindowType::Normal,
            state: WindowState::Created,

            title: title.into(),
            geometry,

            focused: false,
            always_on_top: false,
            resizable: true,
            decorated: true,

            workspace_id: None,
            active_tab_id: None,

            lifecycle: Lifecycle::new(),
        }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn window_type(&self) -> WindowType {
        self.window_type
    }

    pub fn set_window_type(&mut self, window_type: WindowType) {
        self.window_type = window_type;
    }

    pub fn state(&self) -> WindowState {
        self.state
    }

    pub fn set_state(&mut self, state: WindowState) {
        self.state = state;
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn geometry(&self) -> WindowGeometry {
        self.geometry
    }

    pub fn set_geometry(&mut self, geometry: WindowGeometry) {
        self.geometry = geometry;
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn always_on_top(&self) -> bool {
        self.always_on_top
    }

    pub fn set_always_on_top(&mut self, enabled: bool) {
        self.always_on_top = enabled;
    }

    pub fn resizable(&self) -> bool {
        self.resizable
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
    }

    pub fn decorated(&self) -> bool {
        self.decorated
    }

    pub fn set_decorated(&mut self, decorated: bool) {
        self.decorated = decorated;
    }

    pub fn workspace_id(&self) -> Option<&str> {
        self.workspace_id.as_deref()
    }

    pub fn set_workspace_id(
        &mut self,
        workspace_id: Option<String>,
    ) {
        self.workspace_id = workspace_id;
    }

    pub fn active_tab_id(&self) -> Option<&str> {
        self.active_tab_id.as_deref()
    }

    pub fn set_active_tab_id(
        &mut self,
        tab_id: Option<String>,
    ) {
        self.active_tab_id = tab_id;
    }

    pub fn lifecycle(&self) -> &Lifecycle {
        &self.lifecycle
    }

    pub fn lifecycle_mut(&mut self) -> &mut Lifecycle {
        &mut self.lifecycle
    }

    pub fn is_open(&self) -> bool {
        self.state != WindowState::Closed
    }

    pub fn is_visible(&self) -> bool {
        matches!(
            self.state,
            WindowState::Visible
                | WindowState::Maximized
                | WindowState::Fullscreen
        )
    }

    pub fn is_fullscreen(&self) -> bool {
        self.state == WindowState::Fullscreen
    }

    pub fn show(&mut self) {
        self.state = WindowState::Visible;
    }

    pub fn hide(&mut self) {
        self.state = WindowState::Hidden;
        self.focused = false;
    }

    pub fn minimize(&mut self) {
        self.state = WindowState::Minimized;
        self.focused = false;
    }

    pub fn maximize(&mut self) {
        self.state = WindowState::Maximized;
    }

    pub fn fullscreen(&mut self) {
        self.state = WindowState::Fullscreen;
    }

    pub fn close(&mut self) {
        self.state = WindowState::Closed;
        self.focused = false;
    }

    /// Returns whether the lifecycle system considers this window active.
    pub fn lifecycle_running(&self) -> bool {
        self.lifecycle.phase() == LifecyclePhase::Running
    }
}
