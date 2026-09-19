use crate::tabs::TabId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabContextAction {
    NewTab,
    CloseTab,
    CloseOtherTabs,
    CloseTabsToRight,
    ReopenClosedTab,

    DuplicateTab,
    RenameTab,

    PinTab,
    UnpinTab,
    MuteTab,
    UnmuteTab,

    MoveToNewWindow,
    MoveToExistingWindow,

    MoveToNextPosition,
    MoveToPreviousPosition,
    MoveToFirstPosition,
    MoveToLastPosition,

    CopyTabTitle,
    CopyTabPath,

    ReloadTab,
    DetachTab,

    Custom(String),
}

#[derive(Debug, Clone)]
pub struct TabContextMenuItem {
    id: String,
    label: String,
    action: TabContextAction,
    enabled: bool,
    visible: bool,
    checked: bool,
}

impl TabContextMenuItem {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        action: TabContextAction,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            action,
            enabled: true,
            visible: true,
            checked: false,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn action(&self) -> &TabContextAction {
        &self.action
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

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    pub fn can_activate(&self) -> bool {
        self.visible && self.enabled
    }
}

#[derive(Debug, Clone)]
pub struct TabContextMenu {
    tab_id: Option<TabId>,
    items: Vec<TabContextMenuItem>,
    open: bool,
}

impl Default for TabContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl TabContextMenu {
    pub fn new() -> Self {
        let mut menu = Self {
            tab_id: None,
            items: Vec::new(),
            open: false,
        };

        menu.populate_defaults();
        menu
    }

    fn populate_defaults(&mut self) {
        self.items.push(TabContextMenuItem::new(
            "new-tab",
            "New Tab",
            TabContextAction::NewTab,
        ));

        self.items.push(TabContextMenuItem::new(
            "close-tab",
            "Close Tab",
            TabContextAction::CloseTab,
        ));

        self.items.push(TabContextMenuItem::new(
            "close-other-tabs",
            "Close Other Tabs",
            TabContextAction::CloseOtherTabs,
        ));

        self.items.push(TabContextMenuItem::new(
            "close-tabs-right",
            "Close Tabs to the Right",
            TabContextAction::CloseTabsToRight,
        ));

        self.items.push(TabContextMenuItem::new(
            "reopen-closed-tab",
            "Reopen Closed Tab",
            TabContextAction::ReopenClosedTab,
        ));

        self.items.push(TabContextMenuItem::new(
            "duplicate-tab",
            "Duplicate Tab",
            TabContextAction::DuplicateTab,
        ));

        self.items.push(TabContextMenuItem::new(
            "rename-tab",
            "Rename Tab",
            TabContextAction::RenameTab,
        ));

        self.items.push(TabContextMenuItem::new(
            "pin-tab",
            "Pin Tab",
            TabContextAction::PinTab,
        ));

        self.items.push(TabContextMenuItem::new(
            "mute-tab",
            "Mute Tab",
            TabContextAction::MuteTab,
        ));

        self.items.push(TabContextMenuItem::new(
            "move-new-window",
            "Move to New Window",
            TabContextAction::MoveToNewWindow,
        ));

        self.items.push(TabContextMenuItem::new(
            "copy-title",
            "Copy Tab Title",
            TabContextAction::CopyTabTitle,
        ));

        self.items.push(TabContextMenuItem::new(
            "reload-tab",
            "Reload Tab",
            TabContextAction::ReloadTab,
        ));
    }

    pub fn tab_id(&self) -> Option<TabId> {
        self.tab_id
    }

    pub fn items(&self) -> &[TabContextMenuItem] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut [TabContextMenuItem] {
        &mut self.items
    }

    pub fn get(&self, id: &str) -> Option<&TabContextMenuItem> {
        self.items.iter().find(|item| item.id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut TabContextMenuItem> {
        self.items.iter_mut().find(|item| item.id() == id)
    }

    pub fn open_for(&mut self, tab_id: TabId) {
        self.tab_id = Some(tab_id);
        self.open = true;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.tab_id = None;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn set_pin_state(&mut self, pinned: bool) {
        if let Some(item) = self.get_mut("pin-tab") {
            item.set_checked(pinned);

            if pinned {
                item.label = "Unpin Tab".to_string();
            } else {
                item.label = "Pin Tab".to_string();
            }
        }
    }

    pub fn set_mute_state(&mut self, muted: bool) {
        if let Some(item) = self.get_mut("mute-tab") {
            item.set_checked(muted);

            if muted {
                item.label = "Unmute Tab".to_string();
            } else {
                item.label = "Mute Tab".to_string();
            }
        }
    }

    pub fn set_close_enabled(&mut self, enabled: bool) {
        if let Some(item) = self.get_mut("close-tab") {
            item.set_enabled(enabled);
        }
    }

    pub fn activate(&self, item_id: &str) -> Option<&TabContextAction> {
        let item = self.get(item_id)?;

        if !item.can_activate() {
            return None;
        }

        Some(item.action())
    }

    pub fn reset(&mut self) {
        self.close();

        for item in &mut self.items {
            item.set_enabled(true);
            item.set_visible(true);
            item.set_checked(false);
        }
    }
}
