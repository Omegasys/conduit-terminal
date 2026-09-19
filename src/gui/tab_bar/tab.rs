use crate::tabs::{TabActivity, TabId, TabState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabBarTabState {
    Normal,
    Active,
    Hovered,
    Pressed,
    Closing,
    Attention,
    Disabled,
}

impl Default for TabBarTabState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone)]
pub struct TabBarTab {
    id: TabId,
    title: String,
    icon: Option<String>,
    state: TabBarTabState,
    tab_state: TabState,
    activity: TabActivity,
    pinned: bool,
    muted: bool,
    dirty: bool,
    close_button_visible: bool,
    enabled: bool,
    visible: bool,
}

impl TabBarTab {
    pub fn new(id: TabId, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            icon: None,
            state: TabBarTabState::Normal,
            tab_state: TabState::Created,
            activity: TabActivity::None,
            pinned: false,
            muted: false,
            dirty: false,
            close_button_visible: true,
            enabled: true,
            visible: true,
        }
    }

    pub fn id(&self) -> TabId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn state(&self) -> TabBarTabState {
        self.state
    }

    pub fn tab_state(&self) -> TabState {
        self.tab_state
    }

    pub fn activity(&self) -> TabActivity {
        self.activity
    }

    pub fn is_pinned(&self) -> bool {
        self.pinned
    }

    pub fn is_muted(&self) -> bool {
        self.muted
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn close_button_visible(&self) -> bool {
        self.close_button_visible
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_icon(&mut self, icon: impl Into<String>) {
        self.icon = Some(icon.into());
    }

    pub fn clear_icon(&mut self) {
        self.icon = None;
    }

    pub fn set_state(&mut self, state: TabBarTabState) {
        self.state = state;
    }

    pub fn set_tab_state(&mut self, state: TabState) {
        self.tab_state = state;
    }

    pub fn set_activity(&mut self, activity: TabActivity) {
        self.activity = activity;

        if !matches!(activity, TabActivity::None) {
            self.state = TabBarTabState::Attention;
        }
    }

    pub fn clear_activity(&mut self) {
        self.activity = TabActivity::None;

        if self.state == TabBarTabState::Attention {
            self.state = TabBarTabState::Normal;
        }
    }

    pub fn set_pinned(&mut self, pinned: bool) {
        self.pinned = pinned;
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn set_close_button_visible(&mut self, visible: bool) {
        self.close_button_visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !enabled {
            self.state = TabBarTabState::Disabled;
        } else if self.state == TabBarTabState::Disabled {
            self.state = TabBarTabState::Normal;
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn activate(&mut self) {
        self.state = TabBarTabState::Active;
        self.tab_state = TabState::Active;
        self.clear_activity();
    }

    pub fn deactivate(&mut self) {
        self.state = TabBarTabState::Normal;

        if self.tab_state == TabState::Active {
            self.tab_state = TabState::Inactive;
        }
    }

    pub fn hover(&mut self) {
        if self.enabled && self.state != TabBarTabState::Active {
            self.state = TabBarTabState::Hovered;
        }
    }

    pub fn unhover(&mut self) {
        if self.state == TabBarTabState::Hovered {
            self.state = TabBarTabState::Normal;
        }
    }

    pub fn press(&mut self) {
        if self.enabled {
            self.state = TabBarTabState::Pressed;
        }
    }

    pub fn begin_close(&mut self) {
        self.state = TabBarTabState::Closing;
        self.tab_state = TabState::Closing;
    }

    pub fn can_activate(&self) -> bool {
        self.visible && self.enabled
    }
}
