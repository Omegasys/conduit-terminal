//! Tab ordering and drag/drop behavior.

use super::tab::TabId;

/// Desired location when moving a tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabDropPosition {
    Before,
    After,
    First,
    Last,
}

/// A tab reorder operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabReorder {
    pub tab_id: TabId,
    pub target: TabId,
    pub position: TabDropPosition,
}

impl TabReorder {
    pub fn before(
        tab_id: TabId,
        target: TabId,
    ) -> Self {
        Self {
            tab_id,
            target,
            position:
                TabDropPosition::Before,
        }
    }

    pub fn after(
        tab_id: TabId,
        target: TabId,
    ) -> Self {
        Self {
            tab_id,
            target,
            position:
                TabDropPosition::After,
        }
    }
}
