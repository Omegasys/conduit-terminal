use super::{
    actions::MenuAction,
    item::MenuItem,
    menu::Menu,
};

#[derive(Debug, Clone)]
pub struct TabsMenu {
    menu: Menu,
}

impl Default for TabsMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl TabsMenu {
    pub fn new() -> Self {
        let mut menu = Menu::new("tabs", "Tabs");

        let mut new_tab =
            MenuItem::new("new-tab", "New Tab", MenuAction::NewTab);
        new_tab.set_accelerator("Ctrl+T");
        menu.add_item(new_tab);

        let mut close_tab =
            MenuItem::new("close-tab", "Close Tab", MenuAction::CloseTab);
        close_tab.set_accelerator("Ctrl+W");
        menu.add_item(close_tab);

        menu.add_item(
            MenuItem::new(
                "reopen-tab",
                "Reopen Closed Tab",
                MenuAction::ReopenTab,
            ),
        );

        menu.add_item(MenuItem::separator("separator-1"));

        let mut next_tab =
            MenuItem::new("next-tab", "Next Tab", MenuAction::NextTab);
        next_tab.set_accelerator("Ctrl+Tab");
        menu.add_item(next_tab);

        let mut previous_tab =
            MenuItem::new("previous-tab", "Previous Tab", MenuAction::PreviousTab);
        previous_tab.set_accelerator("Ctrl+Shift+Tab");
        menu.add_item(previous_tab);

        menu.add_item(MenuItem::separator("separator-2"));

        menu.add_item(
            MenuItem::new(
                "new-window",
                "Move Tab to New Window",
                MenuAction::NewWindow,
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

    pub fn set_tab_actions_enabled(&mut self, enabled: bool) {
        for id in [
            "new-tab",
            "close-tab",
            "reopen-tab",
            "next-tab",
            "previous-tab",
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
