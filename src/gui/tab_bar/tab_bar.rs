use std::collections::HashMap;

use crate::tabs::TabId;

use super::{
    context_menu::{TabContextAction, TabContextMenu},
    controls::{TabBarControl, TabBarControlAction, TabBarControls},
    tab::TabBarTab,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabBarLayout {
    Horizontal,
    Compact,
    Expanded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabBarOverflowMode {
    None,
    Scroll,
    Menu,
    ScrollAndMenu,
}

#[derive(Debug)]
pub struct TabBar {
    id: String,
    tabs: Vec<TabBarTab>,
    indices: HashMap<TabId, usize>,
    active_tab: Option<TabId>,
    hovered_tab: Option<TabId>,

    layout: TabBarLayout,
    overflow_mode: TabBarOverflowMode,

    visible: bool,
    enabled: bool,
    movable: bool,
    reorderable: bool,

    scroll_offset: usize,

    controls: TabBarControls,
    context_menu: TabContextMenu,
}

impl TabBar {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            tabs: Vec::new(),
            indices: HashMap::new(),
            active_tab: None,
            hovered_tab: None,

            layout: TabBarLayout::Horizontal,
            overflow_mode: TabBarOverflowMode::ScrollAndMenu,

            visible: true,
            enabled: true,
            movable: true,
            reorderable: true,

            scroll_offset: 0,

            controls: TabBarControls::new(),
            context_menu: TabContextMenu::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn tabs(&self) -> &[TabBarTab] {
        &self.tabs
    }

    pub fn tabs_mut(&mut self) -> &mut [TabBarTab] {
        &mut self.tabs
    }

    pub fn active_tab(&self) -> Option<TabId> {
        self.active_tab
    }

    pub fn hovered_tab(&self) -> Option<TabId> {
        self.hovered_tab
    }

    pub fn layout(&self) -> TabBarLayout {
        self.layout
    }

    pub fn overflow_mode(&self) -> TabBarOverflowMode {
        self.overflow_mode
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_movable(&self) -> bool {
        self.movable
    }

    pub fn is_reorderable(&self) -> bool {
        self.reorderable
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn controls(&self) -> &TabBarControls {
        &self.controls
    }

    pub fn controls_mut(&mut self) -> &mut TabBarControls {
        &mut self.controls
    }

    pub fn context_menu(&self) -> &TabContextMenu {
        &self.context_menu
    }

    pub fn context_menu_mut(&mut self) -> &mut TabContextMenu {
        &mut self.context_menu
    }

    pub fn add_tab(&mut self, tab: TabBarTab) {
        let id = tab.id();

        if self.indices.contains_key(&id) {
            return;
        }

        let index = self.tabs.len();

        self.tabs.push(tab);
        self.indices.insert(id, index);

        if self.active_tab.is_none() {
            self.activate_tab(id);
        }
    }

    pub fn insert_tab(&mut self, index: usize, tab: TabBarTab) {
        let id = tab.id();

        if self.indices.contains_key(&id) {
            return;
        }

        let index = index.min(self.tabs.len());

        self.tabs.insert(index, tab);
        self.rebuild_indices();

        if self.active_tab.is_none() {
            self.activate_tab(id);
        }
    }

    pub fn remove_tab(&mut self, id: TabId) -> Option<TabBarTab> {
        let index = self.indices.get(&id).copied()?;

        let tab = self.tabs.remove(index);

        if self.active_tab == Some(id) {
            self.active_tab = self.tabs.first().map(TabBarTab::id);

            if let Some(active) = self.active_tab {
                self.activate_tab(active);
            }
        }

        if self.hovered_tab == Some(id) {
            self.hovered_tab = None;
        }

        self.context_menu.close();
        self.rebuild_indices();

        Some(tab)
    }

    pub fn get_tab(&self, id: TabId) -> Option<&TabBarTab> {
        let index = self.indices.get(&id)?;
        self.tabs.get(*index)
    }

    pub fn get_tab_mut(&mut self, id: TabId) -> Option<&mut TabBarTab> {
        let index = self.indices.get(&id)?;
        self.tabs.get_mut(*index)
    }

    pub fn activate_tab(&mut self, id: TabId) -> bool {
        if !self.indices.contains_key(&id) {
            return false;
        }

        for tab in &mut self.tabs {
            if tab.id() == id {
                tab.activate();
            } else {
                tab.deactivate();
            }
        }

        self.active_tab = Some(id);
        self.ensure_tab_visible(id);

        true
    }

    pub fn close_tab(&mut self, id: TabId) -> Option<TabBarTab> {
        if let Some(tab) = self.get_tab_mut(id) {
            tab.begin_close();
        }

        self.remove_tab(id)
    }

    pub fn set_hovered(&mut self, id: Option<TabId>) {
        if let Some(previous) = self.hovered_tab {
            if previous != id {
                if let Some(tab) = self.get_tab_mut(previous) {
                    tab.unhover();
                }
            }
        }

        self.hovered_tab = id;

        if let Some(current) = id {
            if let Some(tab) = self.get_tab_mut(current) {
                tab.hover();
            }
        }
    }

    pub fn reorder_tab(&mut self, id: TabId, new_index: usize) -> bool {
        if !self.reorderable {
            return false;
        }

        let Some(old_index) = self.indices.get(&id).copied() else {
            return false;
        };

        let tab = self.tabs.remove(old_index);
        let new_index = new_index.min(self.tabs.len());

        self.tabs.insert(new_index, tab);
        self.rebuild_indices();

        true
    }

    pub fn move_tab_left(&mut self, id: TabId) -> bool {
        let Some(index) = self.indices.get(&id).copied() else {
            return false;
        };

        if index == 0 {
            return false;
        }

        self.reorder_tab(id, index - 1)
    }

    pub fn move_tab_right(&mut self, id: TabId) -> bool {
        let Some(index) = self.indices.get(&id).copied() else {
            return false;
        };

        if index + 1 >= self.tabs.len() {
            return false;
        }

        self.reorder_tab(id, index + 1)
    }

    pub fn move_tab_first(&mut self, id: TabId) -> bool {
        self.reorder_tab(id, 0)
    }

    pub fn move_tab_last(&mut self, id: TabId) -> bool {
        if self.tabs.is_empty() {
            return false;
        }

        self.reorder_tab(id, self.tabs.len() - 1)
    }

    pub fn pin_tab(&mut self, id: TabId, pinned: bool) -> bool {
        let Some(tab) = self.get_tab_mut(id) else {
            return false;
        };

        tab.set_pinned(pinned);
        true
    }

    pub fn mute_tab(&mut self, id: TabId, muted: bool) -> bool {
        let Some(tab) = self.get_tab_mut(id) else {
            return false;
        };

        tab.set_muted(muted);
        true
    }

    pub fn set_layout(&mut self, layout: TabBarLayout) {
        self.layout = layout;
    }

    pub fn set_overflow_mode(&mut self, mode: TabBarOverflowMode) {
        self.overflow_mode = mode;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if enabled {
            self.controls.enable_all();
        } else {
            self.controls.disable_all();
        }

        for tab in &mut self.tabs {
            tab.set_enabled(enabled);
        }
    }

    pub fn set_movable(&mut self, movable: bool) {
        self.movable = movable;
    }

    pub fn set_reorderable(&mut self, reorderable: bool) {
        self.reorderable = reorderable;
    }

    pub fn scroll_left(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_right(&mut self) {
        if self.scroll_offset < self.tabs.len().saturating_sub(1) {
            self.scroll_offset += 1;
        }
    }

    pub fn scroll_to_start(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn scroll_to_end(&mut self) {
        self.scroll_offset = self.tabs.len().saturating_sub(1);
    }

    pub fn ensure_tab_visible(&mut self, id: TabId) {
        if let Some(index) = self.indices.get(&id).copied() {
            if index < self.scroll_offset {
                self.scroll_offset = index;
            }
        }
    }

    pub fn open_context_menu(&mut self, id: TabId) -> bool {
        let Some(tab) = self.get_tab(id) else {
            return false;
        };

        self.context_menu.set_pin_state(tab.is_pinned());
        self.context_menu.set_mute_state(tab.is_muted());
        self.context_menu.open_for(id);

        true
    }

    pub fn close_context_menu(&mut self) {
        self.context_menu.close();
    }

    pub fn execute_context_action(
        &mut self,
        action: &TabContextAction,
    ) -> bool {
        let Some(id) = self.context_menu.tab_id() else {
            return false;
        };

        match action {
            TabContextAction::CloseTab => {
                self.close_tab(id);
            }

            TabContextAction::PinTab => {
                self.pin_tab(id, true);
            }

            TabContextAction::UnpinTab => {
                self.pin_tab(id, false);
            }

            TabContextAction::MuteTab => {
                self.mute_tab(id, true);
            }

            TabContextAction::UnmuteTab => {
                self.mute_tab(id, false);
            }

            TabContextAction::MoveToPreviousPosition => {
                self.move_tab_left(id);
            }

            TabContextAction::MoveToNextPosition => {
                self.move_tab_right(id);
            }

            TabContextAction::MoveToFirstPosition => {
                self.move_tab_first(id);
            }

            TabContextAction::MoveToLastPosition => {
                self.move_tab_last(id);
            }

            _ => {
                return false;
            }
        }

        self.context_menu.close();
        true
    }

    pub fn activate_control(
        &mut self,
        control: TabBarControl,
    ) -> Option<TabBarControlAction> {
        let state = self.controls.get(control)?;

        if !state.is_visible() || !state.is_enabled() {
            return None;
        }

        let action = state.action();

        match action {
            TabBarControlAction::ScrollLeft => self.scroll_left(),
            TabBarControlAction::ScrollRight => self.scroll_right(),
            TabBarControlAction::OpenOverflow => {
                // The rendering layer can display the overflow menu.
            }
            TabBarControlAction::NewTab
            | TabBarControlAction::OpenTabList => {}
        }

        Some(action)
    }

    pub fn visible_tabs(&self) -> impl Iterator<Item = &TabBarTab> {
        self.tabs.iter().filter(|tab| tab.is_visible())
    }

    pub fn pinned_tabs(&self) -> impl Iterator<Item = &TabBarTab> {
        self.visible_tabs().filter(|tab| tab.is_pinned())
    }

    pub fn unpinned_tabs(&self) -> impl Iterator<Item = &TabBarTab> {
        self.visible_tabs().filter(|tab| !tab.is_pinned())
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    pub fn clear(&mut self) {
        self.tabs.clear();
        self.indices.clear();
        self.active_tab = None;
        self.hovered_tab = None;
        self.scroll_offset = 0;
        self.context_menu.close();
    }

    fn rebuild_indices(&mut self) {
        self.indices.clear();

        for (index, tab) in self.tabs.iter().enumerate() {
            self.indices.insert(tab.id(), index);
        }
    }
}
