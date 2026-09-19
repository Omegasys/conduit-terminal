#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolCategory {
    Terminal,
    FileSystem,
    Monitoring,
    Networking,
    Development,
    Security,
    Diagnostics,
    Media,
    Custom,
}

#[derive(Debug, Clone)]
pub struct SidebarTool {
    id: String,
    name: String,
    description: String,
    category: ToolCategory,
    icon: Option<String>,
    command: Option<String>,
    enabled: bool,
    visible: bool,
    pinned: bool,
}

impl SidebarTool {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        category: ToolCategory,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            category,
            icon: None,
            command: None,
            enabled: true,
            visible: true,
            pinned: false,
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

    pub fn category(&self) -> ToolCategory {
        self.category
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn command(&self) -> Option<&str> {
        self.command.as_deref()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_pinned(&self) -> bool {
        self.pinned
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn set_icon(&mut self, icon: Option<String>) {
        self.icon = icon;
    }

    pub fn set_command(&mut self, command: Option<String>) {
        self.command = command;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_pinned(&mut self, pinned: bool) {
        self.pinned = pinned;
    }

    pub fn can_activate(&self) -> bool {
        self.visible && self.enabled
    }
}

#[derive(Debug, Default)]
pub struct SidebarToolManager {
    tools: Vec<SidebarTool>,
}

impl SidebarToolManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, tool: SidebarTool) {
        if let Some(existing) =
            self.tools.iter_mut().find(|item| item.id() == tool.id())
        {
            *existing = tool;
            return;
        }

        self.tools.push(tool);
    }

    pub fn remove(&mut self, id: &str) -> Option<SidebarTool> {
        let index = self.tools.iter().position(|item| item.id() == id)?;
        Some(self.tools.remove(index))
    }

    pub fn get(&self, id: &str) -> Option<&SidebarTool> {
        self.tools.iter().find(|item| item.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut SidebarTool> {
        self.tools.iter_mut().find(|item| item.id() == id)
    }

    pub fn by_category(
        &self,
        category: ToolCategory,
    ) -> impl Iterator<Item = &SidebarTool> {
        self.tools
            .iter()
            .filter(move |item| item.category() == category)
    }

    pub fn pinned(&self) -> impl Iterator<Item = &SidebarTool> {
        self.tools.iter().filter(|item| item.is_pinned())
    }

    pub fn visible(&self) -> impl Iterator<Item = &SidebarTool> {
        self.tools.iter().filter(|item| item.is_visible())
    }

    pub fn iter(&self) -> impl Iterator<Item = &SidebarTool> {
        self.tools.iter()
    }

    pub fn clear(&mut self) {
        self.tools.clear();
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}
