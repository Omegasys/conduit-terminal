use std::collections::VecDeque;

use super::diff::ConfigDiff;
use super::errors::ConfigError;
use super::ConfigValue;

#[derive(Debug, Clone)]
pub struct ConfigSnapshot {
    pub values: std::collections::BTreeMap<String, ConfigValue>,
    pub reason: String,
}

impl ConfigSnapshot {
    pub fn new<S: Into<String>>(
        values: std::collections::BTreeMap<String, ConfigValue>,
        reason: S,
    ) -> Self {
        Self {
            values,
            reason: reason.into(),
        }
    }
}

#[derive(Debug)]
pub struct ConfigRollbackManager {
    history: VecDeque<ConfigSnapshot>,
    max_history: usize,
}

impl ConfigRollbackManager {
    pub fn new(max_history: usize) -> Self {
        Self {
            history: VecDeque::new(),
            max_history: max_history.max(1),
        }
    }

    pub fn record<S: Into<String>>(
        &mut self,
        values: &std::collections::BTreeMap<String, ConfigValue>,
        reason: S,
    ) {
        self.history
            .push_back(ConfigSnapshot::new(values.clone(), reason));

        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    pub fn latest(&self) -> Option<&ConfigSnapshot> {
        self.history.back()
    }

    pub fn get(&self, index: usize) -> Option<&ConfigSnapshot> {
        self.history.get(index)
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }

    pub fn rollback_to(
        &self,
        index: usize,
        current: &mut std::collections::BTreeMap<String, ConfigValue>,
    ) -> Result<ConfigDiff, ConfigError> {
        let snapshot = self
            .history
            .get(index)
            .ok_or(ConfigError::SnapshotNotFound(index))?;

        let diff = ConfigDiff::from_maps(current, &snapshot.values);

        *current = snapshot.values.clone();

        Ok(diff)
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }
}

impl Default for ConfigRollbackManager {
    fn default() -> Self {
        Self::new(32)
    }
}
