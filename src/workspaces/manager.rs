//! Workspace manager.

use super::{
    state::WorkspaceState,
    workspace::{
        Workspace,
        WorkspaceId,
        WorkspaceStatus,
    },
};

/// Errors produced by workspace management.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceManagerError {
    WorkspaceNotFound(WorkspaceId),
    WorkspaceAlreadyExists(WorkspaceId),
    CannotCloseLastWorkspace,
    InvalidWorkspace,
}

impl std::fmt::Display
    for WorkspaceManagerError
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::WorkspaceNotFound(id) => {
                write!(
                    formatter,
                    "workspace not found: {}",
                    id
                )
            }

            Self::WorkspaceAlreadyExists(id) => {
                write!(
                    formatter,
                    "workspace already exists: {}",
                    id
                )
            }

            Self::CannotCloseLastWorkspace => {
                write!(
                    formatter,
                    "cannot close the last workspace"
                )
            }

            Self::InvalidWorkspace => {
                write!(
                    formatter,
                    "invalid workspace"
                )
            }
        }
    }
}

impl std::error::Error
    for WorkspaceManagerError {}

/// Owns the logical collection of workspaces.
#[derive(Debug, Default)]
pub struct WorkspaceManager {
    workspaces: Vec<Workspace>,
    active_workspace: Option<WorkspaceId>,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self {
            workspaces:
                Vec::new(),

            active_workspace:
                None,
        }
    }

    pub fn len(&self) -> usize {
        self.workspaces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.workspaces.is_empty()
    }

    pub fn workspaces(
        &self,
    ) -> &[Workspace] {
        &self.workspaces
    }

    pub fn active_workspace_id(
        &self,
    ) -> Option<WorkspaceId> {
        self.active_workspace
    }

    pub fn active_workspace(
        &self,
    ) -> Option<&Workspace> {
        self.active_workspace
            .and_then(
                |id| self.get(id)
            )
    }

    pub fn active_workspace_mut(
        &mut self,
    ) -> Option<&mut Workspace> {
        let id =
            self.active_workspace?;

        self.get_mut(id)
    }

    pub fn get(
        &self,
        id: WorkspaceId,
    ) -> Option<&Workspace> {
        self.workspaces
            .iter()
            .find(
                |workspace| {
                    workspace.id() == id
                }
            )
    }

    pub fn get_mut(
        &mut self,
        id: WorkspaceId,
    ) -> Option<&mut Workspace> {
        self.workspaces
            .iter_mut()
            .find(
                |workspace| {
                    workspace.id() == id
                }
            )
    }

    pub fn add(
        &mut self,
        workspace: Workspace,
    ) -> WorkspaceId {
        let id =
            workspace.id();

        if self.workspaces
            .is_empty()
        {
            self.active_workspace =
                Some(id);
        }

        self.workspaces
            .push(workspace);

        id
    }

    pub fn activate(
        &mut self,
        id: WorkspaceId,
    ) -> Result<(), WorkspaceManagerError> {
        if self.get(id).is_none() {
            return Err(
                WorkspaceManagerError::WorkspaceNotFound(
                    id
                )
            );
        }

        for workspace
            in &mut self.workspaces
        {
            if workspace.id() == id {
                workspace.set_status(
                    WorkspaceStatus::Active
                );
            } else if workspace.status()
                != WorkspaceStatus::Closed
            {
                workspace.set_status(
                    WorkspaceStatus::Inactive
                );
            }
        }

        self.active_workspace =
            Some(id);

        Ok(())
    }

    pub fn remove(
        &mut self,
        id: WorkspaceId,
    ) -> Result<(), WorkspaceManagerError> {
        if self.workspaces.len() <= 1 {
            return Err(
                WorkspaceManagerError::CannotCloseLastWorkspace
            );
        }

        let index =
            self.workspaces
                .iter()
                .position(
                    |workspace| {
                        workspace.id() == id
                    }
                )
                .ok_or(
                    WorkspaceManagerError::WorkspaceNotFound(
                        id
                    )
                )?;

        let was_active =
            self.active_workspace
                == Some(id);

        self.workspaces
            .remove(index);

        if was_active {
            let next_index =
                index.min(
                    self.workspaces.len()
                        .saturating_sub(1)
                );

            let next_id =
                self.workspaces
                    .get(next_index)
                    .map(
                        Workspace::id
                    );

            if let Some(next_id) =
                next_id
            {
                self.activate(
                    next_id
                )?;
            }
        }

        Ok(())
    }

    pub fn restoration_state(
        &self,
    ) -> WorkspaceState {
        WorkspaceState::from_manager(
            self
        )
    }

    pub fn close_all(
        &mut self,
    ) {
        self.workspaces.clear();

        self.active_workspace =
            None;
    }
}
