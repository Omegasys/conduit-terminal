use super::{
    actions::MenuAction,
    item::MenuItem,
    menu::Menu,
};

#[derive(Debug, Clone)]
pub struct ToolsMenu {
    menu: Menu,
}

impl Default for ToolsMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolsMenu {
    pub fn new() -> Self {
        let mut menu = Menu::new("tools", "Tools");

        menu.add_item(
            MenuItem::new(
                "settings",
                "Settings…",
                MenuAction::OpenSettings,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "profiles",
                "Profiles…",
                MenuAction::OpenProfiles,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "themes",
                "Themes…",
                MenuAction::OpenThemes,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "keybindings",
                "Keyboard Shortcuts…",
                MenuAction::OpenKeybindings,
            ),
        );

        menu.add_item(MenuItem::separator("separator-1"));

        menu.add_item(
            MenuItem::new(
                "reload-configuration",
                "Reload Configuration",
                MenuAction::ReloadConfiguration,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "reload-resources",
                "Reload Resources",
                MenuAction::ReloadResources,
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

    pub fn set_configuration_actions_enabled(&mut self, enabled: bool) {
        for id in [
            "settings",
            "profiles",
            "themes",
            "keybindings",
            "reload-configuration",
            "reload-resources",
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
