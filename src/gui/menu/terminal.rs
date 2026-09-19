use super::{
    actions::MenuAction,
    item::MenuItem,
    menu::Menu,
};

#[derive(Debug, Clone)]
pub struct TerminalMenu {
    menu: Menu,
}

impl Default for TerminalMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalMenu {
    pub fn new() -> Self {
        let mut menu = Menu::new("terminal", "Terminal");

        let mut new_terminal =
            MenuItem::new("new-terminal", "New Terminal", MenuAction::OpenTerminal);
        new_terminal.set_accelerator("Ctrl+Alt+T");
        menu.add_item(new_terminal);

        menu.add_item(MenuItem::separator("separator-1"));

        menu.add_item(
            MenuItem::new(
                "new-pane",
                "New Pane",
                MenuAction::NewPane,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "split-horizontal",
                "Split Horizontally",
                MenuAction::SplitHorizontal,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "split-vertical",
                "Split Vertically",
                MenuAction::SplitVertical,
            ),
        );

        menu.add_item(MenuItem::separator("separator-2"));

        menu.add_item(
            MenuItem::new(
                "clear-selection",
                "Clear Selection",
                MenuAction::ClearSelection,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "find",
                "Find…",
                MenuAction::Find,
            ),
        );

        menu.add_item(MenuItem::separator("separator-3"));

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

    pub fn set_terminal_creation_enabled(&mut self, enabled: bool) {
        if let Some(item) = self.menu.get_mut("new-terminal") {
            item.set_enabled(enabled);
        }
    }

    pub fn set_pane_actions_enabled(&mut self, enabled: bool) {
        for id in ["new-pane", "split-horizontal", "split-vertical"] {
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
