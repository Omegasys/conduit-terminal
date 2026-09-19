use std::collections::BTreeMap;

use super::errors::ConfigError;
use super::ConfigValue;

#[derive(Debug, Clone)]
pub struct ConfigWorkspace {
    name: String,
    values: BTreeMap<String, ConfigValue>,
    read_only: bool,
}

impl ConfigWorkspace {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            values: BTreeMap::new(),
            read_only: false,
        }
    }

    pub fn read_only<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            values: BTreeMap::new(),
            read_only: true,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_read_only(&self) -> bool {
        self.read_only
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

    pub fn set<S: Into<String>>(
        &mut self,
        key: S,
        value: ConfigValue,
    ) -> Result<(), ConfigError> {
        if self.read_only {
            return Err(ConfigError::ReadOnlyWorkspace(self.name.clone()));
        }

        self.values.insert(key.into(), value);
        Ok(())
    }

    pub fn remove(&mut self, key: &str) -> Result<Option<ConfigValue>, ConfigError> {
        if self.read_only {
            return Err(ConfigError::ReadOnlyWorkspace(self.name.clone()));
        }

        Ok(self.values.remove(key))
    }

    pub fn clear(&mut self) -> Result<(), ConfigError> {
        if self.read_only {
            return Err(ConfigError::ReadOnlyWorkspace(self.name.clone()));
        }

        self.values.clear();
        Ok(())
    }

    pub fn replace(
        &mut self,
        values: BTreeMap<String, ConfigValue>,
    ) -> Result<(), ConfigError> {
        if self.read_only {
            return Err(ConfigError::ReadOnlyWorkspace(self.name.clone()));
        }

        self.values = values;
        Ok(())
    }

    pub fn into_values(self) -> BTreeMap<String, ConfigValue> {
        self.values
    }
}

#[derive(Debug, Default)]
pub struct ConfigWorkspaceManager {
    workspaces: BTreeMap<String, ConfigWorkspace>,
    active: Option<String>,
}

impl ConfigWorkspaceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, workspace: ConfigWorkspace) -> Result<(), ConfigError> {
        let name = workspace.name().to_owned();

        if self.workspaces.contains_key(&name) {
            return Err(ConfigError::WorkspaceAlreadyExists(name));
        }

        self.workspaces.insert(name.clone(), workspace);

        if self.active.is_none() {
            self.active = Some(name);
        }

        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<ConfigWorkspace, ConfigError> {
        if self.active.as_deref() == Some(name) {
            self.active = None;
        }

        self.workspaces
            .remove(name)
            .ok_or_else(|| ConfigError::WorkspaceNotFound(name.to_owned()))
    }

    pub fn get(&self, name: &str) -> Option<&ConfigWorkspace> {
        self.workspaces.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut ConfigWorkspace> {
        self.workspaces.get_mut(name)
    }

    pub fn active(&self) -> Option<&ConfigWorkspace> {
        self.active
            .as_deref()
            .and_then(|name| self.workspaces.get(name))
    }

    pub fn active_mut(&mut self) -> Option<&mut ConfigWorkspace> {
        let name = self.active.clone()?;
        self.workspaces.get_mut(&name)
    }

    pub fn activate(&mut self, name: &str) -> Result<(), ConfigError> {
        if !self.workspaces.contains_key(name) {
            return Err(ConfigError::WorkspaceNotFound(name.to_owned()));
        }

        self.active = Some(name.to_owned());
        Ok(())
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.workspaces.keys().map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.workspaces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.workspaces.is_empty()
    }
}
