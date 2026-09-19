//! Workspace saving.
//!
//! This module prepares a serializable workspace snapshot. Actual disk
//! serialization can be provided by Conduit's configuration/resource layer.

use super::{
    manager::WorkspaceManager,
    state::WorkspaceState,
    workspace::{
        WorkspaceId,
        WorkspaceStatus,
    },
};

/// Options controlling workspace saving.
#[derive(Debug, Clone, Copy)]
pub struct WorkspaceSaveOptions {
    pub include_closed: bool,
    pub mark_clean: bool,
}

impl Default for WorkspaceSaveOptions {
    fn default() -> Self {
        Self {
            include_closed: false,
            mark_clean: true,
        }
    }
}

/// Produces workspace snapshots.
#[derive(Debug, Default)]
pub struct WorkspaceSaver;

impl WorkspaceSaver {
    pub fn new() -> Self {
        Self
    }

    pub fn save(
        &self,
        manager: &mut WorkspaceManager,
        options: WorkspaceSaveOptions,
    ) -> WorkspaceState {
        let mut state =
            WorkspaceState::from_manager(
                manager
            );

        if !options.include_closed {
            state.workspaces.retain(
                |workspace| {
                    workspace.status
                        != WorkspaceStatus::Closed
                }
            );
        }

        if options.mark_clean {
            for workspace
                in &mut state.workspaces
            {
                workspace.dirty = false;
            }

            if let Some(active) =
                manager.active_workspace_mut()
            {
                active.mark_clean();
            }
        }

        state
    }

    pub fn save_workspace(
        &self,
        manager: &mut WorkspaceManager,
        workspace_id: WorkspaceId,
        options: WorkspaceSaveOptions,
    ) -> Option<WorkspaceState> {
        let state =
            self.save(
                manager,
                options,
            );

        let exists =
            state.workspaces
                .iter()
                .any(
                    |workspace| {
                        workspace.id
                            == workspace_id
                    }
                );

        exists.then_some(state)
    }
}
