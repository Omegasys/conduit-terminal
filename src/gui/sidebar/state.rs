use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarState {
    Created,
    Initializing,
    Visible,
    Hidden,
    Updating,
    Disabled,
    Closing,
    Closed,
}

#[derive(Debug, Clone)]
pub struct SidebarStateSnapshot {
    state: SidebarState,
    visible: bool,
    width: u32,
    active_panel: String,
    captured_at: SystemTime,
}

impl SidebarStateSnapshot {
    pub fn new(
        state: SidebarState,
        visible: bool,
        width: u32,
        active_panel: impl Into<String>,
    ) -> Self {
        Self {
            state,
            visible,
            width,
            active_panel: active_panel.into(),
            captured_at: SystemTime::now(),
        }
    }

    pub fn state(&self) -> SidebarState {
        self.state
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn active_panel(&self) -> &str {
        &self.active_panel
    }

    pub fn captured_at(&self) -> SystemTime {
        self.captured_at
    }
}
