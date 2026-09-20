use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationSettingsUiState {
    Clean,
    Modified,
    Applying,
    Applied,
    Error,
}

#[derive(Debug, Clone)]
pub struct ApplicationSettingsUi {
    values: std::collections::BTreeMap<String, ConfigValue>,
    state: ApplicationSettingsUiState,
    last_error: Option<String>,
}

impl ApplicationSettingsUi {
    pub fn new() -> Self {
        Self {
            values: std::collections::BTreeMap::new(),
            state: ApplicationSettingsUiState::Clean,
            last_error: None,
        }
    }

    pub fn state(&self) -> ApplicationSettingsUiState {
        self.state
    }

    pub fn values(
        &self,
    ) -> &std::collections::BTreeMap<String, ConfigValue> {
        &self.values
    }

    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.values.get(key)
    }

    pub fn set(
        &mut self,
        key: impl Into<String>,
        value: ConfigValue,
    ) {
        self.values.insert(key.into(), value);
        self.state = ApplicationSettingsUiState::Modified;
        self.last_error = None;
    }

    pub fn remove(&mut self, key: &str) -> Option<ConfigValue> {
        let value = self.values.remove(key);

        if value.is_some() {
            self.state = ApplicationSettingsUiState::Modified;
        }

        value
    }

    pub fn replace(
        &mut self,
        values: std::collections::BTreeMap<String, ConfigValue>,
    ) {
        self.values = values;
        self.state = ApplicationSettingsUiState::Modified;
        self.last_error = None;
    }

    pub fn mark_applying(&mut self) {
        self.state = ApplicationSettingsUiState::Applying;
        self.last_error = None;
    }

    pub fn mark_applied(&mut self) {
        self.state = ApplicationSettingsUiState::Applied;
        self.last_error = None;
    }

    pub fn mark_clean(&mut self) {
        self.state = ApplicationSettingsUiState::Clean;
        self.last_error = None;
    }

    pub fn mark_error(&mut self, error: impl Into<String>) {
        self.state = ApplicationSettingsUiState::Error;
        self.last_error = Some(error.into());
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn is_modified(&self) -> bool {
        self.state == ApplicationSettingsUiState::Modified
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.state = ApplicationSettingsUiState::Clean;
        self.last_error = None;
    }
}

impl Default for ApplicationSettingsUi {
    fn default() -> Self {
        Self::new()
    }
}
