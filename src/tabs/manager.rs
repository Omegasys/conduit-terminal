//! Tab manager.
//!
//! Owns the logical ordering and selection of tabs belonging to a
//! window.

use super::{
    reordering::{
        TabDropPosition,
        TabReorder,
    },
    restoration::TabRestoreData,
    state::TabState,
    tab::{
        Tab,
        TabId,
    },
};

/// Errors produced by tab management operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabManagerError {
    TabNotFound(TabId),
    CannotCloseLastTab,
    InvalidReorder,
}

impl std::fmt::Display for TabManagerError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::TabNotFound(id) => {
                write!(
                    formatter,
                    "tab not found: {}",
                    id
                )
            }

            Self::CannotCloseLastTab => {
                write!(
                    formatter,
                    "cannot close the last tab"
                )
            }

            Self::InvalidReorder => {
                write!(
                    formatter,
                    "invalid tab reorder operation"
                )
            }
        }
    }
}

impl std::error::Error
    for TabManagerError {}

/// Manages tabs for a window.
#[derive(Debug)]
pub struct TabManager {
    tabs: Vec<Tab>,
    active_tab: Option<TabId>,

    window_id: Option<u64>,
}

impl TabManager {
    pub fn new(
        window_id: Option<u64>,
    ) -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: None,
            window_id,
        }
    }

    pub fn window_id(&self) -> Option<u64> {
        self.window_id
    }

    pub fn set_window_id(
        &mut self,
        window_id: Option<u64>,
    ) {
        self.window_id = window_id;

        for tab in &mut self.tabs {
            tab.set_window_id(
                window_id,
            );
        }
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn active_tab_id(&self) -> Option<TabId> {
        self.active_tab
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.active_tab
            .and_then(|id| {
                self.get(id)
            })
    }

    pub fn active_tab_mut(
        &mut self,
    ) -> Option<&mut Tab> {
        let id = self.active_tab?;

        self.get_mut(id)
    }

    pub fn get(
        &self,
        id: TabId,
    ) -> Option<&Tab> {
        self.tabs
            .iter()
            .find(|tab| tab.id() == id)
    }

    pub fn get_mut(
        &mut self,
        id: TabId,
    ) -> Option<&mut Tab> {
        self.tabs
            .iter_mut()
            .find(|tab| tab.id() == id)
    }

    /// Creates a new tab and makes it active.
    pub fn create(
        &mut self,
        title: impl Into<String>,
    ) -> TabId {
        let mut tab =
            Tab::new(title);

        tab.set_window_id(
            self.window_id,
        );

        tab.set_state(
            TabState::Active,
        );

        let id = tab.id();

        self.deactivate_all();

        self.tabs.push(tab);
        self.active_tab = Some(id);

        id
    }

    /// Adds an existing tab.
    pub fn add(
        &mut self,
        mut tab: Tab,
    ) -> TabId {
        tab.set_window_id(
            self.window_id,
        );

        let id = tab.id();

        if self.tabs.is_empty() {
            self.active_tab =
                Some(id);

            tab.set_state(
                TabState::Active,
            );
        }

        self.tabs.push(tab);

        id
    }

    /// Activates a tab.
    pub fn activate(
        &mut self,
        id: TabId,
    ) -> Result<(), TabManagerError> {
        if !self.tabs.iter().any(
            |tab| tab.id() == id
        ) {
            return Err(
                TabManagerError::TabNotFound(
                    id
                )
            );
        }

        self.deactivate_all();

        if let Some(tab) =
            self.get_mut(id)
        {
            tab.set_state(
                TabState::Active,
            );

            tab.clear_activity();
        }

        self.active_tab =
            Some(id);

        Ok(())
    }

    /// Closes a tab.
    pub fn close(
        &mut self,
        id: TabId,
    ) -> Result<(), TabManagerError> {
        let index = self
            .tabs
            .iter()
            .position(|tab| tab.id() == id)
            .ok_or(
                TabManagerError::TabNotFound(
                    id
                )
            )?;

        if self.tabs.len() <= 1 {
            return Err(
                TabManagerError::CannotCloseLastTab
            );
        }

        let was_active =
            self.active_tab == Some(id);

        self.tabs.remove(index);

        if was_active {
            let next_index =
                index.min(
                    self.tabs.len() - 1
                );

            let next_id =
                self.tabs[next_index]
                    .id();

            self.activate(next_id)?;
        }

        Ok(())
    }

    /// Reorders a tab.
    pub fn reorder(
        &mut self,
        operation: TabReorder,
    ) -> Result<(), TabManagerError> {
        let source_index =
            self.tabs
                .iter()
                .position(
                    |tab| {
                        tab.id()
                            == operation.tab_id
                    }
                )
                .ok_or(
                    TabManagerError::TabNotFound(
                        operation.tab_id
                    )
                )?;

        let target_index =
            self.tabs
                .iter()
                .position(
                    |tab| {
                        tab.id()
                            == operation.target
                    }
                )
                .ok_or(
                    TabManagerError::TabNotFound(
                        operation.target
                    )
                )?;

        if source_index == target_index {
            return Err(
                TabManagerError::InvalidReorder
            );
        }

        let tab =
            self.tabs.remove(
                source_index
            );

        let mut destination =
            target_index;

        if source_index
            < target_index
        {
            destination =
                destination
                    .saturating_sub(1);
        }

        match operation.position {
            TabDropPosition::Before => {}

            TabDropPosition::After => {
                destination =
                    destination
                        .saturating_add(1);
            }

            TabDropPosition::First => {
                destination = 0;
            }

            TabDropPosition::Last => {
                destination =
                    self.tabs.len();
            }
        }

        destination =
            destination.min(
                self.tabs.len()
            );

        self.tabs.insert(
            destination,
            tab,
        );

        Ok(())
    }

    /// Moves a tab directly to the first position.
    pub fn move_first(
        &mut self,
        id: TabId,
    ) -> Result<(), TabManagerError> {
        let index =
            self.index_of(id)?;

        let tab =
            self.tabs.remove(index);

        self.tabs.insert(
            0,
            tab,
        );

        Ok(())
    }

    /// Moves a tab directly to the last position.
    pub fn move_last(
        &mut self,
        id: TabId,
    ) -> Result<(), TabManagerError> {
        let index =
            self.index_of(id)?;

        let tab =
            self.tabs.remove(index);

        self.tabs.push(tab);

        Ok(())
    }

    /// Returns restoration data in current tab order.
    pub fn restoration_data(
        &self,
    ) -> Vec<TabRestoreData> {
        self.tabs
            .iter()
            .enumerate()
            .map(
                |(index, tab)| {
                    TabRestoreData::from_tab(
                        tab,
                        index,
                    )
                }
            )
            .collect()
    }

    /// Closes every tab.
    pub fn close_all(
        &mut self,
    ) {
        self.tabs.clear();
        self.active_tab = None;
    }

    fn index_of(
        &self,
        id: TabId,
    ) -> Result<usize, TabManagerError> {
        self.tabs
            .iter()
            .position(
                |tab| tab.id() == id
            )
            .ok_or(
                TabManagerError::TabNotFound(
                    id
                )
            )
    }

    fn deactivate_all(&mut self) {
        for tab in &mut self.tabs {
            if tab.state()
                != TabState::Closed
            {
                tab.set_state(
                    TabState::Inactive,
                );
            }
        }
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new(None)
    }
}
