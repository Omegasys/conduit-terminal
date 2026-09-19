//! Workspace restoration.

use super::{
    manager::WorkspaceManager,
    state::WorkspaceState,
    workspace::{
        Workspace,
        WorkspaceId,
        WorkspaceStatus,
    },
};

/// Options controlling workspace restoration.
#[derive(Debug, Clone, Copy)]
pub struct WorkspaceRestoreOptions {
    /// Replace the current workspace collection.
    pub replace_existing: bool,

    /// Restore the previously active workspace.
    pub restore_active: bool,
}

impl Default for WorkspaceRestoreOptions {
    fn default() -> Self {
        Self {
            replace_existing: true,
            restore_active: true,
        }
    }
}

/// Restores logical workspace state.
#[derive(Debug, Default)]
pub struct WorkspaceRestorer;

impl WorkspaceRestorer {
    pub fn new() -> Self {
        Self
    }

    pub fn restore(
        &self,
        manager: &mut WorkspaceManager,
        state: &WorkspaceState,
        options: WorkspaceRestoreOptions,
    ) -> Result<(), String> {
        if options.replace_existing {
            manager.close_all();
        }

        for saved in
            &state.workspaces
        {
            let mut workspace =
                Workspace::new(
                    saved.name.clone()
                );

            workspace.set_description(
                saved.description.clone()
            );

            workspace.set_auto_restore(
                saved.auto_restore
            );

            for window_id
                in &saved.window_ids
            {
                workspace.add_window(
                    *window_id
                );
            }

            for tab_id
                in &saved.tab_ids
            {
                workspace.add_tab(
                    *tab_id
                );
            }

            workspace.set_active_window(
                saved.active_window_id
            );

            workspace.set_active_tab(
                saved.active_tab_id
            );

            workspace.set_status(
                if saved.active {
                    WorkspaceStatus::Active
                } else {
                    WorkspaceStatus::Inactive
                }
            );

            workspace.set_dirty(
                saved.dirty
            );

            manager.add(
                workspace
            );
        }

        if options.restore_active {
            if let Some(active_id) =
                state.active_workspace
            {
                if manager.get(active_id)
                    .is_some()
                {
                    manager
                        .activate(
                            active_id
                        )
                        .map_err(
                            |error| {
                                error.to_string()
                            }
                        )?;
                } else if let Some(first) =
                    manager.workspaces()
                        .first()
                        .map(
                            Workspace::id
                        )
                {
                    manager
                        .activate(
                            first
                        )
                        .map_err(
                            |error| {
                                error.to_string()
                            }
                        )?;
                }
            }
        }

        Ok(())
    }

    /// Restores a single workspace into the manager.
    pub fn restore_one(
        &self,
        manager: &mut WorkspaceManager,
        state: &WorkspaceState,
        workspace_id: WorkspaceId,
    ) -> Result<(), String> {
        let Some(saved) =
            state.workspaces
                .iter()
                .find(
                    |workspace| {
                        workspace.id
                            == workspace_id
                    }
                )
        else {
            return Err(
                format!(
                    "workspace not found in state: {}",
                    workspace_id
                )
            );
        };

        let mut workspace =
            Workspace::new(
                saved.name.clone()
            );

        workspace.set_description(
            saved.description.clone()
        );

        workspace.set_auto_restore(
            saved.auto_restore
        );

        for window_id
            in &saved.window_ids
        {
            workspace.add_window(
                *window_id
            );
        }

        for tab_id
            in &saved.tab_ids
        {
            workspace.add_tab(
                *tab_id
            );
        }

        workspace.set_active_window(
            saved.active_window_id
        );

        workspace.set_active_tab(
            saved.active_tab_id
        );

        manager.add(
            workspace
        );

        Ok(())
    }
}
