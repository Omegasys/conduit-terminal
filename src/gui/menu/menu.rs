use super::item::MenuItem;

#[derive(Debug, Clone)]
pub struct Menu {
    id: String,
    label: String,
    items: Vec<MenuItem>,
    enabled: bool,
    visible: bool,
}

impl Menu {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            items: Vec::new(),
            enabled: true,
            visible: true,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn items(&self) -> &[MenuItem] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut [MenuItem] {
        &mut self.items
    }

    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    pub fn insert_item(&mut self, index: usize, item: MenuItem) {
        let index = index.min(self.items.len());
        self.items.insert(index, item);
    }

    pub fn remove_item(&mut self, id: &str) -> Option<MenuItem> {
        let index = self.items.iter().position(|item| item.id() == id)?;
        Some(self.items.remove(index))
    }

    pub fn get(&self, id: &str) -> Option<&MenuItem> {
        self.items.iter().find(|item| item.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut MenuItem> {
        self.items.iter_mut().find(|item| item.id() == id)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn can_open(&self) -> bool {
        self.enabled && self.visible
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
