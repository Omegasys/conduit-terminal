//! Workspace switching.

use super::{
    manager::{
        WorkspaceManager,
        WorkspaceManagerError,
    },
    workspace::WorkspaceId,
};

/// Handles switching between workspaces.
#[derive(Debug, Default)]
pub struct WorkspaceSwitcher;

impl WorkspaceSwitcher {
    pub fn new() -> Self {
        Self
    }

    pub fn switch_to(
        &self,
        manager: &mut WorkspaceManager,
        workspace_id: WorkspaceId,
    ) -> Result<(), WorkspaceManagerError> {
        manager.activate(
            workspace_id
        )
    }

    pub fn next(
        &self,
        manager: &mut WorkspaceManager,
    ) -> Result<WorkspaceId, WorkspaceManagerError> {
        let workspaces =
            manager.workspaces();

        if workspaces.is_empty() {
            return Err(
                WorkspaceManagerError::InvalidWorkspace
            );
        }

        let current =
            manager.active_workspace_id();

        let current_index =
            current.and_then(
                |id| {
                    workspaces
                        .iter()
                        .position(
                            |workspace| {
                                workspace.id()
                                    == id
                            }
                        )
                }
            );

        let next_index =
            match current_index {
                Some(index) =>
                    (index + 1)
                        % workspaces.len(),

                None => 0,
            };

        let id =
            workspaces[next_index]
                .id();

        manager.activate(
            id
        )?;

        Ok(id)
    }

    pub fn previous(
        &self,
        manager: &mut WorkspaceManager,
    ) -> Result<WorkspaceId, WorkspaceManagerError> {
        let workspaces =
            manager.workspaces();

        if workspaces.is_empty() {
            return Err(
                WorkspaceManagerError::InvalidWorkspace
            );
        }

        let current =
            manager.active_workspace_id();

        let current_index =
            current.and_then(
                |id| {
                    workspaces
                        .iter()
                        .position(
                            |workspace| {
                                workspace.id()
                                    == id
                            }
                        )
                }
            );

        let previous_index =
            match current_index {
                Some(0) =>
                    workspaces.len() - 1,

                Some(index) =>
                    index - 1,

                None =>
                    workspaces.len() - 1,
            };

        let id =
            workspaces[previous_index]
                .id();

        manager.activate(
            id
        )?;

        Ok(id)
    }
}
