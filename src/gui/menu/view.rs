use super::{
    actions::MenuAction,
    item::MenuItem,
    menu::Menu,
};

#[derive(Debug, Clone)]
pub struct ViewMenu {
    menu: Menu,
}

impl Default for ViewMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewMenu {
    pub fn new() -> Self {
        let mut menu = Menu::new("view", "View");

        let mut sidebar =
            MenuItem::check("sidebar", "Sidebar", MenuAction::ToggleSidebar);
        sidebar.set_accelerator("Ctrl+Shift+B");
        menu.add_item(sidebar);

        let mut toolbar =
            MenuItem::check("toolbar", "Toolbar", MenuAction::ToggleToolbar);
        toolbar.set_checked(true);
        menu.add_item(toolbar);

        let mut status_bar =
            MenuItem::check("status-bar", "Status Bar", MenuAction::ToggleStatusBar);
        status_bar.set_checked(true);
        menu.add_item(status_bar);

        menu.add_item(MenuItem::separator("separator-1"));

        let mut command_palette = MenuItem::new(
            "command-palette",
            "Command Palette…",
            MenuAction::ToggleCommandPalette,
        );
        command_palette.set_accelerator("Ctrl+Shift+P");
        menu.add_item(command_palette);

        menu.add_item(MenuItem::separator("separator-2"));

        let mut fullscreen =
            MenuItem::check("fullscreen", "Fullscreen", MenuAction::ToggleFullscreen);
        fullscreen.set_accelerator("F11");
        menu.add_item(fullscreen);

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

    pub fn set_sidebar_visible(&mut self, visible: bool) {
        if let Some(item) = self.menu.get_mut("sidebar") {
            item.set_checked(visible);
        }
    }

    pub fn set_toolbar_visible(&mut self, visible: bool) {
        if let Some(item) = self.menu.get_mut("toolbar") {
            item.set_checked(visible);
        }
    }

    pub fn set_status_bar_visible(&mut self, visible: bool) {
        if let Some(item) = self.menu.get_mut("status-bar") {
            item.set_checked(visible);
        }
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        if let Some(item) = self.menu.get_mut("fullscreen") {
            item.set_checked(fullscreen);
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
