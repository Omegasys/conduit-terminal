//! Workspace layout coordination.
//!
//! Workspace layouts describe how windows and tabs are arranged at the
//! workspace level. Detailed pane splitting remains owned by PaneLayout.

/// High-level workspace layout mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceLayoutMode {
    /// A normal desktop-style arrangement.
    Freeform,

    /// Windows are automatically tiled.
    Tiled,

    /// Windows are arranged horizontally.
    Horizontal,

    /// Windows are arranged vertically.
    Vertical,

    /// One primary window with secondary windows around it.
    MasterStack,
}

impl Default for WorkspaceLayoutMode {
    fn default() -> Self {
        Self::Freeform
    }
}

/// Logical workspace layout configuration.
#[derive(Debug, Clone)]
pub struct WorkspaceLayout {
    mode: WorkspaceLayoutMode,

    /// IDs of windows in visual ordering.
    window_order: Vec<u64>,

    /// Whether the active window receives layout priority.
    focus_priority: bool,

    /// Gap between windows, expressed in logical pixels.
    gap: u32,

    /// Whether layout changes should happen automatically.
    automatic: bool,
}

impl WorkspaceLayout {
    pub fn new() -> Self {
        Self {
            mode:
                WorkspaceLayoutMode::Freeform,

            window_order:
                Vec::new(),

            focus_priority: true,

            gap: 8,

            automatic: true,
        }
    }

    pub fn mode(
        &self,
    ) -> WorkspaceLayoutMode {
        self.mode
    }

    pub fn set_mode(
        &mut self,
        mode: WorkspaceLayoutMode,
    ) {
        self.mode = mode;
    }

    pub fn window_order(
        &self,
    ) -> &[u64] {
        &self.window_order
    }

    pub fn set_window_order(
        &mut self,
        order: Vec<u64>,
    ) {
        self.window_order =
            order;
    }

    pub fn add_window(
        &mut self,
        window_id: u64,
    ) {
        if !self.window_order
            .contains(&window_id)
        {
            self.window_order
                .push(window_id);
        }
    }

    pub fn remove_window(
        &mut self,
        window_id: u64,
    ) -> bool {
        let Some(index) =
            self.window_order
                .iter()
                .position(
                    |id| *id == window_id
                )
        else {
            return false;
        };

        self.window_order
            .remove(index);

        true
    }

    pub fn focus_priority(
        &self,
    ) -> bool {
        self.focus_priority
    }

    pub fn set_focus_priority(
        &mut self,
        enabled: bool,
    ) {
        self.focus_priority =
            enabled;
    }

    pub fn gap(
        &self,
    ) -> u32 {
        self.gap
    }

    pub fn set_gap(
        &mut self,
        gap: u32,
    ) {
        self.gap = gap;
    }

    pub fn automatic(
        &self,
    ) -> bool {
        self.automatic
    }

    pub fn set_automatic(
        &mut self,
        enabled: bool,
    ) {
        self.automatic =
            enabled;
    }

    pub fn is_tiled(&self) -> bool {
        !matches!(
            self.mode,
            WorkspaceLayoutMode::Freeform
        )
    }

    pub fn clear(&mut self) {
        self.window_order.clear();
    }
}

impl Default for WorkspaceLayout {
    fn default() -> Self {
        Self::new()
    }
}
