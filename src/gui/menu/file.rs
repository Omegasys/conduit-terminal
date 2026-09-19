use super::{
    actions::MenuAction,
    item::{MenuItem, MenuItemKind},
    menu::Menu,
};

#[derive(Debug, Clone)]
pub struct FileMenu {
    menu: Menu,
}

impl Default for FileMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl FileMenu {
    pub fn new() -> Self {
        let mut menu = Menu::new("file", "File");

        let mut new_window =
            MenuItem::new("new-window", "New Window", MenuAction::NewWindow);
        new_window.set_accelerator("Ctrl+Shift+N");
        menu.add_item(new_window);

        let mut new_tab =
            MenuItem::new("new-tab", "New Tab", MenuAction::NewTab);
        new_tab.set_accelerator("Ctrl+Shift+T");
        menu.add_item(new_tab);

        menu.add_item(MenuItem::separator("separator-1"));

        let mut open_file =
            MenuItem::new("open-file", "Open File…", MenuAction::OpenFile);
        open_file.set_accelerator("Ctrl+O");
        menu.add_item(open_file);

        let mut open_folder =
            MenuItem::new("open-folder", "Open Folder…", MenuAction::OpenFolder);
        open_folder.set_accelerator("Ctrl+Shift+O");
        menu.add_item(open_folder);

        menu.add_item(MenuItem::separator("separator-2"));

        menu.add_item(
            MenuItem::new(
                "save-session",
                "Save Session",
                MenuAction::SaveSession,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "restore-session",
                "Restore Session",
                MenuAction::RestoreSession,
            ),
        );

        menu.add_item(MenuItem::separator("separator-3"));

        let mut close_window =
            MenuItem::new("close-window", "Close Window", MenuAction::CloseWindow);
        close_window.set_accelerator("Ctrl+Shift+W");
        menu.add_item(close_window);

        let mut quit =
            MenuItem::new("quit", "Quit", MenuAction::Quit);
        quit.set_accelerator("Ctrl+Q");
        menu.add_item(quit);

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

    pub fn set_open_file_enabled(&mut self, enabled: bool) {
        if let Some(item) = self.menu.get_mut("open-file") {
            item.set_enabled(enabled);
        }
    }

    pub fn set_open_folder_enabled(&mut self, enabled: bool) {
        if let Some(item) = self.menu.get_mut("open-folder") {
            item.set_enabled(enabled);
        }
    }

    pub fn set_session_actions_enabled(&mut self, enabled: bool) {
        for id in ["save-session", "restore-session"] {
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

    pub fn visible_items(&self) -> impl Iterator<Item = &MenuItem> {
        self.menu.items().iter().filter(|item| item.is_visible())
    }

    pub fn actionable_items(&self) -> impl Iterator<Item = &MenuItem> {
        self.visible_items()
            .filter(|item| item.is_enabled())
            .filter(|item| !matches!(item.kind(), MenuItemKind::Separator))
    }
}
