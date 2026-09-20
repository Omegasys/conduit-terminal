use std::collections::BTreeMap;

use crate::windows::{Window, WindowId, WindowState, WindowType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowDisplayState {
    Normal,
    Focused,
    Minimized,
    Maximized,
    Fullscreen,
    Hidden,
}

impl WindowDisplayState {
    pub fn from_window(window: &Window) -> Self {
        match window.state() {
            WindowState::Maximized => Self::Maximized,
            WindowState::Minimized => Self::Minimized,
            WindowState::Fullscreen => Self::Fullscreen,
            WindowState::Hidden | WindowState::Closed => Self::Hidden,
            WindowState::Visible => {
                if window.focused() {
                    Self::Focused
                } else {
                    Self::Normal
                }
            }
            _ => Self::Normal,
        }
    }
}

pub struct TuiWindow {
    window: Window,
    display_state: WindowDisplayState,
    visible: bool,
    selected: bool,
}

impl TuiWindow {
    pub fn new(window: Window) -> Self {
        let display_state = WindowDisplayState::from_window(&window);

        Self {
            window,
            display_state,
            visible: true,
            selected: false,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }

    pub fn title(&self) -> &str {
        self.window.title()
    }

    pub fn window_type(&self) -> WindowType {
        self.window.window_type()
    }

    pub fn display_state(&self) -> WindowDisplayState {
        self.display_state
    }

    pub fn set_display_state(&mut self, state: WindowDisplayState) {
        self.display_state = state;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn refresh_state(&mut self) {
        self.display_state = WindowDisplayState::from_window(&self.window);
    }
}

pub struct TuiWindowManager {
    windows: BTreeMap<WindowId, TuiWindow>,
    active: Option<WindowId>,
}

impl Default for TuiWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiWindowManager {
    pub fn new() -> Self {
        Self {
            windows: BTreeMap::new(),
            active: None,
        }
    }

    pub fn add(&mut self, window: Window) -> WindowId {
        let id = window.id();
        let mut tui_window = TuiWindow::new(window);

        if self.active.is_none() {
            tui_window.set_selected(true);
            self.active = Some(id);
        }

        self.windows.insert(id, tui_window);
        id
    }

    pub fn remove(&mut self, id: WindowId) -> Option<TuiWindow> {
        let removed = self.windows.remove(&id);

        if self.active == Some(id) {
            self.active = self.windows.keys().next().copied();

            if let Some(active) = self.active {
                if let Some(window) = self.windows.get_mut(&active) {
                    window.set_selected(true);
                }
            }
        }

        removed
    }

    pub fn get(&self, id: WindowId) -> Option<&TuiWindow> {
        self.windows.get(&id)
    }

    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut TuiWindow> {
        self.windows.get_mut(&id)
    }

    pub fn activate(&mut self, id: WindowId) -> bool {
        if !self.windows.contains_key(&id) {
            return false;
        }

        for window in self.windows.values_mut() {
            window.set_selected(false);
        }

        if let Some(window) = self.windows.get_mut(&id) {
            window.set_selected(true);
        }

        self.active = Some(id);
        true
    }

    pub fn active(&self) -> Option<&TuiWindow> {
        self.active.and_then(|id| self.windows.get(&id))
    }

    pub fn active_id(&self) -> Option<WindowId> {
        self.active
    }

    pub fn windows(&self) -> impl Iterator<Item = &TuiWindow> {
        self.windows.values()
    }

    pub fn windows_mut(&mut self) -> impl Iterator<Item = &mut TuiWindow> {
        self.windows.values_mut()
    }

    pub fn visible(&self) -> Vec<&TuiWindow> {
        self.windows.values().filter(|window| window.visible()).collect()
    }

    pub fn len(&self) -> usize {
        self.windows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    pub fn refresh(&mut self) {
        for window in self.windows.values_mut() {
            window.refresh_state();
        }
    }

    pub fn clear(&mut self) {
        self.windows.clear();
        self.active = None;
    }
}
