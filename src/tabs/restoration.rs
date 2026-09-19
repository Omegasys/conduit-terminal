//! Tab session restoration.

use super::{
    icons::TabIcon,
    state::{
        TabActivity,
        TabState,
    },
    tab::{
        Tab,
        TabId,
    },
};

/// Controls tab restoration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabRestorePolicy {
    Disabled,
    RestoreOpenTabs,
    RestoreFullSession,
}

impl Default for TabRestorePolicy {
    fn default() -> Self {
        Self::RestoreOpenTabs
    }
}

/// Serializable tab restoration state.
#[derive(Debug, Clone)]
pub struct TabRestoreData {
    pub id: Option<TabId>,

    pub title: String,
    pub icon: TabIcon,

    pub state: TabState,
    pub activity: TabActivity,

    pub window_id: Option<u64>,
    pub workspace_id: Option<String>,

    pub pane_ids: Vec<String>,
    pub active_pane_id: Option<String>,

    pub pinned: bool,
    pub muted: bool,
    pub dirty: bool,

    pub order: usize,
}

impl TabRestoreData {
    pub fn from_tab(
        tab: &Tab,
        order: usize,
    ) -> Self {
        Self {
            id: Some(tab.id()),

            title:
                tab.title()
                    .current()
                    .to_string(),

            icon: tab.icon().clone(),

            state: tab.state(),
            activity: tab.activity(),

            window_id:
                tab.window_id(),

            workspace_id:
                tab.workspace_id()
                    .map(ToOwned::to_owned),

            pane_ids:
                tab.pane_ids()
                    .to_vec(),

            active_pane_id:
                tab.active_pane_id()
                    .map(ToOwned::to_owned),

            pinned: tab.pinned(),
            muted: tab.muted(),
            dirty: tab.dirty(),

            order,
        }
    }

    pub fn apply_to_tab(
        &self,
        tab: &mut Tab,
    ) {
        tab.set_title(
            self.title.clone(),
        );

        tab.set_icon(
            self.icon.clone(),
        );

        tab.set_state(self.state);
        tab.set_activity(
            self.activity,
        );

        tab.set_window_id(
            self.window_id,
        );

        tab.set_workspace_id(
            self.workspace_id.clone(),
        );

        for pane in &self.pane_ids {
            tab.add_pane(pane.clone());
        }

        if let Some(active) =
            &self.active_pane_id
        {
            tab.set_active_pane(
                active.clone(),
            );
        }

        tab.set_pinned(self.pinned);
        tab.set_muted(self.muted);
        tab.set_dirty(self.dirty);
    }
}
