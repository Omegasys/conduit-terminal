use super::{
    actions::MenuAction,
    item::MenuItem,
    menu::Menu,
};

#[derive(Debug, Clone)]
pub struct SessionMenu {
    menu: Menu,
}

impl Default for SessionMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionMenu {
    pub fn new() -> Self {
        let mut menu = Menu::new("session", "Session");

        menu.add_item(
            MenuItem::new(
                "save",
                "Save Session",
                MenuAction::SaveSession,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "restore",
                "Restore Session",
                MenuAction::RestoreSession,
            ),
        );

        menu.add_item(MenuItem::separator("separator-1"));

        menu.add_item(
            MenuItem::new(
                "new-workspace",
                "New Workspace",
                MenuAction::NewWorkspace,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "close-workspace",
                "Close Workspace",
                MenuAction::CloseWorkspace,
            ),
        );

        menu.add_item(MenuItem::separator("separator-2"));

        menu.add_item(
            MenuItem::new(
                "next-workspace",
                "Next Workspace",
                MenuAction::NextWorkspace,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "previous-workspace",
                "Previous Workspace",
                MenuAction::PreviousWorkspace,
            ),
        );

        Self { menu }
    }

    pub fn menu(&self) -> &Menu {
        &self.menu
    }

    pub fn menu_mut(&mut self) -> &mut Menu {
        &mut self.menu
    }

    pub fn into_menu(self) -> Menu {
        self.menu
    }

    pub fn set_session_enabled(&mut self, enabled: bool) {
        for id in ["save", "restore"] {
            if let Some(item) = self.menu.get_mut(id) {
                item.set_enabled(enabled);
            }
        }
    }

    pub fn set_workspace_actions_enabled(&mut self, enabled: bool) {
        for id in [
            "new-workspace",
            "close-workspace",
            "next-workspace",
            "previous-workspace",
        ] {
            if let Some(item) = self.menu.get_mut(id) {
                item.set_enabled(enabled);
            }
        }
    }

    pub fn activate(&self, item_id: &str) -> Option<&MenuAction> {
        let item = self.menu.get(item_id)?;

        if !item.can_activate() {
            return None;
        }

        item.action()
    }
}
