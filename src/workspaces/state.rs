//! Serializable workspace state.

use super::{
    manager::WorkspaceManager,
    workspace::{
        WorkspaceId,
        WorkspaceStatus,
    },
};

/// Serializable state for one workspace.
#[derive(Debug, Clone)]
pub struct SavedWorkspace {
    pub id: WorkspaceId,

    pub name: String,
    pub description: Option<String>,

    pub status: WorkspaceStatus,

    pub window_ids: Vec<u64>,
    pub tab_ids: Vec<u64>,

    pub active_window_id: Option<u64>,
    pub active_tab_id: Option<u64>,

    pub auto_restore: bool,
    pub dirty: bool,

    pub created_at: u64,
    pub updated_at: u64,

    pub active: bool,
}

/// Complete workspace manager state.
#[derive(Debug, Clone, Default)]
pub struct WorkspaceState {
    pub workspaces:
        Vec<SavedWorkspace>,

    pub active_workspace:
        Option<WorkspaceId>,
}

impl WorkspaceState {
    pub fn new() -> Self {
        Self {
            workspaces:
                Vec::new(),

            active_workspace:
                None,
        }
    }

    pub fn from_manager(
        manager: &WorkspaceManager,
    ) -> Self {
        let active =
            manager.active_workspace_id();

        let workspaces =
            manager
                .workspaces()
                .iter()
                .map(
                    |workspace| {
                        SavedWorkspace {
                            id:
                                workspace.id(),

                            name:
                                workspace
                                    .name()
                                    .to_string(),

                            description:
                                workspace
                                    .description()
                                    .map(
                                        ToOwned::to_owned
                                    ),

                            status:
                                workspace
                                    .status(),

                            window_ids:
                                workspace
                                    .window_ids()
                                    .to_vec(),

                            tab_ids:
                                workspace
                                    .tab_ids()
                                    .to_vec(),

                            active_window_id:
                                workspace
                                    .active_window_id(),

                            active_tab_id:
                                workspace
                                    .active_tab_id(),

                            auto_restore:
                                workspace
                                    .auto_restore(),

                            dirty:
                                workspace
                                    .dirty(),

                            created_at:
                                workspace
                                    .created_at(),

                            updated_at:
                                workspace
                                    .updated_at(),

                            active:
                                Some(
                                    workspace.id()
                                )
                                == active,
                        }
                    }
                )
                .collect();

        Self {
            workspaces,
            active_workspace:
                active,
        }
    }

    pub fn len(&self) -> usize {
        self.workspaces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.workspaces.is_empty()
    }

    pub fn find(
        &self,
        id: WorkspaceId,
    ) -> Option<&SavedWorkspace> {
        self.workspaces
            .iter()
            .find(
                |workspace| {
                    workspace.id == id
                }
            )
    }

    pub fn active(
        &self,
    ) -> Option<&SavedWorkspace> {
        self.active_workspace
            .and_then(
                |id| self.find(id)
            )
    }
}
