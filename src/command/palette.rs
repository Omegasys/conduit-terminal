use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CommandPaletteEntry {
    id: String,
    name: String,
    description: Option<String>,
    category: Option<String>,
    keywords: Vec<String>,
    enabled: bool,
}

impl CommandPaletteEntry {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            category: None,
            keywords: Vec::new(),
            enabled: true,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    pub fn keywords(&self) -> &[String] {
        &self.keywords
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = Some(description.into());
    }

    pub fn set_category(&mut self, category: impl Into<String>) {
        self.category = Some(category.into());
    }

    pub fn add_keyword(&mut self, keyword: impl Into<String>) {
        self.keywords.push(keyword.into());
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn matches(&self, query: &str) -> bool {
        if query.trim().is_empty() {
            return true;
        }

        let query = query.to_lowercase();

        self.name.to_lowercase().contains(&query)
            || self.id.to_lowercase().contains(&query)
            || self
                .description
                .as_ref()
                .is_some_and(|value| value.to_lowercase().contains(&query))
            || self
                .category
                .as_ref()
                .is_some_and(|value| value.to_lowercase().contains(&query))
            || self
                .keywords
                .iter()
                .any(|keyword| keyword.to_lowercase().contains(&query))
    }
}

#[derive(Debug, Clone)]
pub enum CommandPaletteItem {
    Command(CommandPaletteEntry),
    Separator,
    Header(String),
}

#[derive(Debug, Default)]
pub struct CommandPalette {
    entries: HashMap<String, CommandPaletteEntry>,
}

impl CommandPalette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, entry: CommandPaletteEntry) {
        self.entries.insert(entry.id.clone(), entry);
    }

    pub fn unregister(&mut self, id: &str) -> Option<CommandPaletteEntry> {
        self.entries.remove(id)
    }

    pub fn get(&self, id: &str) -> Option<&CommandPaletteEntry> {
        self.entries.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut CommandPaletteEntry> {
        self.entries.get_mut(id)
    }

    pub fn search(&self, query: &str) -> Vec<CommandPaletteEntry> {
        let mut results: Vec<_> = self
            .entries
            .values()
            .filter(|entry| entry.enabled() && entry.matches(query))
            .cloned()
            .collect();

        results.sort_by(|a, b| a.name().cmp(b.name()));
        results
    }

    pub fn all(&self) -> Vec<CommandPaletteEntry> {
        let mut entries: Vec<_> = self
            .entries
            .values()
            .filter(|entry| entry.enabled())
            .cloned()
            .collect();

        entries.sort_by(|a, b| a.name().cmp(b.name()));
        entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
