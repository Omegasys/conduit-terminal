use std::collections::HashMap;

use crate::windows::{
    WindowGeometry,
    WindowId,
    WindowType,
};

/// GUI-specific identifier.
///
/// The underlying window subsystem owns the actual logical WindowId.
/// This type identifies the GUI representation of that window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GuiWindowId(u64);

impl GuiWindowId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

/// GUI window lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiWindowState {
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

/// GUI representation of a Conduit window.
#[derive(Debug, Clone)]
pub struct GuiWindow {
    id: GuiWindowId,
    backend_window: Option<WindowId>,
    title: String,
    geometry: WindowGeometry,
    window_type: WindowType,
    state: GuiWindowState,
    focused: bool,
    always_on_top: bool,
    resizable: bool,
    decorated: bool,
}

impl GuiWindow {
    pub fn new(
        id: GuiWindowId,
        title: impl Into<String>,
        window_type: WindowType,
    ) -> Self {
        Self {
            id,
            backend_window: None,
            title: title.into(),
            geometry: WindowGeometry::default(),
            window_type,
            state: GuiWindowState::Created,
            focused: false,
            always_on_top: false,
            resizable: true,
            decorated: true,
        }
    }

    pub fn id(&self) -> GuiWindowId {
        self.id
    }

    pub fn backend_window(&self) -> Option<WindowId> {
        self.backend_window
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn geometry(&self) -> WindowGeometry {
        self.geometry
    }

    pub fn window_type(&self) -> WindowType {
        self.window_type
    }

    pub fn state(&self) -> GuiWindowState {
        self.state
    }

    pub fn is_visible(&self) -> bool {
        matches!(
            self.state,
            GuiWindowState::Visible
                | GuiWindowState::Maximized
                | GuiWindowState::Fullscreen
        )
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn set_backend_window(&mut self, window: WindowId) {
        self.backend_window = Some(window);
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_geometry(&mut self, geometry: WindowGeometry) {
        self.geometry = geometry;
    }

    pub fn set_state(&mut self, state: GuiWindowState) {
        self.state = state;
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn set_always_on_top(&mut self, enabled: bool) {
        self.always_on_top = enabled;
    }

    pub fn set_resizable(&mut self, enabled: bool) {
        self.resizable = enabled;
    }

    pub fn set_decorated(&mut self, enabled: bool) {
        self.decorated = enabled;
    }

    pub fn always_on_top(&self) -> bool {
        self.always_on_top
    }

    pub fn resizable(&self) -> bool {
        self.resizable
    }

    pub fn decorated(&self) -> bool {
        self.decorated
    }
}

/// Manages GUI window representations.
#[derive(Debug, Default)]
pub struct GuiWindowManager {
    windows: HashMap<GuiWindowId, GuiWindow>,
    active: Option<GuiWindowId>,
    next_id: u64,
}

impl GuiWindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            active: None,
            next_id: 1,
        }
    }

    pub fn create(
        &mut self,
        title: impl Into<String>,
        window_type: WindowType,
    ) -> GuiWindowId {
        let id = GuiWindowId::new(self.next_id);
        self.next_id = self.next_id.saturating_add(1);

        let window = GuiWindow::new(id, title, window_type);

        self.windows.insert(id, window);

        if self.active.is_none() {
            self.active = Some(id);
        }

        id
    }

    pub fn add(&mut self, window: GuiWindow) -> bool {
        let id = window.id();

        if self.windows.contains_key(&id) {
            return false;
        }

        self.windows.insert(id, window);

        if self.active.is_none() {
            self.active = Some(id);
        }

        true
    }

    pub fn get(&self, id: GuiWindowId) -> Option<&GuiWindow> {
        self.windows.get(&id)
    }

    pub fn get_mut(&mut self, id: GuiWindowId) -> Option<&mut GuiWindow> {
        self.windows.get_mut(&id)
    }

    pub fn activate(&mut self, id: GuiWindowId) -> bool {
        if !self.windows.contains_key(&id) {
            return false;
        }

        if let Some(previous) = self.active {
            if let Some(window) = self.windows.get_mut(&previous) {
                window.set_focused(false);
            }
        }

        if let Some(window) = self.windows.get_mut(&id) {
            window.set_focused(true);
            window.set_state(GuiWindowState::Visible);
        }

        self.active = Some(id);
        true
    }

    pub fn close(&mut self, id: GuiWindowId) -> bool {
        if let Some(window) = self.windows.get_mut(&id) {
            window.set_state(GuiWindowState::Closing);
        } else {
            return false;
        }

        self.windows.remove(&id);

        if self.active == Some(id) {
            self.active = self.windows.keys().next().copied();

            if let Some(active) = self.active {
                if let Some(window) = self.windows.get_mut(&active) {
                    window.set_focused(true);
                }
            }
        }

        true
    }

    pub fn close_all(&mut self) {
        for window in self.windows.values_mut() {
            window.set_state(GuiWindowState::Closing);
        }

        self.windows.clear();
        self.active = None;
    }

    pub fn active(&self) -> Option<GuiWindowId> {
        self.active
    }

    pub fn iter(&self) -> impl Iterator<Item = &GuiWindow> {
        self.windows.values()
    }

    pub fn len(&self) -> usize {
        self.windows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }
}
