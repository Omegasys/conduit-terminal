use crate::tabs::TabId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabCommandAction {
    Create,
    Close,
    Focus,
    Next,
    Previous,
    First,
    Last,
    MoveLeft,
    MoveRight,
    Pin,
    Unpin,
    Mute,
    Unmute,
    List,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabCommand {
    action: TabCommandAction,
    tab_id: Option<TabId>,
}

impl TabCommand {
    pub fn new(action: TabCommandAction) -> Self {
        Self {
            action,
            tab_id: None,
        }
    }

    pub fn with_tab_id(
        action: TabCommandAction,
        tab_id: TabId,
    ) -> Self {
        Self {
            action,
            tab_id: Some(tab_id),
        }
    }

    pub fn action(&self) -> TabCommandAction {
        self.action
    }

    pub fn tab_id(&self) -> Option<TabId> {
        self.tab_id
    }

    pub fn set_tab_id(&mut self, id: TabId) {
        self.tab_id = Some(id);
    }

    pub fn is_targeted(&self) -> bool {
        self.tab_id.is_some()
    }
}
