//! Top-level window manager.
//!
//! The manager owns logical windows and provides creation, lookup,
//! focus, closing, placement, and restoration operations.

use std::collections::HashMap;

use super::{
    placement::{
        WindowGeometry,
        WindowPlacement,
    },
    restoration::WindowRestoreData,
    window::{
        Window,
        WindowId,
    },
};

/// Window manager errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowManagerError {
    WindowNotFound(WindowId),
    CannotCloseLastWindow,
}

impl std::fmt::Display for WindowManagerError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::WindowNotFound(id) => {
                write!(formatter, "window not found: {}", id)
            }

            Self::CannotCloseLastWindow => {
                write!(
                    formatter,
                    "cannot close the last active window"
                )
            }
        }
    }
}

impl std::error::Error for WindowManagerError {}

/// Owns all top-level Conduit windows.
pub struct WindowManager {
    windows: HashMap<WindowId, Window>,
    active_window: Option<WindowId>,

    placement: WindowPlacement,

    screen_width: u32,
    screen_height: u32,
}

impl WindowManager {
    pub fn new(
        screen_width: u32,
        screen_height: u32,
    ) -> Self {
        Self {
            windows: HashMap::new(),
            active_window: None,

            placement: WindowPlacement::default(),

            screen_width,
            screen_height,
        }
    }

    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    pub fn active_window(&self) -> Option<&Window> {
        self.active_window
            .and_then(|id| self.windows.get(&id))
    }

    pub fn active_window_id(&self) -> Option<WindowId> {
        self.active_window
    }

    pub fn get(
        &self,
        id: WindowId,
    ) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn get_mut(
        &mut self,
        id: WindowId,
    ) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    pub fn windows(&self) -> impl Iterator<Item = &Window> {
        self.windows.values()
    }

    /// Creates and registers a new window.
    pub fn create_window(
        &mut self,
        title: impl Into<String>,
    ) -> WindowId {
        let geometry = self
            .placement
            .resolve(
                self.screen_width,
                self.screen_height,
            );

        let mut window = Window::new(
            title,
            geometry,
        );

        window.show();

        let id = window.id();

        self.windows.insert(id, window);

        self.focus(id);

        id
    }

    /// Creates a window from saved restoration information.
    pub fn restore_window(
        &mut self,
        restore_data: &WindowRestoreData,
    ) -> WindowId {
        let geometry = restore_data.geometry;

        let mut window = Window::new(
            restore_data.title.clone(),
            geometry,
        );

        restore_data.apply_to_window(&mut window);

        let id = window.id();

        self.windows.insert(id, window);

        if restore_data.focused {
            self.focus(id);
        }

        id
    }

    /// Focuses a window and removes focus from the previous one.
    pub fn focus(
        &mut self,
        id: WindowId,
    ) -> Result<(), WindowManagerError> {
        if !self.windows.contains_key(&id) {
            return Err(
                WindowManagerError::WindowNotFound(id)
            );
        }

        if let Some(previous) = self.active_window {
            if let Some(window) =
                self.windows.get_mut(&previous)
            {
                window.set_focused(false);
            }
        }

        if let Some(window) = self.windows.get_mut(&id) {
            window.set_focused(true);
        }

        self.active_window = Some(id);

        Ok(())
    }

    /// Closes a window.
    pub fn close(
        &mut self,
        id: WindowId,
    ) -> Result<(), WindowManagerError> {
        if !self.windows.contains_key(&id) {
            return Err(
                WindowManagerError::WindowNotFound(id)
            );
        }

        if self.windows.len() <= 1 {
            return Err(
                WindowManagerError::CannotCloseLastWindow
            );
        }

        self.windows.remove(&id);

        if self.active_window == Some(id) {
            self.active_window = self
                .windows
                .keys()
                .next()
                .copied();

            if let Some(active) = self.active_window {
                if let Some(window) =
                    self.windows.get_mut(&active)
                {
                    window.set_focused(true);
                }
            }
        }

        Ok(())
    }

    /// Changes the logical desktop size used for automatic placement.
    pub fn set_screen_size(
        &mut self,
        width: u32,
        height: u32,
    ) {
        self.screen_width = width.max(1);
        self.screen_height = height.max(1);
    }

    /// Sets the default placement strategy.
    pub fn set_placement(
        &mut self,
        placement: WindowPlacement,
    ) {
        self.placement = placement;
    }

    pub fn placement(&self) -> WindowPlacement {
        self.placement
    }

    /// Updates a window's geometry.
    pub fn set_geometry(
        &mut self,
        id: WindowId,
        geometry: WindowGeometry,
    ) -> Result<(), WindowManagerError> {
        let window = self
            .windows
            .get_mut(&id)
            .ok_or(
                WindowManagerError::WindowNotFound(id)
            )?;

        window.set_geometry(geometry);

        Ok(())
    }

    /// Generates restoration data for all currently managed windows.
    pub fn restoration_data(
        &self,
    ) -> Vec<WindowRestoreData> {
        self.windows
            .values()
            .map(WindowRestoreData::from_window)
            .collect()
    }

    /// Closes all windows.
    pub fn close_all(&mut self) {
        for window in self.windows.values_mut() {
            window.close();
        }

        self.windows.clear();
        self.active_window = None;
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}
