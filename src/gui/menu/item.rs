use super::actions::MenuAction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuItemKind {
    Action,
    Check,
    Radio,
    Separator,
    Submenu,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    id: String,
    label: String,
    kind: MenuItemKind,
    action: Option<MenuAction>,
    enabled: bool,
    visible: bool,
    checked: bool,
    radio_group: Option<String>,
    accelerator: Option<String>,
    submenu: Option<String>,
}

impl MenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, action: MenuAction) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: MenuItemKind::Action,
            action: Some(action),
            enabled: true,
            visible: true,
            checked: false,
            radio_group: None,
            accelerator: None,
            submenu: None,
        }
    }

    pub fn separator(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: String::new(),
            kind: MenuItemKind::Separator,
            action: None,
            enabled: false,
            visible: true,
            checked: false,
            radio_group: None,
            accelerator: None,
            submenu: None,
        }
    }

    pub fn check(
        id: impl Into<String>,
        label: impl Into<String>,
        action: MenuAction,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: MenuItemKind::Check,
            action: Some(action),
            enabled: true,
            visible: true,
            checked: false,
            radio_group: None,
            accelerator: None,
            submenu: None,
        }
    }

    pub fn radio(
        id: impl Into<String>,
        label: impl Into<String>,
        action: MenuAction,
        group: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: MenuItemKind::Radio,
            action: Some(action),
            enabled: true,
            visible: true,
            checked: false,
            radio_group: Some(group.into()),
            accelerator: None,
            submenu: None,
        }
    }

    pub fn submenu(
        id: impl Into<String>,
        label: impl Into<String>,
        submenu: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: MenuItemKind::Submenu,
            action: None,
            enabled: true,
            visible: true,
            checked: false,
            radio_group: None,
            accelerator: None,
            submenu: Some(submenu.into()),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn kind(&self) -> &MenuItemKind {
        &self.kind
    }

    pub fn action(&self) -> Option<&MenuAction> {
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

    pub fn radio_group(&self) -> Option<&str> {
        self.radio_group.as_deref()
    }

    pub fn accelerator(&self) -> Option<&str> {
        self.accelerator.as_deref()
    }

    pub fn submenu(&self) -> Option<&str> {
        self.submenu.as_deref()
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    pub fn set_accelerator(&mut self, accelerator: impl Into<String>) {
        self.accelerator = Some(accelerator.into());
    }

    pub fn clear_accelerator(&mut self) {
        self.accelerator = None;
    }

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    pub fn can_activate(&self) -> bool {
        self.visible
            && self.enabled
            && !matches!(self.kind, MenuItemKind::Separator)
    }
}
