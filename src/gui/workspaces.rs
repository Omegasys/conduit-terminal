use crate::resources::{
    WorkspaceLayout,
    WorkspaceResource,
    WorkspaceStartup,
};

/// GUI representation of a workspace.
#[derive(Debug, Clone)]
pub struct GuiWorkspace {
    name: String,
    description: String,
    layout: WorkspaceLayout,
    startup: WorkspaceStartup,
    active: bool,
    dirty: bool,
}

impl GuiWorkspace {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            layout: WorkspaceLayout::Freeform,
            startup: WorkspaceStartup::Restore,
            active: false,
            dirty: false,
        }
    }

    pub fn from_resource(workspace: &WorkspaceResource) -> Self {
        Self {
            name: workspace.name().to_string(),
            description: workspace.description().to_string(),
            layout: workspace.layout(),
            startup: workspace.startup(),
            active: false,
            dirty: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn layout(&self) -> WorkspaceLayout {
        self.layout
    }

    pub fn startup(&self) -> WorkspaceStartup {
        self.startup
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
        self.dirty = true;
    }

    pub fn set_layout(&mut self, layout: WorkspaceLayout) {
        self.layout = layout;
        self.dirty = true;
    }

    pub fn set_startup(&mut self, startup: WorkspaceStartup) {
        self.startup = startup;
        self.dirty = true;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

/// Manages workspaces displayed by the GUI.
#[derive(Debug, Default)]
pub struct GuiWorkspaceManager {
    workspaces: Vec<GuiWorkspace>,
    active: Option<String>,
}

impl GuiWorkspaceManager {
    pub fn new() -> Self {
        Self {
            workspaces: Vec::new(),
            active: None,
        }
    }

    pub fn add(&mut self, workspace: GuiWorkspace) -> bool {
        if self
            .workspaces
            .iter()
            .any(|existing| existing.name() == workspace.name())
        {
            return false;
        }

        self.workspaces.push(workspace);
        true
    }

    pub fn add_from_resource(
        &mut self,
        workspace: &WorkspaceResource,
    ) -> bool {
        self.add(GuiWorkspace::from_resource(workspace))
    }

    pub fn remove(&mut self, name: &str) -> Option<GuiWorkspace> {
        let index = self
            .workspaces
            .iter()
            .position(|workspace| workspace.name() == name)?;

        let removed = self.workspaces.remove(index);

        if self.active.as_deref() == Some(name) {
            self.active = None;
        }

        Some(removed)
    }

    pub fn get(&self, name: &str) -> Option<&GuiWorkspace> {
        self.workspaces
            .iter()
            .find(|workspace| workspace.name() == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut GuiWorkspace> {
        self.workspaces
            .iter_mut()
            .find(|workspace| workspace.name() == name)
    }

    pub fn activate(&mut self, name: &str) -> bool {
        if !self.workspaces.iter().any(|workspace| workspace.name() == name) {
            return false;
        }

        for workspace in &mut self.workspaces {
            workspace.set_active(workspace.name() == name);
        }

        self.active = Some(name.to_string());
        true
    }

    pub fn active(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub fn iter(&self) -> impl Iterator<Item = &GuiWorkspace> {
        self.workspaces.iter()
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
