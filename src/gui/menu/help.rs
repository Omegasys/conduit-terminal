use super::{
    actions::MenuAction,
    item::MenuItem,
    menu::Menu,
};

#[derive(Debug, Clone)]
pub struct HelpMenu {
    menu: Menu,
}

impl Default for HelpMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl HelpMenu {
    pub fn new() -> Self {
        let mut menu = Menu::new("help", "Help");

        menu.add_item(
            MenuItem::new(
                "documentation",
                "Documentation",
                MenuAction::Documentation,
            ),
        );

        menu.add_item(MenuItem::separator("separator-1"));

        menu.add_item(
            MenuItem::new(
                "about",
                "About Conduit",
                MenuAction::About,
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

    pub fn set_documentation_enabled(&mut self, enabled: bool) {
        if let Some(item) = self.menu.get_mut("documentation") {
            item.set_enabled(enabled);
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
