use super::actions::ToolbarAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarButtonKind {
    Action,
    Toggle,
    Dropdown,
    Separator,
}

#[derive(Debug, Clone)]
pub struct ToolbarButton {
    id: String,
    label: String,
    tooltip: Option<String>,
    icon: Option<String>,
    kind: ToolbarButtonKind,
    action: Option<ToolbarAction>,
    enabled: bool,
    visible: bool,
    checked: bool,
    overflow: bool,
}

impl ToolbarButton {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        action: ToolbarAction,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            tooltip: None,
            icon: None,
            kind: ToolbarButtonKind::Action,
            action: Some(action),
            enabled: true,
            visible: true,
            checked: false,
            overflow: false,
        }
    }

    pub fn toggle(
        id: impl Into<String>,
        label: impl Into<String>,
        action: ToolbarAction,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            tooltip: None,
            icon: None,
            kind: ToolbarButtonKind::Toggle,
            action: Some(action),
            enabled: true,
            visible: true,
            checked: false,
            overflow: false,
        }
    }

    pub fn separator(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: String::new(),
            tooltip: None,
            icon: None,
            kind: ToolbarButtonKind::Separator,
            action: None,
            enabled: false,
            visible: true,
            checked: false,
            overflow: false,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn tooltip(&self) -> Option<&str> {
        self.tooltip.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn kind(&self) -> ToolbarButtonKind {
        self.kind
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

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn is_overflow(&self) -> bool {
        self.overflow
    }

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    pub fn set_tooltip(&mut self, tooltip: impl Into<String>) {
        self.tooltip = Some(tooltip.into());
    }

    pub fn clear_tooltip(&mut self) {
        self.tooltip = None;
    }

    pub fn set_icon(&mut self, icon: impl Into<String>) {
        self.icon = Some(icon.into());
    }

    pub fn clear_icon(&mut self) {
        self.icon = None;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_checked(&mut self, checked: bool) {
        if self.kind == ToolbarButtonKind::Toggle {
            self.checked = checked;
        }
    }

    pub fn set_overflow(&mut self, overflow: bool) {
        self.overflow = overflow;
    }

    pub fn can_activate(&self) -> bool {
        self.visible
            && self.enabled
            && !matches!(self.kind, ToolbarButtonKind::Separator)
    }
}

#[derive(Debug, Clone)]
pub struct ToolbarButtonGroup {
    id: String,
    buttons: Vec<ToolbarButton>,
    enabled: bool,
    visible: bool,
}

impl ToolbarButtonGroup {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            buttons: Vec::new(),
            enabled: true,
            visible: true,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn buttons(&self) -> &[ToolbarButton] {
        &self.buttons
    }

    pub fn buttons_mut(&mut self) -> &mut [ToolbarButton] {
        &mut self.buttons
    }

    pub fn add(&mut self, button: ToolbarButton) {
        self.buttons.push(button);
    }

    pub fn get(&self, id: &str) -> Option<&ToolbarButton> {
        self.buttons.iter().find(|button| button.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ToolbarButton> {
        self.buttons.iter_mut().find(|button| button.id() == id)
    }

    pub fn remove(&mut self, id: &str) -> Option<ToolbarButton> {
        let index = self.buttons.iter().position(|button| button.id() == id)?;
        Some(self.buttons.remove(index))
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        for button in &mut self.buttons {
            button.set_enabled(enabled);
        }
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

    pub fn len(&self) -> usize {
        self.buttons.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buttons.is_empty()
    }
}
