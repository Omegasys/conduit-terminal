#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavigationTarget {
    Home,
    Root,
    CurrentDirectory,
    Recent,
    Favorites,
    FileSystem,
    Processes,
    Search,
    Settings,
    Custom,
}

#[derive(Debug, Clone)]
pub struct NavigationEntry {
    id: String,
    label: String,
    target: NavigationTarget,
    path: Option<String>,
    icon: Option<String>,
    enabled: bool,
    visible: bool,
    pinned: bool,
}

impl NavigationEntry {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        target: NavigationTarget,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            target,
            path: None,
            icon: None,
            enabled: true,
            visible: true,
            pinned: false,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn target(&self) -> NavigationTarget {
        self.target
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
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

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }

    pub fn set_icon(&mut self, icon: Option<String>) {
        self.icon = icon;
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
pub struct NavigationManager {
    entries: Vec<NavigationEntry>,
    active: Option<String>,
}

impl NavigationManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: NavigationEntry) {
        if let Some(existing) =
            self.entries.iter_mut().find(|item| item.id() == entry.id())
        {
            *existing = entry;
            return;
        }

        self.entries.push(entry);
    }

    pub fn remove(&mut self, id: &str) -> Option<NavigationEntry> {
        let index = self.entries.iter().position(|item| item.id() == id)?;

        if self.active.as_deref() == Some(id) {
            self.active = None;
        }

        Some(self.entries.remove(index))
    }

    pub fn get(&self, id: &str) -> Option<&NavigationEntry> {
        self.entries.iter().find(|item| item.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut NavigationEntry> {
        self.entries.iter_mut().find(|item| item.id() == id)
    }

    pub fn activate(&mut self, id: &str) -> bool {
        if !self
            .entries
            .iter()
            .any(|item| item.id() == id && item.can_activate())
        {
            return false;
        }

        self.active = Some(id.to_string());
        true
    }

    pub fn active(&self) -> Option<&NavigationEntry> {
        self.active.as_deref().and_then(|id| self.get(id))
    }

    pub fn active_id(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub fn by_target(
        &self,
        target: NavigationTarget,
    ) -> impl Iterator<Item = &NavigationEntry> {
        self.entries
            .iter()
            .filter(move |item| item.target() == target)
    }

    pub fn visible(&self) -> impl Iterator<Item = &NavigationEntry> {
        self.entries.iter().filter(|item| item.is_visible())
    }

    pub fn pinned(&self) -> impl Iterator<Item = &NavigationEntry> {
        self.entries.iter().filter(|item| item.is_pinned())
    }

    pub fn iter(&self) -> impl Iterator<Item = &NavigationEntry> {
        self.entries.iter()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.active = None;
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
