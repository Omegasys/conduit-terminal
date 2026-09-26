//! Environment handling for custom shells.

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct CustomEnvironment {
    inherit_parent: bool,
    variables: BTreeMap<String, String>,
}

impl Default for CustomEnvironment {
    fn default() -> Self {
        Self {
            inherit_parent: true,
            variables: BTreeMap::new(),
        }
    }
}

impl CustomEnvironment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inherit_parent(&self) -> bool {
        self.inherit_parent
    }

    pub fn set_inherit_parent(&mut self, value: bool) {
        self.inherit_parent = value;
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.variables.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(String::as_str)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    pub fn variables(&self) -> &BTreeMap<String, String> {
        &self.variables
    }

    pub fn clear(&mut self) {
        self.variables.clear();
    }

    pub fn len(&self) -> usize {
        self.variables.len()
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}
