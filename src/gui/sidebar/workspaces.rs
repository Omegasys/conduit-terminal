#[derive(Debug, Clone)]
pub struct SidebarWorkspace {
    id: String,
    name: String,
    description: String,
    active: bool,
    pinned: bool,
    dirty: bool,
    visible: bool,
    enabled: bool,
    window_count: usize,
    tab_count: usize,
}

impl SidebarWorkspace {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            active: false,
            pinned: false,
            dirty: false,
            visible: true,
            enabled: true,
            window_count: 0,
            tab_count: 0,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_pinned(&self) -> bool {
        self.pinned
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn window_count(&self) -> usize {
        self.window_count
    }

    pub fn tab_count(&self) -> usize {
        self.tab_count
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_pinned(&mut self, pinned: bool) {
        self.pinned = pinned;
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_counts(&mut self, windows: usize, tabs: usize) {
        self.window_count = windows;
        self.tab_count = tabs;
    }

    pub fn can_activate(&self) -> bool {
        self.visible && self.enabled
    }
}

#[derive(Debug, Default)]
pub struct SidebarWorkspaceManager {
    workspaces: Vec<SidebarWorkspace>,
    active: Option<String>,
}

impl SidebarWorkspaceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, workspace: SidebarWorkspace) {
        if let Some(existing) = self
            .workspaces
            .iter_mut()
            .find(|item| item.id() == workspace.id())
        {
            *existing = workspace;
            return;
        }

        self.workspaces.push(workspace);
    }

    pub fn remove(&mut self, id: &str) -> Option<SidebarWorkspace> {
        let index = self.workspaces.iter().position(|item| item.id() == id)?;

        if self.active.as_deref() == Some(id) {
            self.active = None;
        }

        Some(self.workspaces.remove(index))
    }

    pub fn get(&self, id: &str) -> Option<&SidebarWorkspace> {
        self.workspaces.iter().find(|item| item.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut SidebarWorkspace> {
        self.workspaces
            .iter_mut()
            .find(|item| item.id() == id)
    }

    pub fn activate(&mut self, id: &str) -> bool {
        if !self
            .workspaces
            .iter()
            .any(|item| item.id() == id && item.can_activate())
        {
            return false;
        }

        for workspace in &mut self.workspaces {
            workspace.set_active(workspace.id() == id);
        }

        self.active = Some(id.to_string());
        true
    }

    pub fn active(&self) -> Option<&SidebarWorkspace> {
        self.active
            .as_deref()
            .and_then(|id| self.get(id))
    }

    pub fn active_id(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub fn iter(&self) -> impl Iterator<Item = &SidebarWorkspace> {
        self.workspaces.iter()
    }

    pub fn visible(&self) -> impl Iterator<Item = &SidebarWorkspace> {
        self.workspaces.iter().filter(|item| item.is_visible())
    }

    pub fn clear(&mut self) {
        self.workspaces.clear();
        self.active = None;
    }

    pub fn len(&self) -> usize {
        self.workspaces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.workspaces.is_empty()
    }
}
