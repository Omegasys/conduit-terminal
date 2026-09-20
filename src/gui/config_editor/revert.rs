use crate::config_engine::{ConfigRollbackManager, ConfigState};

/// Result of reverting configuration changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevertResult {
    Reverted,
    NothingToRevert,
    SnapshotUnavailable,
}

/// Controls what happens when a revert is requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevertMode {
    LastChange,
    LastSnapshot,
    Original,
}

/// Configuration revert controller.
#[derive(Debug, Default)]
pub struct ConfigRevertController {
    last_result: Option<RevertResult>,
}

impl ConfigRevertController {
    pub fn new() -> Self {
        Self {
            last_result: None,
        }
    }

    pub fn last_result(&self) -> Option<RevertResult> {
        self.last_result
    }

    /// Restores the current state to the latest rollback snapshot.
    pub fn revert_to_latest(
        &mut self,
        state: &mut ConfigState,
        rollback: &mut ConfigRollbackManager,
    ) -> RevertResult {
        let snapshot = match rollback.latest() {
            Some(snapshot) => snapshot,
            None => {
                self.last_result = Some(RevertResult::SnapshotUnavailable);
                return RevertResult::SnapshotUnavailable;
            }
        };

        state.replace(snapshot.values().clone());
        state.mark_saved();

        self.last_result = Some(RevertResult::Reverted);

        RevertResult::Reverted
    }

    /// Reverts the most recent snapshot and removes it from history.
    pub fn pop_and_revert(
        &mut self,
        state: &mut ConfigState,
        rollback: &mut ConfigRollbackManager,
    ) -> RevertResult {
        let snapshot = match rollback.pop_latest() {
            Some(snapshot) => snapshot,
            None => {
                self.last_result = Some(RevertResult::NothingToRevert);
                return RevertResult::NothingToRevert;
            }
        };

        state.replace(snapshot.values().clone());
        state.mark_saved();

        self.last_result = Some(RevertResult::Reverted);

        RevertResult::Reverted
    }

    /// Discards unsaved changes and restores the supplied original values.
    pub fn revert_to_original(
        &mut self,
        state: &mut ConfigState,
        original: &std::collections::BTreeMap<
            String,
            crate::config_engine::ConfigValue,
        >,
    ) -> RevertResult {
        state.replace(original.clone());
        state.mark_saved();

        self.last_result = Some(RevertResult::Reverted);

        RevertResult::Reverted
    }

    pub fn reset(&mut self) {
        self.last_result = None;
    }
}
