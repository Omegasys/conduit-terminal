use super::actions::ToolbarAction;

#[derive(Debug, Clone)]
pub struct OverflowItem {
    id: String,
    label: String,
    icon: Option<String>,
    action: Option<ToolbarAction>,
    enabled: bool,
    visible: bool,
}

impl OverflowItem {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        action: ToolbarAction,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            action: Some(action),
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

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn action(&self) -> Option<&ToolbarAction> {
        self.action.as_ref()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_icon(&mut self, icon: impl Into<String>) {
        self.icon = Some(icon.into());
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn can_activate(&self) -> bool {
        self.visible && self.enabled && self.action.is_some()
    }
}

#[derive(Debug, Clone, Default)]
pub struct OverflowMenu {
    items: Vec<OverflowItem>,
    open: bool,
}

impl OverflowMenu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, item: OverflowItem) {
        self.items.push(item);
    }

    pub fn items(&self) -> &[OverflowItem] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut [OverflowItem] {
        &mut self.items
    }

    pub fn get(&self, id: &str) -> Option<&OverflowItem> {
        self.items.iter().find(|item| item.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut OverflowItem> {
        self.items.iter_mut().find(|item| item.id() == id)
    }

    pub fn remove(&mut self, id: &str) -> Option<OverflowItem> {
        let index = self.items.iter().position(|item| item.id() == id)?;
        Some(self.items.remove(index))
    }

    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
