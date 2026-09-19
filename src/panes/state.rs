//! Pane state and restoration data.
//!
//! This module provides a serializable logical representation of panes.
//! PTY/process state is intentionally not serialized here.

use super::{
    pane::{
        Pane,
        PaneId,
        PaneOrientation,
    },
};

/// Serializable pane state.
#[derive(Debug, Clone)]
pub struct PaneState {
    pub id: Option<PaneId>,

    pub session_id: Option<String>,

    pub title: String,

    pub orientation: PaneOrientation,

    pub active: bool,
    pub focused: bool,
    pub zoomed: bool,
    pub visible: bool,

    pub size_ratio: f32,

    pub order: usize,
}

impl PaneState {
    pub fn from_pane(
        pane: &Pane,
        order: usize,
    ) -> Self {
        Self {
            id: Some(pane.id()),

            session_id:
                pane.session_id()
                    .map(ToOwned::to_owned),

            title:
                pane.title()
                    .to_string(),

            orientation:
                pane.orientation(),

            active:
                pane.active(),

            focused:
                pane.focused(),

            zoomed:
                pane.zoomed(),

            visible:
                pane.visible(),

            size_ratio:
                pane.size_ratio(),

            order,
        }
    }

    pub fn apply_to(
        &self,
        pane: &mut Pane,
    ) {
        pane.set_session_id(
            self.session_id.clone()
        );

        pane.set_title(
            self.title.clone()
        );

        pane.set_orientation(
            self.orientation
        );

        pane.set_active(
            self.active
        );

        pane.set_focused(
            self.focused
        );

        pane.set_zoomed(
            self.zoomed
        );

        pane.set_visible(
            self.visible
        );

        pane.set_size_ratio(
            self.size_ratio
        );
    }
}

/// Complete logical state for a pane collection.
#[derive(Debug, Clone, Default)]
pub struct PaneCollectionState {
    pub panes: Vec<PaneState>,

    pub active_pane: Option<PaneId>,
}

impl PaneCollectionState {
    pub fn from_manager(
        manager: &super::manager::PaneManager,
    ) -> Self {
        let panes =
            manager
                .panes()
                .iter()
                .enumerate()
                .map(
                    |(index, pane)| {
                        PaneState::from_pane(
                            pane,
                            index,
                        )
                    }
                )
                .collect();

        Self {
            panes,
            active_pane:
                manager.active_pane_id(),
        }
    }

    pub fn pane_count(&self) -> usize {
        self.panes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.panes.is_empty()
    }

    pub fn find(
        &self,
        id: PaneId,
    ) -> Option<&PaneState> {
        self.panes
            .iter()
            .find(
                |pane| {
                    pane.id == Some(id)
                }
            )
    }
}
