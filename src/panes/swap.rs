//! Pane swapping.
//!
//! Swapping changes pane positions in the logical layout while preserving
//! the panes themselves and their associated session references.

use super::{
    layout::LayoutNode,
    manager::{
        PaneManager,
        PaneManagerError,
    },
    pane::PaneId,
};

/// Swaps two panes in a pane manager.
pub fn swap(
    manager: &mut PaneManager,
    first: PaneId,
    second: PaneId,
) -> Result<(), PaneManagerError> {
    if manager.get(first).is_none() {
        return Err(
            PaneManagerError::PaneNotFound(
                first
            )
        );
    }

    if manager.get(second).is_none() {
        return Err(
            PaneManagerError::PaneNotFound(
                second
            )
        );
    }

    if first == second {
        return Ok(());
    }

    let first_index =
        manager
            .panes()
            .iter()
            .position(
                |pane| pane.id() == first
            )
            .ok_or(
                PaneManagerError::PaneNotFound(
                    first
                )
            )?;

    let second_index =
        manager
            .panes()
            .iter()
            .position(
                |pane| pane.id() == second
            )
            .ok_or(
                PaneManagerError::PaneNotFound(
                    second
                )
            )?;

    /*
     * PaneManager intentionally does not expose its internal vector
     * mutably. Swapping therefore operates through the layout tree.
     *
     * The actual pane objects remain unchanged, which means session
     * associations, titles, focus state, and other pane metadata stay
     * attached to their original pane IDs.
     */
    swap_layout_panes(
        manager.layout_mut(),
        first,
        second,
    );

    let _ = (
        first_index,
        second_index,
    );

    Ok(())
}

/// Swaps pane IDs throughout a layout tree.
fn swap_layout_panes(
    layout: &mut super::layout::PaneLayout,
    first: PaneId,
    second: PaneId,
) {
    if let Some(root) =
        layout.root_mut()
    {
        swap_layout_nodes(
            root,
            first,
            second,
        );
    }
}

fn swap_layout_nodes(
    node: &mut LayoutNode,
    first: PaneId,
    second: PaneId,
) {
    match node {
        LayoutNode::Pane(id) => {
            if *id == first {
                *id = second;
            } else if *id == second {
                *id = first;
            }
        }

        LayoutNode::Split {
            first: left,
            second: right,
            ..
        } => {
            swap_layout_nodes(
                left,
                first,
                second,
            );

            swap_layout_nodes(
                right,
                first,
                second,
            );
        }
    }
}

/// Swaps the active pane with another pane.
pub fn swap_with_active(
    manager: &mut PaneManager,
    other: PaneId,
) -> Result<(), PaneManagerError> {
    let active =
        manager
            .active_pane_id()
            .ok_or(
                PaneManagerError::InvalidLayout
            )?;

    swap(
        manager,
        active,
        other,
    )
}
