use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SettingsCategory {
    General,
    Appearance,
    Terminal,
    Keyboard,
    Profiles,
    Workspaces,
    Security,
    Plugins,
    Accessibility,
    Advanced,
}

impl SettingsCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Appearance => "Appearance",
            Self::Terminal => "Terminal",
            Self::Keyboard => "Keyboard",
            Self::Profiles => "Profiles",
            Self::Workspaces => "Workspaces",
            Self::Security => "Security",
            Self::Plugins => "Plugins",
            Self::Accessibility => "Accessibility",
            Self::Advanced => "Advanced",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsItem {
    key: String,
    label: String,
    description: String,
    value: ConfigValue,
    editable: bool,
}

impl SettingsItem {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
        value: ConfigValue,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            description: description.into(),
            value,
            editable: true,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn value(&self) -> &ConfigValue {
        &self.value
    }

    pub fn set_value(&mut self, value: ConfigValue) {
        if self.editable {
            self.value = value;
        }
    }

    pub fn editable(&self) -> bool {
        self.editable
    }

    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
    }
}

pub struct SettingsState {
    visible: bool,
    active_category: SettingsCategory,
    categories: Vec<SettingsCategory>,
    items: BTreeMap<String, SettingsItem>,
    selected_key: Option<String>,
    modified: bool,
    search: String,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            visible: false,
            active_category: SettingsCategory::General,
            categories: vec![
                SettingsCategory::General,
                SettingsCategory::Appearance,
                SettingsCategory::Terminal,
                SettingsCategory::Keyboard,
                SettingsCategory::Profiles,
                SettingsCategory::Workspaces,
                SettingsCategory::Security,
                SettingsCategory::Plugins,
                SettingsCategory::Accessibility,
                SettingsCategory::Advanced,
            ],
            items: BTreeMap::new(),
            selected_key: None,
            modified: false,
            search: String::new(),
        }
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn active_category(&self) -> SettingsCategory {
        self.active_category
    }

    pub fn set_active_category(&mut self, category: SettingsCategory) {
        self.active_category = category;
        self.selected_key = None;
    }

    pub fn categories(&self) -> &[SettingsCategory] {
        &self.categories
    }

    pub fn add_item(&mut self, item: SettingsItem) {
        self.items.insert(item.key.clone(), item);
    }

    pub fn remove_item(&mut self, key: &str) -> Option<SettingsItem> {
        let removed = self.items.remove(key);

        if self.selected_key.as_deref() == Some(key) {
            self.selected_key = None;
        }

        removed
    }

    pub fn item(&self, key: &str) -> Option<&SettingsItem> {
        self.items.get(key)
    }

    pub fn item_mut(&mut self, key: &str) -> Option<&mut SettingsItem> {
        self.items.get_mut(key)
    }

    pub fn items(&self) -> impl Iterator<Item = &SettingsItem> {
        self.items.values()
    }

    pub fn select(&mut self, key: impl Into<String>) -> bool {
        let key = key.into();

        if self.items.contains_key(&key) {
            self.selected_key = Some(key);
            true
        } else {
            false
        }
    }

    pub fn selected(&self) -> Option<&SettingsItem> {
        self.selected_key
            .as_deref()
            .and_then(|key| self.items.get(key))
    }

    pub fn selected_key(&self) -> Option<&str> {
        self.selected_key.as_deref()
    }

    pub fn set_value(&mut self, key: &str, value: ConfigValue) -> bool {
        if let Some(item) = self.items.get_mut(key) {
            if item.editable() {
                item.set_value(value);
                self.modified = true;
                return true;
            }
        }

        false
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    pub fn mark_clean(&mut self) {
        self.modified = false;
    }

    pub fn mark_modified(&mut self) {
        self.modified = true;
    }

    pub fn search(&self) -> &str {
        &self.search
    }

    pub fn set_search(&mut self, search: impl Into<String>) {
        self.search = search.into();
    }

    pub fn clear_search(&mut self) {
        self.search.clear();
    }

    pub fn filtered_items(&self) -> Vec<&SettingsItem> {
        let query = self.search.trim().to_lowercase();

        if query.is_empty() {
            return self.items.values().collect();
        }

        self.items
            .values()
            .filter(|item| {
                item.key.to_lowercase().contains(&query)
                    || item.label.to_lowercase().contains(&query)
                    || item.description.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.selected_key = None;
        self.modified = false;
    }
}
