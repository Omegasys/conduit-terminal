//! Pane layout tree.
//!
//! Layouts are represented as a tree rather than a flat list. This makes
//! nested splits possible without coupling the pane manager to a GUI
//! framework.

use super::{
    pane::PaneId,
    split::{
        SplitDirection,
        SplitRatio,
    },
};

/// A node in the pane layout tree.
#[derive(Debug, Clone)]
pub enum LayoutNode {
    Pane(PaneId),

    Split {
        direction: SplitDirection,
        ratio: SplitRatio,
        first: Box<LayoutNode>,
        second: Box<LayoutNode>,
    },
}

impl LayoutNode {
    pub fn pane(
        id: PaneId,
    ) -> Self {
        Self::Pane(id)
    }

    pub fn split(
        direction: SplitDirection,
        ratio: SplitRatio,
        first: LayoutNode,
        second: LayoutNode,
    ) -> Self {
        Self::Split {
            direction,
            ratio,
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    pub fn contains(
        &self,
        pane_id: PaneId,
    ) -> bool {
        match self {
            Self::Pane(id) => {
                *id == pane_id
            }

            Self::Split {
                first,
                second,
                ..
            } => {
                first.contains(pane_id)
                    || second.contains(pane_id)
            }
        }
    }

    pub fn pane_ids(
        &self,
        output: &mut Vec<PaneId>,
    ) {
        match self {
            Self::Pane(id) => {
                output.push(*id);
            }

            Self::Split {
                first,
                second,
                ..
            } => {
                first.pane_ids(output);
                second.pane_ids(output);
            }
        }
    }

    pub fn pane_count(&self) -> usize {
        match self {
            Self::Pane(_) => 1,

            Self::Split {
                first,
                second,
                ..
            } => {
                first.pane_count()
                    + second.pane_count()
            }
        }
    }

    pub fn split_count(&self) -> usize {
        match self {
            Self::Pane(_) => 0,

            Self::Split {
                first,
                second,
                ..
            } => {
                1
                    + first.split_count()
                    + second.split_count()
            }
        }
    }

    /// Splits the specified pane.
    ///
    /// The existing pane becomes the first child and the new pane becomes
    /// the second child.
    pub fn split_pane(
        &mut self,
        target: PaneId,
        new_pane: PaneId,
        direction: SplitDirection,
        ratio: SplitRatio,
    ) -> bool {
        match self {
            Self::Pane(id) if *id == target => {
                let original =
                    Self::Pane(*id);

                let replacement =
                    Self::split(
                        direction,
                        ratio,
                        original,
                        Self::Pane(new_pane),
                    );

                *self = replacement;

                true
            }

            Self::Pane(_) => false,

            Self::Split {
                first,
                second,
                ..
            } => {
                first.split_pane(
                    target,
                    new_pane,
                    direction,
                    ratio,
                ) || second.split_pane(
                    target,
                    new_pane,
                    direction,
                    ratio,
                )
            }
        }
    }

    /// Removes a pane and collapses its parent split.
    pub fn remove_pane(
        &mut self,
        target: PaneId,
    ) -> bool {
        match self {
            Self::Pane(_) => false,

            Self::Split {
                first,
                second,
                ..
            } => {
                if matches!(
                    first.as_ref(),
                    Self::Pane(id)
                    if *id == target
                ) {
                    *self =
                        (**second).clone();

                    return true;
                }

                if matches!(
                    second.as_ref(),
                    Self::Pane(id)
                    if *id == target
                ) {
                    *self =
                        (**first).clone();

                    return true;
                }

                if first.remove_pane(target) {
                    return true;
                }

                second.remove_pane(target)
            }
        }
    }

    /// Changes the ratio of the split containing the target pane.
    pub fn resize_for_pane(
        &mut self,
        target: PaneId,
        ratio: SplitRatio,
    ) -> bool {
        match self {
            Self::Pane(_) => false,

            Self::Split {
                ratio: current,
                first,
                second,
                ..
            } => {
                if first.contains(target) {
                    *current = ratio;
                    return true;
                }

                if second.contains(target) {
                    *current = ratio.inverse();
                    return true;
                }

                first.resize_for_pane(
                    target,
                    ratio,
                ) || second.resize_for_pane(
                    target,
                    ratio,
                )
            }
        }
    }

    /// Returns the first pane in layout order.
    pub fn first_pane(
        &self,
    ) -> Option<PaneId> {
        match self {
            Self::Pane(id) => Some(*id),

            Self::Split {
                first,
                ..
            } => first.first_pane(),
        }
    }
}

/// Complete layout state for a tab.
#[derive(Debug, Clone)]
pub struct PaneLayout {
    root: Option<LayoutNode>,
}

impl PaneLayout {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn from_pane(
        pane_id: PaneId,
    ) -> Self {
        Self {
            root: Some(
                LayoutNode::Pane(pane_id),
            ),
        }
    }

    pub fn root(
        &self,
    ) -> Option<&LayoutNode> {
        self.root.as_ref()
    }

    pub fn root_mut(
        &mut self,
    ) -> Option<&mut LayoutNode> {
        self.root.as_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn pane_count(&self) -> usize {
        self.root
            .as_ref()
            .map(
                LayoutNode::pane_count
            )
            .unwrap_or(0)
    }

    pub fn pane_ids(&self) -> Vec<PaneId> {
        let mut ids =
            Vec::new();

        if let Some(root) =
            &self.root
        {
            root.pane_ids(&mut ids);
        }

        ids
    }

    pub fn contains(
        &self,
        pane_id: PaneId,
    ) -> bool {
        self.root
            .as_ref()
            .map(
                |root| root.contains(pane_id)
            )
            .unwrap_or(false)
    }

    pub fn split(
        &mut self,
        target: PaneId,
        new_pane: PaneId,
        direction: SplitDirection,
        ratio: SplitRatio,
    ) -> bool {
        let Some(root) =
            self.root.as_mut()
        else {
            return false;
        };

        root.split_pane(
            target,
            new_pane,
            direction,
            ratio,
        )
    }

    pub fn remove(
        &mut self,
        pane_id: PaneId,
    ) -> bool {
        let Some(root) =
            self.root.as_mut()
        else {
            return false;
        };

        if matches!(
            root,
            LayoutNode::Pane(id)
            if *id == pane_id
        ) {
            self.root = None;
            return true;
        }

        root.remove_pane(pane_id)
    }

    pub fn resize(
        &mut self,
        pane_id: PaneId,
        ratio: SplitRatio,
    ) -> bool {
        self.root
            .as_mut()
            .map(
                |root| {
                    root.resize_for_pane(
                        pane_id,
                        ratio,
                    )
                }
            )
            .unwrap_or(false)
    }

    pub fn first_pane(
        &self,
    ) -> Option<PaneId> {
        self.root
            .as_ref()
            .and_then(
                LayoutNode::first_pane
            )
    }
}

impl Default for PaneLayout {
    fn default() -> Self {
        Self::new()
    }
}
