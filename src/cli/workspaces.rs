use crate::workspaces::WorkspaceId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkspaceCommandAction {
    Create,
    Close,
    Focus,
    Next,
    Previous,
    Activate,
    Save,
    Restore,
    List,
}

impl WorkspaceCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Create => "new-workspace",
            Self::Close => "close-workspace",
            Self::Focus | Self::Activate => "activate-workspace",
            Self::Next => "next-workspace",
            Self::Previous => "previous-workspace",
            Self::Save => "save-workspace",
            Self::Restore => "restore-workspace",
            Self::List => "list-workspaces",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceCommand {
    action: WorkspaceCommandAction,
    workspace_id: Option<WorkspaceId>,
    name: Option<String>,
}

impl WorkspaceCommand {
    pub fn new(action: WorkspaceCommandAction) -> Self {
        Self {
            action,
            workspace_id: None,
            name: None,
        }
    }

    pub fn with_id(
        action: WorkspaceCommandAction,
        workspace_id: WorkspaceId,
    ) -> Self {
        Self {
            action,
            workspace_id: Some(workspace_id),
            name: None,
        }
    }

    pub fn with_name(
        action: WorkspaceCommandAction,
        name: impl Into<String>,
    ) -> Self {
        Self {
            action,
            workspace_id: None,
            name: Some(name.into()),
        }
    }

    pub fn action(&self) -> WorkspaceCommandAction {
        self.action
    }

    pub fn workspace_id(&self) -> Option<WorkspaceId> {
        self.workspace_id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_workspace_id(&mut self, id: WorkspaceId) {
        self.workspace_id = Some(id);
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = Some(name.into());
    }

    pub fn is_targeted(&self) -> bool {
        self.workspace_id.is_some() || self.name.is_some()
    }
}
