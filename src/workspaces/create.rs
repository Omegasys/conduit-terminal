//! Workspace creation.

use super::{
    manager::WorkspaceManager,
    workspace::{
        Workspace,
        WorkspaceId,
    },
};

/// Options used when creating a workspace.
#[derive(Debug, Clone)]
pub struct WorkspaceCreateOptions {
    pub name: String,
    pub description: Option<String>,
    pub auto_restore: bool,
}

impl WorkspaceCreateOptions {
    pub fn new(
        name: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: None,
            auto_restore: true,
        }
    }

    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description =
            Some(
                description.into()
            );

        self
    }

    pub fn auto_restore(
        mut self,
        enabled: bool,
    ) -> Self {
        self.auto_restore =
            enabled;

        self
    }
}

/// Creates workspaces using a shared manager.
#[derive(Debug, Default)]
pub struct WorkspaceCreator;

impl WorkspaceCreator {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        manager: &mut WorkspaceManager,
        options: WorkspaceCreateOptions,
    ) -> WorkspaceId {
        let mut workspace =
            Workspace::new(
                options.name
            );

        workspace.set_description(
            options.description
        );

        workspace.set_auto_restore(
            options.auto_restore
        );

        /*
         * Creating a workspace should not require an immediate window
         * or terminal. Those resources can be attached afterward.
         */
        let id =
            manager.add(
                workspace
            );

        /*
         * The newly created workspace becomes active when it is the first
         * workspace. For subsequent workspaces, explicitly activating it
         * provides predictable "create workspace" behavior.
         */
        let _ =
            manager.activate(id);

        id
    }

    pub fn create_default(
        &self,
        manager: &mut WorkspaceManager,
    ) -> WorkspaceId {
        self.create(
            manager,
            WorkspaceCreateOptions::new(
                "Workspace"
            )
        )
    }
}
