//! Workspace representation.

use std::fmt;
use std::sync::atomic::{
    AtomicU64,
    Ordering,
};

/// Globally unique workspace identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkspaceId(u64);

impl WorkspaceId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 =
            AtomicU64::new(1);

        Self(
            NEXT_ID.fetch_add(
                1,
                Ordering::Relaxed,
            ),
        )
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl Default for WorkspaceId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WorkspaceId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "workspace-{}",
            self.0
        )
    }
}

/// Current workspace lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceStatus {
    Created,
    Loading,
    Active,
    Inactive,
    Saving,
    Restoring,
    Closing,
    Closed,
}

/// Logical Conduit workspace.
#[derive(Debug, Clone)]
pub struct Workspace {
    id: WorkspaceId,

    name: String,
    description: Option<String>,

    status: WorkspaceStatus,

    /// IDs of windows belonging to this workspace.
    window_ids: Vec<u64>,

    /// IDs of tabs associated with this workspace.
    tab_ids: Vec<u64>,

    active_window_id: Option<u64>,
    active_tab_id: Option<u64>,

    /// Whether this workspace should be automatically restored.
    auto_restore: bool,

    /// Whether the workspace has unsaved logical state.
    dirty: bool,

    created_at: u64,
    updated_at: u64,
}

impl Workspace {
    pub fn new(
        name: impl Into<String>,
    ) -> Self {
        let timestamp =
            current_timestamp();

        Self {
            id: WorkspaceId::new(),

            name: name.into(),
            description: None,

            status:
                WorkspaceStatus::Created,

            window_ids:
                Vec::new(),

            tab_ids:
                Vec::new(),

            active_window_id: None,
            active_tab_id: None,

            auto_restore: true,

            dirty: false,

            created_at: timestamp,
            updated_at: timestamp,
        }
    }

    pub fn id(&self) -> WorkspaceId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) {
        self.name = name.into();
        self.touch();
    }

    pub fn description(
        &self,
    ) -> Option<&str> {
        self.description
            .as_deref()
    }

    pub fn set_description(
        &mut self,
        description: Option<String>,
    ) {
        self.description =
            description;

        self.touch();
    }

    pub fn status(
        &self,
    ) -> WorkspaceStatus {
        self.status
    }

    pub fn set_status(
        &mut self,
        status: WorkspaceStatus,
    ) {
        self.status = status;
        self.touch();
    }

    pub fn window_ids(
        &self,
    ) -> &[u64] {
        &self.window_ids
    }

    pub fn tab_ids(
        &self,
    ) -> &[u64] {
        &self.tab_ids
    }

    pub fn active_window_id(
        &self,
    ) -> Option<u64> {
        self.active_window_id
    }

    pub fn active_tab_id(
        &self,
    ) -> Option<u64> {
        self.active_tab_id
    }

    pub fn set_active_window(
        &mut self,
        window_id: Option<u64>,
    ) {
        self.active_window_id =
            window_id;

        self.touch();
    }

    pub fn set_active_tab(
        &mut self,
        tab_id: Option<u64>,
    ) {
        self.active_tab_id =
            tab_id;

        self.touch();
    }

    pub fn add_window(
        &mut self,
        window_id: u64,
    ) {
        if !self.window_ids
            .contains(&window_id)
        {
            self.window_ids
                .push(window_id);

            self.touch();
        }
    }

    pub fn remove_window(
        &mut self,
        window_id: u64,
    ) -> bool {
        let Some(index) =
            self.window_ids
                .iter()
                .position(
                    |id| *id == window_id
                )
        else {
            return false;
        };

        self.window_ids
            .remove(index);

        if self.active_window_id
            == Some(window_id)
        {
            self.active_window_id =
                self.window_ids
                    .first()
                    .copied();
        }

        self.touch();

        true
    }

    pub fn add_tab(
        &mut self,
        tab_id: u64,
    ) {
        if !self.tab_ids
            .contains(&tab_id)
        {
            self.tab_ids
                .push(tab_id);

            self.touch();
        }
    }

    pub fn remove_tab(
        &mut self,
        tab_id: u64,
    ) -> bool {
        let Some(index) =
            self.tab_ids
                .iter()
                .position(
                    |id| *id == tab_id
                )
        else {
            return false;
        };

        self.tab_ids
            .remove(index);

        if self.active_tab_id
            == Some(tab_id)
        {
            self.active_tab_id =
                self.tab_ids
                    .first()
                    .copied();
        }

        self.touch();

        true
    }

    pub fn contains_window(
        &self,
        window_id: u64,
    ) -> bool {
        self.window_ids
            .contains(&window_id)
    }

    pub fn contains_tab(
        &self,
        tab_id: u64,
    ) -> bool {
        self.tab_ids
            .contains(&tab_id)
    }

    pub fn auto_restore(
        &self,
    ) -> bool {
        self.auto_restore
    }

    pub fn set_auto_restore(
        &mut self,
        enabled: bool,
    ) {
        self.auto_restore =
            enabled;

        self.touch();
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(
        &mut self,
        dirty: bool,
    ) {
        self.dirty = dirty;

        if dirty {
            self.touch();
        }
    }

    pub fn mark_clean(
        &mut self,
    ) {
        self.dirty = false;
    }

    pub fn created_at(
        &self,
    ) -> u64 {
        self.created_at
    }

    pub fn updated_at(
        &self,
    ) -> u64 {
        self.updated_at
    }

    pub fn touch(&mut self) {
        self.updated_at =
            current_timestamp();

        self.dirty = true;
    }

    pub fn is_open(&self) -> bool {
        self.status
            != WorkspaceStatus::Closed
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
