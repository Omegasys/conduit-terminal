use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    View,
    Terminal,
    Tabs,
    Panes,
    Workspaces,
    Sessions,
    Connections,
    Tools,
    Configuration,
    Plugins,
    Navigation,
    Security,
    Help,
    Custom,
}

#[derive(Debug, Clone)]
pub struct CommandCategoryManager {
    categories: Vec<CommandCategory>,
    names: HashMap<CommandCategory, String>,
}

impl CommandCategoryManager {
    pub fn new() -> Self {
        let mut manager = Self {
            categories: Vec::new(),
            names: HashMap::new(),
        };

        manager.initialize_defaults();
        manager
    }

    fn initialize_defaults(&mut self) {
        let defaults = [
            (CommandCategory::File, "File"),
            (CommandCategory::Edit, "Edit"),
            (CommandCategory::View, "View"),
            (CommandCategory::Terminal, "Terminal"),
            (CommandCategory::Tabs, "Tabs"),
            (CommandCategory::Panes, "Panes"),
            (CommandCategory::Workspaces, "Workspaces"),
            (CommandCategory::Sessions, "Sessions"),
            (CommandCategory::Connections, "Connections"),
            (CommandCategory::Tools, "Tools"),
            (CommandCategory::Configuration, "Configuration"),
            (CommandCategory::Plugins, "Plugins"),
            (CommandCategory::Navigation, "Navigation"),
            (CommandCategory::Security, "Security"),
            (CommandCategory::Help, "Help"),
        ];

        for (category, name) in defaults {
            self.add(category, name);
        }
    }

    pub fn add(
        &mut self,
        category: CommandCategory,
        name: impl Into<String>,
    ) {
        if !self.categories.contains(&category) {
            self.categories.push(category);
        }

        self.names.insert(category, name.into());
    }

    pub fn remove(&mut self, category: CommandCategory) {
        self.categories.retain(|item| *item != category);
        self.names.remove(&category);
    }

    pub fn name(&self, category: CommandCategory) -> Option<&str> {
        self.names.get(&category).map(String::as_str)
    }

    pub fn categories(&self) -> &[CommandCategory] {
        &self.categories
    }

    pub fn contains(&self, category: CommandCategory) -> bool {
        self.categories.contains(&category)
    }

    pub fn len(&self) -> usize {
        self.categories.len()
    }

    pub fn is_empty(&self) -> bool {
        self.categories.is_empty()
    }

    pub fn clear(&mut self) {
        self.categories.clear();
        self.names.clear();
    }
}

impl Default for CommandCategoryManager {
    fn default() -> Self {
        Self::new()
    }
}
