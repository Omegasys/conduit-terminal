//! Logical tab representation.

use std::fmt;
use std::sync::atomic::{
    AtomicU64,
    Ordering,
};

use super::{
    icons::TabIcon,
    state::{
        TabActivity,
        TabState,
    },
    title::TabTitle,
};

/// Globally unique tab identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TabId(u64);

impl TabId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 =
            AtomicU64::new(1);

        Self(
            NEXT_ID.fetch_add(
                1,
                Ordering::Relaxed,
            )
        )
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl Default for TabId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TabId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "tab-{}",
            self.0
        )
    }
}

/// A logical Conduit tab.
#[derive(Debug, Clone)]
pub struct Tab {
    id: TabId,

    title: TabTitle,
    icon: TabIcon,

    state: TabState,
    activity: TabActivity,

    window_id: Option<u64>,
    workspace_id: Option<String>,

    /// IDs of panes belonging to this tab.
    pane_ids: Vec<String>,

    /// ID of the currently active pane.
    active_pane_id: Option<String>,

    pinned: bool,
    muted: bool,
    dirty: bool,

    created_at: u64,
}

impl Tab {
    /// Creates a new tab.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: TabId::new(),

            title: TabTitle::new(title),
            icon: TabIcon::default(),

            state: TabState::Created,
            activity: TabActivity::None,

            window_id: None,
            workspace_id: None,

            pane_ids: Vec::new(),
            active_pane_id: None,

            pinned: false,
            muted: false,
            dirty: false,

            created_at: current_timestamp(),
        }
    }

    pub fn id(&self) -> TabId {
        self.id
    }

    pub fn title(&self) -> &TabTitle {
        &self.title
    }

    pub fn title_mut(&mut self) -> &mut TabTitle {
        &mut self.title
    }

    pub fn set_title(
        &mut self,
        title: impl Into<String>,
    ) {
        self.title.set_custom(title);
    }

    pub fn icon(&self) -> &TabIcon {
        &self.icon
    }

    pub fn icon_mut(&mut self) -> &mut TabIcon {
        &mut self.icon
    }

    pub fn set_icon(
        &mut self,
        icon: TabIcon,
    ) {
        self.icon = icon;
    }

    pub fn state(&self) -> TabState {
        self.state
    }

    pub fn set_state(
        &mut self,
        state: TabState,
    ) {
        self.state = state;
    }

    pub fn activity(&self) -> TabActivity {
        self.activity
    }

    pub fn set_activity(
        &mut self,
        activity: TabActivity,
    ) {
        self.activity = activity;
    }

    pub fn clear_activity(&mut self) {
        self.activity = TabActivity::None;
    }

    pub fn window_id(&self) -> Option<u64> {
        self.window_id
    }

    pub fn set_window_id(
        &mut self,
        window_id: Option<u64>,
    ) {
        self.window_id = window_id;
    }

    pub fn workspace_id(&self) -> Option<&str> {
        self.workspace_id
            .as_deref()
    }

    pub fn set_workspace_id(
        &mut self,
        workspace_id: Option<String>,
    ) {
        self.workspace_id = workspace_id;
    }

    pub fn pane_ids(&self) -> &[String] {
        &self.pane_ids
    }

    pub fn active_pane_id(&self) -> Option<&str> {
        self.active_pane_id
            .as_deref()
    }

    pub fn add_pane(
        &mut self,
        pane_id: impl Into<String>,
    ) {
        let pane_id = pane_id.into();

        if self.pane_ids.contains(&pane_id) {
            return;
        }

        if self.active_pane_id.is_none() {
            self.active_pane_id =
                Some(pane_id.clone());
        }

        self.pane_ids.push(pane_id);
    }

    pub fn remove_pane(
        &mut self,
        pane_id: &str,
    ) -> bool {
        let Some(index) = self
            .pane_ids
            .iter()
            .position(|id| id == pane_id)
        else {
            return false;
        };

        self.pane_ids.remove(index);

        if self.active_pane_id.as_deref()
            == Some(pane_id)
        {
            self.active_pane_id =
                self.pane_ids
                    .first()
                    .cloned();
        }

        true
    }

    pub fn set_active_pane(
        &mut self,
        pane_id: impl Into<String>,
    ) -> bool {
        let pane_id = pane_id.into();

        if !self.pane_ids.contains(&pane_id) {
            return false;
        }

        self.active_pane_id = Some(pane_id);
        true
    }

    pub fn pinned(&self) -> bool {
        self.pinned
    }

    pub fn set_pinned(
        &mut self,
        pinned: bool,
    ) {
        self.pinned = pinned;
    }

    pub fn muted(&self) -> bool {
        self.muted
    }

    pub fn set_muted(
        &mut self,
        muted: bool,
    ) {
        self.muted = muted;
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(
        &mut self,
        dirty: bool,
    ) {
        self.dirty = dirty;
    }

    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    pub fn mark_activity(
        &mut self,
        activity: TabActivity,
    ) {
        if !self.muted {
            self.activity = activity;
        }
    }

    pub fn close(&mut self) {
        self.state = TabState::Closed;
        self.activity = TabActivity::None;
    }

    pub fn is_open(&self) -> bool {
        self.state != TabState::Closed
    }
}

fn current_timestamp() -> u64 {
    use std::time::{
        SystemTime,
        UNIX_EPOCH,
    };

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
