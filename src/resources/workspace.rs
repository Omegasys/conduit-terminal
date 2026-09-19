use std::collections::BTreeMap;

use super::resource::{Resource, ResourceError, ResourceId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceLayout {
    Freeform,
    Tiled,
    Horizontal,
    Vertical,
    MasterStack,
}

impl Default for WorkspaceLayout {
    fn default() -> Self {
        Self::Freeform
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceStartup {
    Restore,
    New,
    Prompt,
}

impl Default for WorkspaceStartup {
    fn default() -> Self {
        Self::Restore
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceResource {
    id: ResourceId,
    name: String,
    description: String,
    layout: WorkspaceLayout,
    startup: WorkspaceStartup,
    windows: Vec<String>,
    tabs: Vec<String>,
    active_tab: Option<String>,
    settings: BTreeMap<String, String>,
}

impl WorkspaceResource {
    pub fn new(
        id: ResourceId,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: String::new(),
            layout: WorkspaceLayout::default(),
            startup: WorkspaceStartup::default(),
            windows: Vec::new(),
            tabs: Vec::new(),
            active_tab: None,
            settings: BTreeMap::new(),
        }
    }

    pub fn from_resource(resource: &Resource) -> Result<Self, ResourceError> {
        if resource.kind()
            != super::resource::ResourceKind::Workspace
        {
            return Err(ResourceError::InvalidResource(
                "resource is not a workspace".to_owned(),
            ));
        }

        Ok(Self::new(resource.id(), resource.name()))
    }

    pub fn id(&self) -> ResourceId {
        self.id
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

    pub fn windows(&self) -> &[String] {
        &self.windows
    }

    pub fn tabs(&self) -> &[String] {
        &self.tabs
    }

    pub fn active_tab(&self) -> Option<&str> {
        self.active_tab.as_deref()
    }

    pub fn settings(&self) -> &BTreeMap<String, String> {
        &self.settings
    }

    pub fn set_description(&mut self, value: impl Into<String>) {
        self.description = value.into();
    }

    pub fn set_layout(&mut self, layout: WorkspaceLayout) {
        self.layout = layout;
    }

    pub fn set_startup(&mut self, startup: WorkspaceStartup) {
        self.startup = startup;
    }

    pub fn add_window(&mut self, window_id: impl Into<String>) {
        self.windows.push(window_id.into());
    }

    pub fn add_tab(&mut self, tab_id: impl Into<String>) {
        self.tabs.push(tab_id.into());
    }

    pub fn remove_window(&mut self, window_id: &str) {
        self.windows.retain(|id| id != window_id);
    }

    pub fn remove_tab(&mut self, tab_id: &str) {
        self.tabs.retain(|id| id != tab_id);

        if self.active_tab.as_deref() == Some(tab_id) {
            self.active_tab = None;
        }
    }

    pub fn set_active_tab(&mut self, tab_id: impl Into<String>) {
        self.active_tab = Some(tab_id.into());
    }

    pub fn set_setting(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.settings.insert(key.into(), value.into());
    }
}
