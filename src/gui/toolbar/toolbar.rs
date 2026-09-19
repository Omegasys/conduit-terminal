use super::{
    actions::ToolbarAction,
    buttons::{ToolbarButton, ToolbarButtonGroup},
    overflow::{OverflowItem, OverflowMenu},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarLayout {
    Horizontal,
    Vertical,
    Compact,
    Expanded,
}

#[derive(Debug, Clone)]
pub struct Toolbar {
    id: String,
    title: String,
    position: ToolbarPosition,
    layout: ToolbarLayout,
    visible: bool,
    enabled: bool,
    movable: bool,
    overflow_enabled: bool,
    buttons: Vec<ToolbarButton>,
    groups: Vec<ToolbarButtonGroup>,
    overflow: OverflowMenu,
}

impl Toolbar {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            position: ToolbarPosition::Top,
            layout: ToolbarLayout::Horizontal,
            visible: true,
            enabled: true,
            movable: true,
            overflow_enabled: true,
            buttons: Vec::new(),
            groups: Vec::new(),
            overflow: OverflowMenu::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn position(&self) -> ToolbarPosition {
        self.position
    }

    pub fn layout(&self) -> ToolbarLayout {
        self.layout
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_movable(&self) -> bool {
        self.movable
    }

    pub fn overflow_enabled(&self) -> bool {
        self.overflow_enabled
    }

    pub fn buttons(&self) -> &[ToolbarButton] {
        &self.buttons
    }

    pub fn buttons_mut(&mut self) -> &mut [ToolbarButton] {
        &mut self.buttons
    }

    pub fn groups(&self) -> &[ToolbarButtonGroup] {
        &self.groups
    }

    pub fn groups_mut(&mut self) -> &mut [ToolbarButtonGroup] {
        &mut self.groups
    }

    pub fn overflow(&self) -> &OverflowMenu {
        &self.overflow
    }

    pub fn overflow_mut(&mut self) -> &mut OverflowMenu {
        &mut self.overflow
    }

    pub fn add_button(&mut self, button: ToolbarButton) {
        self.buttons.push(button);
    }

    pub fn insert_button(&mut self, index: usize, button: ToolbarButton) {
        let index = index.min(self.buttons.len());
        self.buttons.insert(index, button);
    }

    pub fn remove_button(&mut self, id: &str) -> Option<ToolbarButton> {
        let index = self.buttons.iter().position(|button| button.id() == id)?;
        Some(self.buttons.remove(index))
    }

    pub fn get_button(&self, id: &str) -> Option<&ToolbarButton> {
        self.buttons.iter().find(|button| button.id() == id)
    }

    pub fn get_button_mut(&mut self, id: &str) -> Option<&mut ToolbarButton> {
        self.buttons.iter_mut().find(|button| button.id() == id)
    }

    pub fn add_group(&mut self, group: ToolbarButtonGroup) {
        self.groups.push(group);
    }

    pub fn get_group(&self, id: &str) -> Option<&ToolbarButtonGroup> {
        self.groups.iter().find(|group| group.id() == id)
    }

    pub fn get_group_mut(&mut self, id: &str) -> Option<&mut ToolbarButtonGroup> {
        self.groups.iter_mut().find(|group| group.id() == id)
    }

    pub fn remove_group(&mut self, id: &str) -> Option<ToolbarButtonGroup> {
        let index = self.groups.iter().position(|group| group.id() == id)?;
        Some(self.groups.remove(index))
    }

    pub fn set_position(&mut self, position: ToolbarPosition) {
        self.position = position;
    }

    pub fn set_layout(&mut self, layout: ToolbarLayout) {
        self.layout = layout;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        for button in &mut self.buttons {
            button.set_enabled(enabled);
        }

        for group in &mut self.groups {
            group.set_enabled(enabled);
        }
    }

    pub fn set_movable(&mut self, movable: bool) {
        self.movable = movable;
    }

    pub fn set_overflow_enabled(&mut self, enabled: bool) {
        self.overflow_enabled = enabled;

        if !enabled {
            self.overflow.close();
        }
    }

    pub fn move_button_to_overflow(&mut self, id: &str) -> bool {
        let Some(button) = self.get_button_mut(id) else {
            return false;
        };

        button.set_overflow(true);

        if let Some(action) = button.action().cloned() {
            let mut item = OverflowItem::new(
                button.id().to_string(),
                button.label().to_string(),
                action,
            );

            if let Some(icon) = button.icon() {
                item.set_icon(icon.to_string());
            }

            self.overflow.add(item);
        }

        true
    }

    pub fn restore_button_from_overflow(&mut self, id: &str) -> bool {
        let Some(button) = self.get_button_mut(id) else {
            return false;
        };

        button.set_overflow(false);
        self.overflow.remove(id);

        true
    }

    pub fn activate(&self, id: &str) -> Option<&ToolbarAction> {
        let button = self.get_button(id)?;

        if !self.visible || !self.enabled || !button.can_activate() {
            return None;
        }

        button.action()
    }

    pub fn activate_overflow(&self, id: &str) -> Option<&ToolbarAction> {
        let item = self.overflow.get(id)?;

        if !self.overflow_enabled || !item.can_activate() {
            return None;
        }

        item.action()
    }

    pub fn visible_buttons(&self) -> impl Iterator<Item = &ToolbarButton> {
        self.buttons
            .iter()
            .filter(|button| button.is_visible())
            .filter(|button| !button.is_overflow())
    }

    pub fn overflow_buttons(&self) -> impl Iterator<Item = &ToolbarButton> {
        self.buttons
            .iter()
            .filter(|button| button.is_visible())
            .filter(|button| button.is_overflow())
    }

    pub fn clear(&mut self) {
        self.buttons.clear();
        self.groups.clear();
        self.overflow.clear();
    }

    pub fn button_count(&self) -> usize {
        self.buttons.len()
    }

    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buttons.is_empty() && self.groups.is_empty()
    }
}
