use std::collections::BTreeMap;

use super::diff::ConfigDiff;
use super::errors::ConfigError;
use super::ConfigValue;

#[derive(Debug, Clone)]
pub struct ConfigTransaction {
    original: BTreeMap<String, ConfigValue>,
    working: BTreeMap<String, ConfigValue>,
    committed: bool,
}

impl ConfigTransaction {
    pub fn new(current: &BTreeMap<String, ConfigValue>) -> Self {
        Self {
            original: current.clone(),
            working: current.clone(),
            committed: false,
        }
    }

    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.working.get(key)
    }

    pub fn values(&self) -> &BTreeMap<String, ConfigValue> {
        &self.working
    }

    pub fn set<S: Into<String>>(
        &mut self,
        key: S,
        value: ConfigValue,
    ) {
        self.working.insert(key.into(), value);
    }

    pub fn remove(&mut self, key: &str) {
        self.working.remove(key);
    }

    pub fn diff(&self) -> ConfigDiff {
        ConfigDiff::from_maps(&self.original, &self.working)
    }

    pub fn is_dirty(&self) -> bool {
        self.original != self.working
    }

    pub fn commit(
        &mut self,
        target: &mut BTreeMap<String, ConfigValue>,
    ) -> Result<ConfigDiff, ConfigError> {
        if self.committed {
            return Err(ConfigError::TransactionAlreadyCommitted);
        }

        let diff = self.diff();

        *target = self.working.clone();
        self.committed = true;

        Ok(diff)
    }

    pub fn rollback(&mut self) {
        self.working = self.original.clone();
        self.committed = false;
    }

    pub fn into_working(self) -> BTreeMap<String, ConfigValue> {
        self.working
    }

    pub fn is_committed(&self) -> bool {
        self.committed
    }
}
