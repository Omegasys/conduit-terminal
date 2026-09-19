//! Pane zoom management.
//!
//! Zooming temporarily presents one pane as the primary visible pane
//! without destroying the underlying split layout.

use super::{
    manager::{
        PaneManager,
        PaneManagerError,
    },
    pane::PaneId,
};

/// State of pane zooming.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoomState {
    Normal,
    Zoomed(PaneId),
}

/// Manages zoom state independently from the pane layout.
#[derive(Debug, Clone, Copy, Default)]
pub struct PaneZoom {
    zoomed_pane: Option<PaneId>,
}

impl PaneZoom {
    pub fn new() -> Self {
        Self {
            zoomed_pane: None,
        }
    }

    pub fn state(
        &self,
    ) -> ZoomState {
        match self.zoomed_pane {
            Some(id) =>
                ZoomState::Zoomed(id),

            None =>
                ZoomState::Normal,
        }
    }

    pub fn zoomed_pane(
        &self,
    ) -> Option<PaneId> {
        self.zoomed_pane
    }

    pub fn is_zoomed(&self) -> bool {
        self.zoomed_pane.is_some()
    }

    /// Zooms the specified pane.
    pub fn zoom(
        &mut self,
        manager: &mut PaneManager,
        pane_id: PaneId,
    ) -> Result<(), PaneManagerError> {
        if manager.get(pane_id).is_none() {
            return Err(
                PaneManagerError::PaneNotFound(
                    pane_id
                )
            );
        }

        if let Some(old) =
            self.zoomed_pane
        {
            if old != pane_id {
                if let Some(pane) =
                    manager.get_mut(old)
                {
                    pane.set_zoomed(
                        false
                    );
                }
            }
        }

        if let Some(pane) =
            manager.get_mut(pane_id)
        {
            pane.set_zoomed(
                true
            );
        }

        self.zoomed_pane =
            Some(pane_id);

        manager.activate(
            pane_id
        )?;

        Ok(())
    }

    /// Exits zoom mode.
    pub fn unzoom(
        &mut self,
        manager: &mut PaneManager,
    ) -> Result<(), PaneManagerError> {
        let Some(pane_id) =
            self.zoomed_pane.take()
        else {
            return Ok(());
        };

        if let Some(pane) =
            manager.get_mut(pane_id)
        {
            pane.set_zoomed(
                false
            );
        }

        Ok(())
    }

    /// Toggles zoom for a pane.
    pub fn toggle(
        &mut self,
        manager: &mut PaneManager,
        pane_id: PaneId,
    ) -> Result<bool, PaneManagerError> {
        if self.zoomed_pane
            == Some(pane_id)
        {
            self.unzoom(manager)?;
            return Ok(false);
        }

        self.zoom(
            manager,
            pane_id,
        )?;

        Ok(true)
    }

    /// Automatically clears zoom if the zoomed pane no longer exists.
    pub fn validate(
        &mut self,
        manager: &mut PaneManager,
    ) {
        let Some(pane_id) =
            self.zoomed_pane
        else {
            return;
        };

        if manager.get(pane_id).is_none() {
            self.zoomed_pane =
                None;
        }
    }
}
