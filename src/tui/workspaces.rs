use crate::workspaces::{Workspace, WorkspaceId, WorkspaceStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceDisplayState {
    Normal,
    Active,
    Loading,
    Saving,
    Restoring,
    Closing,
    Closed,
    Error,
}

impl WorkspaceDisplayState {
    pub fn from_workspace(workspace: &Workspace, active: bool) -> Self {
        match workspace.status() {
            WorkspaceStatus::Loading => Self::Loading,
            WorkspaceStatus::Saving => Self::Saving,
            WorkspaceStatus::Restoring => Self::Restoring,
            WorkspaceStatus::Closing => Self::Closing,
            WorkspaceStatus::Closed => Self::Closed,
            _ if active => Self::Active,
            WorkspaceStatus::Created
            | WorkspaceStatus::Active
            | WorkspaceStatus::Inactive => Self::Normal,
        }
    }
}

pub struct WorkspaceDisplay {
    workspace: Workspace,
    display_state: WorkspaceDisplayState,
    visible: bool,
    index: usize,
}

impl WorkspaceDisplay {
    pub fn new(workspace: Workspace, active: bool, index: usize) -> Self {
        let display_state =
            WorkspaceDisplayState::from_workspace(&workspace, active);

        Self {
            workspace,
            display_state,
            visible: true,
            index,
        }
    }

    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    pub fn workspace_mut(&mut self) -> &mut Workspace {
        &mut self.workspace
    }

    pub fn id(&self) -> WorkspaceId {
        self.workspace.id()
    }

    pub fn name(&self) -> &str {
        self.workspace.name()
    }

    pub fn description(&self) -> &str {
        self.workspace.description()
    }

    pub fn status(&self) -> WorkspaceStatus {
        self.workspace.status()
    }

    pub fn display_state(&self) -> WorkspaceDisplayState {
        self.display_state
    }

    pub fn set_display_state(&mut self, state: WorkspaceDisplayState) {
        self.display_state = state;
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn dirty(&self) -> bool {
        self.workspace.dirty()
    }

    pub fn window_count(&self) -> usize {
        self.workspace.window_ids().len()
    }

    pub fn tab_count(&self) -> usize {
        self.workspace.tab_ids().len()
    }

    pub fn refresh_state(&mut self, active: bool) {
        self.display_state =
            WorkspaceDisplayState::from_workspace(&self.workspace, active);
    }
}

pub struct TuiWorkspaceView {
    workspaces: Vec<WorkspaceDisplay>,
    active: Option<WorkspaceId>,
    compact: bool,
    show_descriptions: bool,
    show_counts: bool,
}

impl Default for TuiWorkspaceView {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiWorkspaceView {
    pub fn new() -> Self {
        Self {
            workspaces: Vec::new(),
            active: None,
            compact: false,
            show_descriptions: true,
            show_counts: true,
        }
    }

    pub fn add(&mut self, workspace: Workspace) -> WorkspaceId {
        let id = workspace.id();
        let index = self.workspaces.len();
        let active = self.active.is_none();

        self.workspaces
            .push(WorkspaceDisplay::new(workspace, active, index));

        if active {
            self.active = Some(id);
        }

        self.refresh_indices();
        id
    }

    pub fn remove(&mut self, id: WorkspaceId) -> Option<WorkspaceDisplay> {
        let index = self.workspaces.iter().position(|w| w.id() == id)?;
        let removed = self.workspaces.remove(index);

        if self.active == Some(id) {
            self.active = self.workspaces.first().map(|w| w.id());
        }

        self.refresh_indices();
        self.refresh();

        Some(removed)
    }

    pub fn get(&self, id: WorkspaceId) -> Option<&WorkspaceDisplay> {
        self.workspaces.iter().find(|w| w.id() == id)
    }

    pub fn get_mut(&mut self, id: WorkspaceId) -> Option<&mut WorkspaceDisplay> {
        self.workspaces.iter_mut().find(|w| w.id() == id)
    }

    pub fn activate(&mut self, id: WorkspaceId) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        self.active = Some(id);
        self.refresh();
        true
    }

    pub fn active(&self) -> Option<&WorkspaceDisplay> {
        self.active.and_then(|id| self.get(id))
    }

    pub fn active_id(&self) -> Option<WorkspaceId> {
        self.active
    }

    pub fn workspaces(&self) -> &[WorkspaceDisplay] {
        &self.workspaces
    }

    pub fn visible_workspaces(&self) -> Vec<&WorkspaceDisplay> {
        self.workspaces
            .iter()
            .filter(|workspace| workspace.visible())
            .collect()
    }

    pub fn next(&mut self) {
        if self.workspaces.is_empty() {
            return;
        }

        let index = self
            .active
            .and_then(|id| self.workspaces.iter().position(|w| w.id() == id))
            .unwrap_or(0);

        let next = (index + 1) % self.workspaces.len();
        let id = self.workspaces[next].id();

        self.activate(id);
    }

    pub fn previous(&mut self) {
        if self.workspaces.is_empty() {
            return;
        }

        let index = self
            .active
            .and_then(|id| self.workspaces.iter().position(|w| w.id() == id))
            .unwrap_or(0);

        let previous = if index == 0 {
            self.workspaces.len() - 1
        } else {
            index - 1
        };

        let id = self.workspaces[previous].id();

        self.activate(id);
    }

    pub fn compact(&self) -> bool {
        self.compact
    }

    pub fn set_compact(&mut self, compact: bool) {
        self.compact = compact;
    }

    pub fn show_descriptions(&self) -> bool {
        self.show_descriptions
    }

    pub fn set_show_descriptions(&mut self, show: bool) {
        self.show_descriptions = show;
    }

    pub fn show_counts(&self) -> bool {
        self.show_counts
    }

    pub fn set_show_counts(&mut self, show: bool) {
        self.show_counts = show;
    }

    pub fn refresh(&mut self) {
        let active = self.active;

        for workspace in &mut self.workspaces {
            workspace.refresh_state(active == Some(workspace.id()));
        }
    }

    fn refresh_indices(&mut self) {
        for (index, workspace) in self.workspaces.iter_mut().enumerate() {
            workspace.set_index(index);
        }
    }

    pub fn len(&self) -> usize {
        self.workspaces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.workspaces.is_empty()
    }

    pub fn clear(&mut self) {
        self.workspaces.clear();
        self.active = None;
    }
}
