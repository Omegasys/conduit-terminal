use std::collections::VecDeque;
use std::time::SystemTime;

use crate::resources::{ResourceId, ResourceState};

/// A snapshot of a resource's live state before a reload.
#[derive(Debug, Clone)]
pub struct LiveSnapshot {
    resource_id: ResourceId,
    state: ResourceState,
    generation: u64,
    captured_at: SystemTime,
    reason: String,
}

impl LiveSnapshot {
    pub fn new(
        resource_id: ResourceId,
        state: ResourceState,
        generation: u64,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            resource_id,
            state,
            generation,
            captured_at: SystemTime::now(),
            reason: reason.into(),
        }
    }

    pub fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    pub fn state(&self) -> &ResourceState {
        &self.state
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn captured_at(&self) -> SystemTime {
        self.captured_at
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Result of a rollback operation.
#[derive(Debug, Clone)]
pub enum RollbackResult {
    Restored(LiveSnapshot),
    NotFound(ResourceId),
    Empty,
}

/// Maintains recent live-state snapshots.
///
/// This is intentionally state-oriented rather than filesystem-oriented.
/// Actual file/config restoration belongs to the resource/configuration
/// layer; this component tracks the live runtime state needed to safely
/// undo a reload.
#[derive(Debug)]
pub struct LiveRollbackManager {
    history: VecDeque<LiveSnapshot>,
    max_history: usize,
}

impl Default for LiveRollbackManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LiveRollbackManager {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            max_history: 32,
        }
    }

    pub fn with_capacity(max_history: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    pub fn record(&mut self, snapshot: LiveSnapshot) {
        if self.max_history == 0 {
            return;
        }

        self.history.push_back(snapshot);

        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    pub fn latest(&self) -> Option<&LiveSnapshot> {
        self.history.back()
    }

    pub fn latest_for(&self, resource_id: ResourceId) -> Option<&LiveSnapshot> {
        self.history
            .iter()
            .rev()
            .find(|snapshot| snapshot.resource_id() == resource_id)
    }

    pub fn rollback(&mut self, resource_id: ResourceId) -> RollbackResult {
        let position = self
            .history
            .iter()
            .rposition(|snapshot| snapshot.resource_id() == resource_id);

        match position {
            Some(index) => {
                let snapshot = self
                    .history
                    .remove(index)
                    .expect("rollback snapshot index must remain valid");

                RollbackResult::Restored(snapshot)
            }
            None => RollbackResult::NotFound(resource_id),
        }
    }

    pub fn pop_latest(&mut self) -> RollbackResult {
        match self.history.pop_back() {
            Some(snapshot) => RollbackResult::Restored(snapshot),
            None => RollbackResult::Empty,
        }
    }

    pub fn remove_resource(&mut self, resource_id: ResourceId) {
        self.history
            .retain(|snapshot| snapshot.resource_id() != resource_id);
    }

    pub fn set_max_history(&mut self, max_history: usize) {
        self.max_history = max_history;

        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    pub fn max_history(&self) -> usize {
        self.max_history
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &LiveSnapshot> {
        self.history.iter()
    }
}
