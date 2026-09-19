use super::{
    actions::MenuAction,
    item::MenuItem,
    menu::Menu,
};

#[derive(Debug, Clone)]
pub struct PanesMenu {
    menu: Menu,
}

impl Default for PanesMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl PanesMenu {
    pub fn new() -> Self {
        let mut menu = Menu::new("panes", "Panes");

        menu.add_item(
            MenuItem::new(
                "new-pane",
                "New Pane",
                MenuAction::NewPane,
            ),
        );

        menu.add_item(MenuItem::separator("separator-1"));

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

        let mut next =
            MenuItem::new("focus-next", "Focus Next Pane", MenuAction::FocusNextPane);
        next.set_accelerator("Ctrl+Alt+Right");
        menu.add_item(next);

        let mut previous = MenuItem::new(
            "focus-previous",
            "Focus Previous Pane",
            MenuAction::FocusPreviousPane,
        );
        previous.set_accelerator("Ctrl+Alt+Left");
        menu.add_item(previous);

        menu.add_item(MenuItem::separator("separator-3"));

        menu.add_item(
            MenuItem::new(
                "zoom",
                "Zoom Active Pane",
                MenuAction::ZoomPane,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "swap",
                "Swap With Next Pane",
                MenuAction::SwapPane,
            ),
        );

        menu.add_item(
            MenuItem::new(
                "close",
                "Close Pane",
                MenuAction::ClosePane,
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

    pub fn set_enabled(&mut self, enabled: bool) {
        for item in self.menu.items_mut() {
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
