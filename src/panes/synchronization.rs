//! Pane synchronization.
//!
//! Synchronization allows related panes to share logical behavior such as
//! synchronized input or coordinated scrolling. Actual terminal I/O is
//! intentionally handled elsewhere.

use std::collections::{
    HashMap,
    HashSet,
};

use super::pane::PaneId;

/// Types of synchronization supported by Conduit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynchronizationMode {
    /// No synchronization.
    None,

    /// Input sent to one pane is logically replicated to its peers.
    Input,

    /// Scroll position is synchronized.
    Scroll,

    /// Both input and scroll position are synchronized.
    InputAndScroll,
}

impl SynchronizationMode {
    pub fn synchronizes_input(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Input
                | Self::InputAndScroll
        )
    }

    pub fn synchronizes_scroll(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Scroll
                | Self::InputAndScroll
        )
    }
}

/// A synchronization group.
#[derive(Debug, Clone)]
pub struct SynchronizationGroup {
    id: u64,
    mode: SynchronizationMode,
    panes: HashSet<PaneId>,
}

impl SynchronizationGroup {
    pub fn new(
        id: u64,
        mode: SynchronizationMode,
    ) -> Self {
        Self {
            id,
            mode,
            panes: HashSet::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn mode(
        &self,
    ) -> SynchronizationMode {
        self.mode
    }

    pub fn set_mode(
        &mut self,
        mode: SynchronizationMode,
    ) {
        self.mode = mode;
    }

    pub fn panes(
        &self,
    ) -> &HashSet<PaneId> {
        &self.panes
    }

    pub fn contains(
        &self,
        pane_id: PaneId,
    ) -> bool {
        self.panes.contains(
            &pane_id
        )
    }

    pub fn add(
        &mut self,
        pane_id: PaneId,
    ) -> bool {
        self.panes.insert(
            pane_id
        )
    }

    pub fn remove(
        &mut self,
        pane_id: PaneId,
    ) -> bool {
        self.panes.remove(
            &pane_id
        )
    }

    pub fn is_empty(&self) -> bool {
        self.panes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.panes.len()
    }
}

/// Manages synchronization groups.
#[derive(Debug, Default)]
pub struct PaneSynchronization {
    groups: HashMap<u64, SynchronizationGroup>,
    pane_groups: HashMap<PaneId, u64>,
    next_group_id: u64,
}

impl PaneSynchronization {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            pane_groups: HashMap::new(),
            next_group_id: 1,
        }
    }

    /// Creates a synchronization group.
    pub fn create_group(
        &mut self,
        mode: SynchronizationMode,
    ) -> u64 {
        let id =
            self.next_group_id;

        self.next_group_id =
            self.next_group_id
                .wrapping_add(1);

        self.groups.insert(
            id,
            SynchronizationGroup::new(
                id,
                mode,
            ),
        );

        id
    }

    pub fn group(
        &self,
        id: u64,
    ) -> Option<&SynchronizationGroup> {
        self.groups.get(&id)
    }

    pub fn group_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut SynchronizationGroup> {
        self.groups.get_mut(&id)
    }

    /// Adds a pane to a synchronization group.
    ///
    /// A pane can belong to only one synchronization group at a time.
    pub fn add_pane(
        &mut self,
        group_id: u64,
        pane_id: PaneId,
    ) -> bool {
        let Some(group) =
            self.groups.get_mut(
                &group_id
            )
        else {
            return false;
        };

        if let Some(old_group) =
            self.pane_groups.get(
                &pane_id
            ).copied()
        {
            if old_group == group_id {
                return false;
            }

            if let Some(old) =
                self.groups.get_mut(
                    &old_group
                )
            {
                old.remove(
                    pane_id
                );
            }
        }

        group.add(pane_id);

        self.pane_groups.insert(
            pane_id,
            group_id,
        );

        true
    }

    /// Removes a pane from its synchronization group.
    pub fn remove_pane(
        &mut self,
        pane_id: PaneId,
    ) -> bool {
        let Some(group_id) =
            self.pane_groups.remove(
                &pane_id
            )
        else {
            return false;
        };

        if let Some(group) =
            self.groups.get_mut(
                &group_id
            )
        {
            group.remove(
                pane_id
            );

            if group.is_empty() {
                self.groups.remove(
                    &group_id
                );
            }
        }

        true
    }

    /// Returns the synchronization group containing a pane.
    pub fn group_for_pane(
        &self,
        pane_id: PaneId,
    ) -> Option<&SynchronizationGroup> {
        let group_id =
            self.pane_groups.get(
                &pane_id
            )?;

        self.groups.get(
            group_id
        )
    }

    /// Returns all panes that should receive an operation originating
    /// from the specified pane.
    pub fn peers(
        &self,
        pane_id: PaneId,
    ) -> Vec<PaneId> {
        let Some(group) =
            self.group_for_pane(
                pane_id
            )
        else {
            return Vec::new();
        };

        group
            .panes()
            .iter()
            .copied()
            .filter(
                |id| *id != pane_id
            )
            .collect()
    }

    /// Removes empty or invalid synchronization groups.
    pub fn cleanup(
        &mut self,
        valid_panes: &HashSet<PaneId>,
    ) {
        let groups: Vec<u64> =
            self.groups
                .iter()
                .filter_map(
                    |(id, group)| {
                        if group
                            .panes()
                            .iter()
                            .all(
                                |pane| {
                                    valid_panes
                                        .contains(pane)
                                }
                            )
                        {
                            None
                        } else {
                            Some(*id)
                        }
                    }
                )
                .collect();

        for group_id in groups {
            if let Some(group) =
                self.groups.get_mut(
                    &group_id
                )
            {
                group.panes.retain(
                    |pane| {
                        valid_panes
                            .contains(pane)
                    }
                );
            }
        }

        let empty_groups: Vec<u64> =
            self.groups
                .iter()
                .filter_map(
                    |(id, group)| {
                        group.is_empty()
                            .then_some(*id)
                    }
                )
                .collect();

        for id in empty_groups {
            self.groups.remove(
                &id
            );
        }

        self.pane_groups.retain(
            |pane, group| {
                valid_panes.contains(pane)
                    && self.groups.contains_key(group)
            }
        );
    }

    pub fn clear(&mut self) {
        self.groups.clear();
        self.pane_groups.clear();
    }
}
