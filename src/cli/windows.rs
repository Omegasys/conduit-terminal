use crate::windows::WindowId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowCommandAction {
    Create,
    Close,
    Focus,
    Minimize,
    Maximize,
    Restore,
    Fullscreen,
    List,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowCommand {
    action: WindowCommandAction,
    window_id: Option<WindowId>,
}

impl WindowCommand {
    pub fn new(action: WindowCommandAction) -> Self {
        Self {
            action,
            window_id: None,
        }
    }

    pub fn with_window_id(
        action: WindowCommandAction,
        window_id: WindowId,
    ) -> Self {
        Self {
            action,
            window_id: Some(window_id),
        }
    }

    pub fn action(&self) -> WindowCommandAction {
        self.action
    }

    pub fn window_id(&self) -> Option<WindowId> {
        self.window_id
    }

    pub fn set_window_id(&mut self, id: WindowId) {
        self.window_id = Some(id);
    }

    pub fn is_targeted(&self) -> bool {
        self.window_id.is_some()
    }
}
