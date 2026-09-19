//! Pane resizing.
//!
//! Provides logical resize operations without depending on a particular
//! GUI, TUI, or rendering backend.

use super::{
    manager::{
        PaneManager,
        PaneManagerError,
    },
    pane::PaneId,
    split::SplitRatio,
};

/// Direction in which a resize request is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeDirection {
    Increase,
    Decrease,
}

/// A pane resize request.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaneResize {
    pub pane_id: PaneId,
    pub direction: ResizeDirection,
    pub amount: f32,
}

impl PaneResize {
    pub fn increase(
        pane_id: PaneId,
        amount: f32,
    ) -> Self {
        Self {
            pane_id,
            direction: ResizeDirection::Increase,
            amount: amount.abs(),
        }
    }

    pub fn decrease(
        pane_id: PaneId,
        amount: f32,
    ) -> Self {
        Self {
            pane_id,
            direction: ResizeDirection::Decrease,
            amount: amount.abs(),
        }
    }
}

/// Default amount used by keyboard-based resize operations.
pub const DEFAULT_RESIZE_STEP: f32 = 0.05;

/// Applies a relative resize to a pane's containing split.
pub fn apply_resize(
    manager: &mut PaneManager,
    request: PaneResize,
) -> Result<(), PaneManagerError> {
    let current_ratio = manager
        .layout()
        .root()
        .and_then(|root| {
            find_ratio_for_pane(
                root,
                request.pane_id,
            )
        })
        .unwrap_or(0.5);

    let delta = match request.direction {
        ResizeDirection::Increase => request.amount,
        ResizeDirection::Decrease => -request.amount,
    };

    let ratio =
        SplitRatio::new(
            current_ratio + delta,
        );

    manager.resize(
        request.pane_id,
        ratio,
    )
}

/// Increases the pane's size by the default keyboard step.
pub fn increase(
    manager: &mut PaneManager,
    pane_id: PaneId,
) -> Result<(), PaneManagerError> {
    apply_resize(
        manager,
        PaneResize::increase(
            pane_id,
            DEFAULT_RESIZE_STEP,
        ),
    )
}

/// Decreases the pane's size by the default keyboard step.
pub fn decrease(
    manager: &mut PaneManager,
    pane_id: PaneId,
) -> Result<(), PaneManagerError> {
    apply_resize(
        manager,
        PaneResize::decrease(
            pane_id,
            DEFAULT_RESIZE_STEP,
        ),
    )
}

/// Sets a pane's containing split to an exact ratio.
pub fn set_ratio(
    manager: &mut PaneManager,
    pane_id: PaneId,
    ratio: f32,
) -> Result<(), PaneManagerError> {
    manager.resize(
        pane_id,
        SplitRatio::new(ratio),
    )
}

fn find_ratio_for_pane(
    node: &super::layout::LayoutNode,
    pane_id: PaneId,
) -> Option<f32> {
    match node {
        super::layout::LayoutNode::Pane(_) => None,

        super::layout::LayoutNode::Split {
            ratio,
            first,
            second,
            ..
        } => {
            if first.contains(pane_id) {
                Some(ratio.value())
            } else if second.contains(pane_id) {
                Some(ratio.value())
            } else {
                find_ratio_for_pane(
                    first,
                    pane_id,
                )
                .or_else(|| {
                    find_ratio_for_pane(
                        second,
                        pane_id,
                    )
                })
            }
        }
    }
}
