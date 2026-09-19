use super::{
    actions::MenuAction,
    item::MenuItem,
    menu::Menu,
};

#[derive(Debug, Clone)]
pub struct EditMenu {
    menu: Menu,
}

impl Default for EditMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl EditMenu {
    pub fn new() -> Self {
        let mut menu = Menu::new("edit", "Edit");

        let mut undo = MenuItem::new("undo", "Undo", MenuAction::Undo);
        undo.set_accelerator("Ctrl+Z");
        menu.add_item(undo);

        let mut redo = MenuItem::new("redo", "Redo", MenuAction::Redo);
        redo.set_accelerator("Ctrl+Shift+Z");
        menu.add_item(redo);

        menu.add_item(MenuItem::separator("separator-1"));

        let mut cut = MenuItem::new("cut", "Cut", MenuAction::Cut);
        cut.set_accelerator("Ctrl+X");
        menu.add_item(cut);

        let mut copy = MenuItem::new("copy", "Copy", MenuAction::Copy);
        copy.set_accelerator("Ctrl+C");
        menu.add_item(copy);

        let mut paste = MenuItem::new("paste", "Paste", MenuAction::Paste);
        paste.set_accelerator("Ctrl+V");
        menu.add_item(paste);

        menu.add_item(MenuItem::separator("separator-2"));

        let mut select_all =
            MenuItem::new("select-all", "Select All", MenuAction::SelectAll);
        select_all.set_accelerator("Ctrl+A");
        menu.add_item(select_all);

        menu.add_item(
            MenuItem::new(
                "clear-selection",
                "Clear Selection",
                MenuAction::ClearSelection,
            ),
        );

        menu.add_item(MenuItem::separator("separator-3"));

        let mut find = MenuItem::new("find", "Find…", MenuAction::Find);
        find.set_accelerator("Ctrl+F");
        menu.add_item(find);

        let mut find_next =
            MenuItem::new("find-next", "Find Next", MenuAction::FindNext);
        find_next.set_accelerator("F3");
        menu.add_item(find_next);

        let mut find_previous =
            MenuItem::new("find-previous", "Find Previous", MenuAction::FindPrevious);
        find_previous.set_accelerator("Shift+F3");
        menu.add_item(find_previous);

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

    pub fn set_clipboard_actions_enabled(&mut self, enabled: bool) {
        for id in ["cut", "copy", "paste", "select-all"] {
            if let Some(item) = self.menu.get_mut(id) {
                item.set_enabled(enabled);
            }
        }
    }

    pub fn set_undo_enabled(&mut self, enabled: bool) {
        if let Some(item) = self.menu.get_mut("undo") {
            item.set_enabled(enabled);
        }
    }

    pub fn set_redo_enabled(&mut self, enabled: bool) {
        if let Some(item) = self.menu.get_mut("redo") {
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
