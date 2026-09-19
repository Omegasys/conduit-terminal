use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

/// GUI-level settings state.
///
/// The actual persistent configuration remains owned by the configuration
/// engine. This structure represents the settings currently exposed by
/// the GUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiSettingsState {
    Clean,
    Modified,
    Applying,
    Error,
}

/// Settings being edited by the GUI.
#[derive(Debug, Clone)]
pub struct GuiSettings {
    values: BTreeMap<String, ConfigValue>,
    state: GuiSettingsState,
    last_error: Option<String>,
}

impl Default for GuiSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl GuiSettings {
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
            state: GuiSettingsState::Clean,
            last_error: None,
        }
    }

    pub fn state(&self) -> GuiSettingsState {
        self.state
    }

    pub fn values(&self) -> &BTreeMap<String, ConfigValue> {
        &self.values
    }

    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.values.get(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn set(
        &mut self,
        key: impl Into<String>,
        value: ConfigValue,
    ) {
        self.values.insert(key.into(), value);
        self.state = GuiSettingsState::Modified;
        self.last_error = None;
    }

    pub fn remove(&mut self, key: &str) -> Option<ConfigValue> {
        let result = self.values.remove(key);

        if result.is_some() {
            self.state = GuiSettingsState::Modified;
        }

        result
    }

    pub fn replace(
        &mut self,
        values: BTreeMap<String, ConfigValue>,
    ) {
        self.values = values;
        self.state = GuiSettingsState::Modified;
        self.last_error = None;
    }

    pub fn mark_applying(&mut self) {
        self.state = GuiSettingsState::Applying;
        self.last_error = None;
    }

    pub fn mark_clean(&mut self) {
        self.state = GuiSettingsState::Clean;
        self.last_error = None;
    }

    pub fn mark_error(&mut self, error: impl Into<String>) {
        self.state = GuiSettingsState::Error;
        self.last_error = Some(error.into());
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn is_modified(&self) -> bool {
        self.state == GuiSettingsState::Modified
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.state = GuiSettingsState::Modified;
        self.last_error = None;
    }
}
