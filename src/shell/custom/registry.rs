//! Registry for custom shells.

use std::collections::BTreeMap;

use super::{
    errors::{CustomShellError, CustomShellResult},
    shell::CustomShell,
};

#[derive(Debug, Default)]
pub struct CustomShellRegistry {
    shells: BTreeMap<String, CustomShell>,
}

impl CustomShellRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, shell: CustomShell) -> CustomShellResult<()> {
        let id = shell.id().to_string();

        if self.shells.contains_key(&id) {
            return Err(CustomShellError::AlreadyRegistered(id));
        }

        self.shells.insert(id, shell);
        Ok(())
    }

    pub fn register_or_replace(&mut self, shell: CustomShell) {
        self.shells.insert(shell.id().to_string(), shell);
    }

    pub fn get(&self, id: &str) -> Option<&CustomShell> {
        self.shells.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut CustomShell> {
        self.shells.get_mut(id)
    }

    pub fn remove(&mut self, id: &str) -> Option<CustomShell> {
        self.shells.remove(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.shells.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.shells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shells.is_empty()
    }

    pub fn ids(&self) -> impl Iterator<Item = &String> {
        self.shells.keys()
    }

    pub fn shells(&self) -> impl Iterator<Item = &CustomShell> {
        self.shells.values()
    }
}
