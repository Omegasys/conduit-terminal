//! Pane manager.
//!
//! The pane manager owns panes and coordinates them with the layout tree.
//! It deliberately does not own terminal sessions or PTYs.

use super::{
    layout::PaneLayout,
    pane::{
        Pane,
        PaneId,
    },
    split::{
        SplitDirection,
        SplitRatio,
    },
};

/// Errors produced by pane management operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaneManagerError {
    PaneNotFound(PaneId),
    CannotCloseLastPane,
    InvalidSplit,
    InvalidLayout,
}

impl std::fmt::Display for PaneManagerError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::PaneNotFound(id) => {
                write!(
                    formatter,
                    "pane not found: {}",
                    id
                )
            }

            Self::CannotCloseLastPane => {
                write!(
                    formatter,
                    "cannot close the last pane"
                )
            }

            Self::InvalidSplit => {
                write!(
                    formatter,
                    "invalid pane split"
                )
            }

            Self::InvalidLayout => {
                write!(
                    formatter,
                    "invalid pane layout"
                )
            }
        }
    }
}

impl std::error::Error
    for PaneManagerError {}

/// Manages panes belonging to one tab.
#[derive(Debug)]
pub struct PaneManager {
    panes: Vec<Pane>,

    layout: PaneLayout,

    active_pane: Option<PaneId>,

    tab_id: Option<u64>,
}

impl PaneManager {
    pub fn new(
        tab_id: Option<u64>,
    ) -> Self {
        Self {
            panes: Vec::new(),
            layout: PaneLayout::new(),
            active_pane: None,
            tab_id,
        }
    }

    pub fn tab_id(&self) -> Option<u64> {
        self.tab_id
    }

    pub fn set_tab_id(
        &mut self,
        tab_id: Option<u64>,
    ) {
        self.tab_id = tab_id;
    }

    pub fn panes(&self) -> &[Pane] {
        &self.panes
    }

    pub fn pane_count(&self) -> usize {
        self.panes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.panes.is_empty()
    }

    pub fn layout(&self) -> &PaneLayout {
        &self.layout
    }

    pub fn layout_mut(
        &mut self,
    ) -> &mut PaneLayout {
        &mut self.layout
    }

    pub fn active_pane_id(
        &self,
    ) -> Option<PaneId> {
        self.active_pane
    }

    pub fn active_pane(
        &self,
    ) -> Option<&Pane> {
        self.active_pane
            .and_then(
                |id| self.get(id)
            )
    }

    pub fn active_pane_mut(
        &mut self,
    ) -> Option<&mut Pane> {
        let id =
            self.active_pane?;

        self.get_mut(id)
    }

    pub fn get(
        &self,
        id: PaneId,
    ) -> Option<&Pane> {
        self.panes
            .iter()
            .find(
                |pane| pane.id() == id
            )
    }

    pub fn get_mut(
        &mut self,
        id: PaneId,
    ) -> Option<&mut Pane> {
        self.panes
            .iter_mut()
            .find(
                |pane| pane.id() == id
            )
    }

    /// Creates the initial pane for an empty tab.
    pub fn create(
        &mut self,
        title: impl Into<String>,
    ) -> PaneId {
        let pane =
            Pane::new(title);

        let id =
            pane.id();

        self.panes.push(pane);

        self.layout =
            PaneLayout::from_pane(id);

        self.activate(id)
            .expect(
                "newly created pane must exist"
            );

        id
    }

    /// Adds an existing pane.
    pub fn add(
        &mut self,
        pane: Pane,
    ) -> PaneId {
        let id =
            pane.id();

        if self.panes.is_empty() {
            self.layout =
                PaneLayout::from_pane(id);

            self.active_pane =
                Some(id);
        }

        self.panes.push(pane);

        id
    }

    /// Splits an existing pane and creates a new pane.
    pub fn split(
        &mut self,
        target: PaneId,
        direction: SplitDirection,
        title: impl Into<String>,
    ) -> Result<PaneId, PaneManagerError> {
        if self.get(target).is_none() {
            return Err(
                PaneManagerError::PaneNotFound(
                    target
                )
            );
        }

        if !self.layout.contains(target) {
            return Err(
                PaneManagerError::InvalidLayout
            );
        }

        let mut pane =
            Pane::new(title);

        pane.set_orientation(
            match direction {
                SplitDirection::Horizontal =>
                    super::pane::PaneOrientation::Horizontal,

                SplitDirection::Vertical =>
                    super::pane::PaneOrientation::Vertical,
            }
        );

        let new_id =
            pane.id();

        self.panes.push(pane);

        let success =
            self.layout.split(
                target,
                new_id,
                direction,
                SplitRatio::default(),
            );

        if !success {
            self.panes
                .retain(
                    |pane| {
                        pane.id()
                            != new_id
                    }
                );

            return Err(
                PaneManagerError::InvalidSplit
            );
        }

        self.activate(new_id)?;

        Ok(new_id)
    }

