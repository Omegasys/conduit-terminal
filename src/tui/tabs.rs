use crate::tabs::{Tab, TabId, TabState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabDisplayState {
    Normal,
    Active,
    Pinned,
    Muted,
    Modified,
    Closed,
}

impl TabDisplayState {
    pub fn from_tab(tab: &Tab) -> Self {
        if tab.state() == TabState::Closed {
            return Self::Closed;
        }

        if tab.pinned() {
            return Self::Pinned;
        }

        if tab.muted() {
            return Self::Muted;
        }

        if tab.dirty() {
            return Self::Modified;
        }

        if tab.state() == TabState::Active {
            return Self::Active;
        }

        Self::Normal
    }
}

pub struct TabDisplay {
    tab: Tab,
    display_state: TabDisplayState,
    visible: bool,
    index: usize,
}

impl TabDisplay {
    pub fn new(tab: Tab, index: usize) -> Self {
        let display_state = TabDisplayState::from_tab(&tab);

        Self {
            tab,
            display_state,
            visible: true,
            index,
        }
    }

    pub fn tab(&self) -> &Tab {
        &self.tab
    }

    pub fn tab_mut(&mut self) -> &mut Tab {
        &mut self.tab
    }

    pub fn id(&self) -> TabId {
        self.tab.id()
    }

    pub fn title(&self) -> &str {
        self.tab.title()
    }

    pub fn display_state(&self) -> TabDisplayState {
        self.display_state
    }

    pub fn set_display_state(&mut self, state: TabDisplayState) {
        self.display_state = state;
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn refresh_state(&mut self) {
        self.display_state = TabDisplayState::from_tab(&self.tab);
    }
}

pub struct TuiTabView {
    tabs: Vec<TabDisplay>,
    active: Option<TabId>,
    show_icons: bool,
    show_close_buttons: bool,
    compact: bool,
}

impl Default for TuiTabView {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiTabView {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active: None,
            show_icons: true,
            show_close_buttons: true,
            compact: false,
        }
    }

    pub fn add(&mut self, tab: Tab) -> TabId {
        let id = tab.id();
        let index = self.tabs.len();

        self.tabs.push(TabDisplay::new(tab, index));

        if self.active.is_none() {
            self.active = Some(id);
        }

        self.refresh_indices();
        id
    }

    pub fn remove(&mut self, id: TabId) -> Option<TabDisplay> {
        let index = self.tabs.iter().position(|tab| tab.id() == id)?;
        let removed = self.tabs.remove(index);

        if self.active == Some(id) {
            self.active = self.tabs.first().map(|tab| tab.id());
        }

        self.refresh_indices();
        removed
    }

    pub fn get(&self, id: TabId) -> Option<&TabDisplay> {
        self.tabs.iter().find(|tab| tab.id() == id)
    }

    pub fn get_mut(&mut self, id: TabId) -> Option<&mut TabDisplay> {
        self.tabs.iter_mut().find(|tab| tab.id() == id)
    }

    pub fn activate(&mut self, id: TabId) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        self.active = Some(id);

        for tab in &mut self.tabs {
            tab.set_display_state(if tab.id() == id {
                TabDisplayState::Active
            } else {
                TabDisplayState::Normal
            });
        }

        true
    }

    pub fn active(&self) -> Option<&TabDisplay> {
        self.active.and_then(|id| self.get(id))
    }

    pub fn active_id(&self) -> Option<TabId> {
        self.active
    }

    pub fn tabs(&self) -> &[TabDisplay] {
        &self.tabs
    }

    pub fn tabs_mut(&mut self) -> &mut [TabDisplay] {
        &mut self.tabs
    }

    pub fn visible_tabs(&self) -> Vec<&TabDisplay> {
        self.tabs.iter().filter(|tab| tab.visible()).collect()
    }

    pub fn next(&mut self) {
        if self.tabs.is_empty() {
            return;
        }

        let index = self
            .active
            .and_then(|id| self.tabs.iter().position(|tab| tab.id() == id))
            .unwrap_or(0);

        let next = (index + 1) % self.tabs.len();
        let id = self.tabs[next].id();
        self.activate(id);
    }

    pub fn previous(&mut self) {
        if self.tabs.is_empty() {
            return;
        }

        let index = self
            .active
            .and_then(|id| self.tabs.iter().position(|tab| tab.id() == id))
            .unwrap_or(0);

        let previous = if index == 0 {
            self.tabs.len() - 1
        } else {
            index - 1
        };

        let id = self.tabs[previous].id();
        self.activate(id);
    }

    pub fn show_icons(&self) -> bool {
        self.show_icons
    }

    pub fn set_show_icons(&mut self, show: bool) {
        self.show_icons = show;
    }

    pub fn show_close_buttons(&self) -> bool {
        self.show_close_buttons
    }

    pub fn set_show_close_buttons(&mut self, show: bool) {
        self.show_close_buttons = show;
    }

    pub fn compact(&self) -> bool {
        self.compact
    }

    pub fn set_compact(&mut self, compact: bool) {
        self.compact = compact;
    }

    pub fn refresh(&mut self) {
        for tab in &mut self.tabs {
            tab.refresh_state();
        }
    }

    fn refresh_indices(&mut self) {
        for (index, tab) in self.tabs.iter_mut().enumerate() {
            tab.set_index(index);
        }
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    pub fn clear(&mut self) {
        self.tabs.clear();
        self.active = None;
    }
}

pub type TabList = TuiTabView;
