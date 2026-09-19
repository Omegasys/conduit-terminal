//! Terminal process environment management.
//!
//! Environment state is kept separate from terminal rendering state so
//! sessions can construct and modify their process environment without
//! coupling it to the screen.

use std::collections::BTreeMap;

/// Environment variables for a terminal session.
#[derive(Debug, Clone)]
pub struct Environment {
    variables: BTreeMap<String, String>,
}

impl Environment {
    /// Creates an environment populated from the current process.
    pub fn from_process() -> Self {
        Self {
            variables: std::env::vars().collect(),
        }
    }

    /// Creates an empty environment.
    pub fn empty() -> Self {
        Self {
            variables: BTreeMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(String::as_str)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    pub fn set<K: Into<String>, V: Into<String>>(
        &mut self,
        key: K,
        value: V,
    ) {
        self.variables.insert(key.into(), value.into());
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

    /// Returns environment variables in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.variables.iter()
    }

    /// Converts the environment into a vector suitable for process
    /// creation.
    pub fn to_vec(&self) -> Vec<(String, String)> {
        self.variables
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }

    /// Applies a collection of overrides.
    pub fn apply(
        &mut self,
        variables: impl IntoIterator<Item = (String, String)>,
    ) {
        for (key, value) in variables {
            self.set(key, value);
        }
    }

    /// Returns the user's configured shell.
    pub fn shell(&self) -> Option<&str> {
        self.get("SHELL")
    }

    /// Returns the current working directory from the environment.
    pub fn current_directory(&self) -> Option<&str> {
        self.get("PWD")
    }

    /// Sets the terminal type.
    pub fn set_term(&mut self, term: &str) {
        self.set("TERM", term);
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::from_process()
    }
}
