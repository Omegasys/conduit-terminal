use std::collections::HashMap;

use super::menu::Menu;

#[derive(Debug, Default)]
pub struct MenuBar {
    menus: Vec<Menu>,
    indices: HashMap<String, usize>,
    active_menu: Option<String>,
}

impl MenuBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_menu(&mut self, menu: Menu) {
        let id = menu.id().to_string();

        if let Some(index) = self.indices.get(&id).copied() {
            self.menus[index] = menu;
            return;
        }

        let index = self.menus.len();
        self.indices.insert(id, index);
        self.menus.push(menu);
    }

    pub fn insert_menu(&mut self, index: usize, menu: Menu) {
        let id = menu.id().to_string();

        if self.indices.contains_key(&id) {
            self.add_menu(menu);
            return;
        }

        let index = index.min(self.menus.len());
        self.menus.insert(index, menu);
        self.rebuild_indices();
    }

    pub fn remove_menu(&mut self, id: &str) -> Option<Menu> {
        let index = self.indices.get(id).copied()?;
        let menu = self.menus.remove(index);

        if self.active_menu.as_deref() == Some(id) {
            self.active_menu = None;
        }

        self.rebuild_indices();
        Some(menu)
    }

    pub fn get(&self, id: &str) -> Option<&Menu> {
        let index = self.indices.get(id)?;
        self.menus.get(*index)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Menu> {
        let index = self.indices.get(id)?;
        self.menus.get_mut(*index)
    }

    pub fn menus(&self) -> &[Menu] {
        &self.menus
    }

    pub fn menus_mut(&mut self) -> &mut [Menu] {
        &mut self.menus
    }

    pub fn open(&mut self, id: &str) -> bool {
        if self.get(id).is_some_and(Menu::can_open) {
            self.active_menu = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn close(&mut self) {
        self.active_menu = None;
    }

    pub fn active_menu(&self) -> Option<&Menu> {
        self.active_menu
            .as_deref()
            .and_then(|id| self.get(id))
    }

    pub fn active_menu_id(&self) -> Option<&str> {
        self.active_menu.as_deref()
    }

    pub fn next_menu(&mut self) -> Option<&Menu> {
        if self.menus.is_empty() {
            return None;
        }

        let current = self
            .active_menu
            .as_deref()
            .and_then(|id| self.indices.get(id).copied());

        let next = match current {
            Some(index) => (index + 1) % self.menus.len(),
            None => 0,
        };

        let id = self.menus[next].id().to_string();
        self.active_menu = Some(id);

        self.menus.get(next)
    }

    pub fn previous_menu(&mut self) -> Option<&Menu> {
        if self.menus.is_empty() {
            return None;
        }

        let current = self
            .active_menu
            .as_deref()
            .and_then(|id| self.indices.get(id).copied());

        let previous = match current {
            Some(0) => self.menus.len() - 1,
            Some(index) => index - 1,
            None => self.menus.len() - 1,
        };

        let id = self.menus[previous].id().to_string();
        self.active_menu = Some(id);

        self.menus.get(previous)
    }

    pub fn len(&self) -> usize {
        self.menus.len()
    }

    pub fn is_empty(&self) -> bool {
        self.menus.is_empty()
    }

    pub fn clear(&mut self) {
        self.menus.clear();
        self.indices.clear();
        self.active_menu = None;
    }

    fn rebuild_indices(&mut self) {
        self.indices.clear();

        for (index, menu) in self.menus.iter().enumerate() {
            self.indices.insert(menu.id().to_string(), index);
        }
    }
}
