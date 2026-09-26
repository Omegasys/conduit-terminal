//! Shell environment tracking.

use std::collections::BTreeMap;

/// Environment variables associated with a shell session.
#[derive(Debug, Clone, Default)]
pub struct Environment {
    variables: BTreeMap<String, String>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_current_process() -> Self {
        Self {
            variables: std::env::vars().collect(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(String::as_str)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    pub fn set(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<String> {
        self.variables.insert(key.into(), value.into())
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.variables.remove(key)
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

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&String, &String)> {
        self.variables.iter()
    }

    pub fn to_vec(&self) -> Vec<(String, String)> {
        self.variables
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }

    pub fn apply(
        &self,
        command: &mut std::process::Command,
    ) {
        command.envs(&self.variables);
    }

    /// Compare two environments and report the changes.
    pub fn diff(&self, other: &Self) -> Vec<EnvironmentChange> {
        let mut changes = Vec::new();

        for (key, value) in &other.variables {
            match self.variables.get(key) {
                None => changes.push(EnvironmentChange::Set {
                    key: key.clone(),
                    value: value.clone(),
                }),
                Some(old) if old != value => {
                    changes.push(EnvironmentChange::Changed {
                        key: key.clone(),
                        old: old.clone(),
                        new: value.clone(),
                    });
                }
                _ => {}
            }
        }

        for (key, value) in &self.variables {
            if !other.variables.contains_key(key) {
                changes.push(EnvironmentChange::Removed {
                    key: key.clone(),
                    value: value.clone(),
                });
            }
        }

        changes
    }
}

/// A change between two shell environments.
#[derive(Debug, Clone)]
pub enum EnvironmentChange {
    Set {
        key: String,
        value: String,
    },
    Changed {
        key: String,
        old: String,
        new: String,
    },
    Removed {
        key: String,
        value: String,
    },
}

impl EnvironmentChange {
    pub fn key(&self) -> &str {
        match self {
            Self::Set { key, .. }
            | Self::Changed { key, .. }
            | Self::Removed { key, .. } => key,
        }
    }
}
