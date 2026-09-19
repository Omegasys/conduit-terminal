use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};

use super::diff::ConfigDiff;
use super::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigStateStatus {
    Uninitialized,
    Loading,
    Ready,
    Dirty,
    Saving,
    Error,
}

#[derive(Debug, Clone)]
pub struct ConfigState {
    values: BTreeMap<String, ConfigValue>,
    status: ConfigStateStatus,
    version: u64,
    config_version: u32,
    last_saved: Option<SystemTime>,
    last_modified: Option<SystemTime>,
    last_error: Option<String>,
}

impl ConfigState {
    pub fn new(config_version: u32) -> Self {
        Self {
            values: BTreeMap::new(),
            status: ConfigStateStatus::Uninitialized,
            version: 0,
            config_version,
            last_saved: None,
            last_modified: None,
            last_error: None,
        }
    }

    pub fn values(&self) -> &BTreeMap<String, ConfigValue> {
        &self.values
    }

    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.values.get(key)
    }

    pub fn status(&self) -> ConfigStateStatus {
        self.status
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn config_version(&self) -> u32 {
        self.config_version
    }

    pub fn last_saved(&self) -> Option<SystemTime> {
        self.last_saved
    }

    pub fn last_modified(&self) -> Option<SystemTime> {
        self.last_modified
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn load(
        &mut self,
        values: BTreeMap<String, ConfigValue>,
    ) {
        self.values = values;
        self.status = ConfigStateStatus::Ready;
        self.version += 1;
        self.last_modified = Some(SystemTime::now());
        self.last_error = None;
    }

    pub fn replace(
        &mut self,
        values: BTreeMap<String, ConfigValue>,
    ) {
        self.values = values;
        self.version += 1;
        self.status = ConfigStateStatus::Dirty;
        self.last_modified = Some(SystemTime::now());
        self.last_error = None;
    }

    pub fn apply_diff(&mut self, diff: &ConfigDiff) {
        diff.apply_to(&mut self.values);

        if !diff.is_empty() {
            self.version += 1;
            self.status = ConfigStateStatus::Dirty;
            self.last_modified = Some(SystemTime::now());
        }
    }

    pub fn mark_loading(&mut self) {
        self.status = ConfigStateStatus::Loading;
        self.last_error = None;
    }

    pub fn mark_saving(&mut self) {
        self.status = ConfigStateStatus::Saving;
        self.last_error = None;
    }

    pub fn mark_saved(&mut self) {
        self.status = ConfigStateStatus::Ready;
        self.last_saved = Some(SystemTime::now());
        self.last_error = None;
    }

    pub fn mark_error<S: Into<String>>(&mut self, error: S) {
        self.status = ConfigStateStatus::Error;
        self.last_error = Some(error.into());
    }

    pub fn is_dirty(&self) -> bool {
        matches!(self.status, ConfigStateStatus::Dirty | ConfigStateStatus::Saving)
    }

    pub fn time_since_save(&self) -> Option<Duration> {
        self.last_saved
            .and_then(|time| SystemTime::now().duration_since(time).ok())
    }
}

impl Default for ConfigState {
    fn default() -> Self {
        Self::new(1)
    }
}