    /// Splits an existing pane using a custom ratio.
    pub fn split_with_ratio(
        &mut self,
        target: PaneId,
        direction: SplitDirection,
        ratio: SplitRatio,
        title: impl Into<String>,
    ) -> Result<PaneId, PaneManagerError> {
        if self.get(target).is_none() {
            return Err(
                PaneManagerError::PaneNotFound(
                    target
                )
            );
        }

        let mut pane =
            Pane::new(title);

        pane.set_orientation(
            match direction {
                SplitDirection::Horizontal =>
                    super::pane::PaneOrientation::Horizontal,

                SplitDirection::Vertical =>
                    super::pane::PaneOrientation::Vertical,
            }
        );

        let new_id =
            pane.id();

        self.panes.push(pane);

        if !self.layout.split(
            target,
            new_id,
            direction,
            ratio,
        ) {
            self.panes
                .retain(
                    |pane| {
                        pane.id()
                            != new_id
                    }
                );

            return Err(
                PaneManagerError::InvalidSplit
            );
        }

        self.activate(new_id)?;

        Ok(new_id)
    }

    /// Activates a pane and clears focus from all other panes.
    pub fn activate(
        &mut self,
        id: PaneId,
    ) -> Result<(), PaneManagerError> {
        if self.get(id).is_none() {
            return Err(
                PaneManagerError::PaneNotFound(
                    id
                )
            );
        }

        for pane in &mut self.panes {
            pane.deactivate();
        }

        if let Some(pane) =
            self.get_mut(id)
        {
            pane.activate();
        }

        self.active_pane =
            Some(id);

        Ok(())
    }

    /// Gives keyboard focus to a pane.
    pub fn focus(
        &mut self,
        id: PaneId,
    ) -> Result<(), PaneManagerError> {
        self.activate(id)
    }

    /// Closes a pane.
    ///
    /// If other panes remain, the nearest remaining pane is activated.
    pub fn close(
        &mut self,
        id: PaneId,
    ) -> Result<(), PaneManagerError> {
        if self.get(id).is_none() {
            return Err(
                PaneManagerError::PaneNotFound(
                    id
                )
            );
        }

        if self.panes.len() <= 1 {
            return Err(
                PaneManagerError::CannotCloseLastPane
            );
        }

        let was_active =
            self.active_pane == Some(id);

        let index =
            self.panes
                .iter()
                .position(
                    |pane| pane.id() == id
                )
                .expect(
                    "pane existence checked above"
                );

        if !self.layout.remove(id) {
            return Err(
                PaneManagerError::InvalidLayout
            );
        }

        self.panes.remove(index);

        if was_active {
            let next_index =
                index.min(
                    self.panes.len() - 1
                );

            let next_id =
                self.panes[next_index]
                    .id();

            self.activate(next_id)?;
        }

        Ok(())
    }

    /// Toggles zoom mode for the active pane.
    pub fn toggle_zoom(
        &mut self,
    ) -> Result<bool, PaneManagerError> {
        let id =
            self.active_pane
                .ok_or(
                    PaneManagerError::InvalidLayout
                )?;

        let pane =
            self.get_mut(id)
                .ok_or(
                    PaneManagerError::PaneNotFound(
                        id
                    )
                )?;

        let zoomed =
            !pane.zoomed();

        pane.set_zoomed(zoomed);

        Ok(zoomed)
    }

    /// Resizes the split associated with a pane.
    pub fn resize(
        &mut self,
        id: PaneId,
        ratio: SplitRatio,
    ) -> Result<(), PaneManagerError> {
        if self.get(id).is_none() {
            return Err(
                PaneManagerError::PaneNotFound(
                    id
                )
            );
        }

        if !self.layout.resize(
            id,
            ratio,
        ) {
            return Err(
                PaneManagerError::InvalidLayout
            );
        }

        Ok(())
    }

    /// Returns panes in visual layout order.
    pub fn ordered_panes(
        &self,
    ) -> Vec<&Pane> {
        self.layout
            .pane_ids()
            .into_iter()
            .filter_map(
                |id| self.get(id)
            )
            .collect()
    }

    /// Removes all panes and resets the layout.
    pub fn close_all(
        &mut self,
    ) {
        self.panes.clear();
        self.layout =
            PaneLayout::new();
        self.active_pane = None;
    }
}

impl Default for PaneManager {
    fn default() -> Self {
        Self::new(None)
    }
}
