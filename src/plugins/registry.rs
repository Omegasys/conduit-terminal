use std::collections::HashMap;

use super::api::PluginId;
use super::lifecycle::LifecycleState;
use super::manifest::PluginManifest;

#[derive(Debug, Clone)]
pub struct PluginRegistration {
    pub id: PluginId,
    pub manifest: PluginManifest,
    pub state: LifecycleState,
    pub enabled: bool,
}

impl PluginRegistration {
    pub fn new(manifest: PluginManifest) -> Self {
        let id = PluginId::new(manifest.id().to_string());

        Self {
            id,
            manifest,
            state: LifecycleState::Discovered,
            enabled: true,
        }
    }
}

#[derive(Debug, Default)]
pub struct PluginRegistry {
    plugins: HashMap<PluginId, PluginRegistration>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        registration: PluginRegistration,
    ) -> Option<PluginRegistration> {
        self.plugins.insert(registration.id.clone(), registration)
    }

    pub fn get(
        &self,
        id: &PluginId,
    ) -> Option<&PluginRegistration> {
        self.plugins.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &PluginId,
    ) -> Option<&mut PluginRegistration> {
        self.plugins.get_mut(id)
    }

    pub fn remove(
        &mut self,
        id: &PluginId,
    ) -> Option<PluginRegistration> {
        self.plugins.remove(id)
    }

    pub fn contains(&self, id: &PluginId) -> bool {
        self.plugins.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&PluginId, &PluginRegistration)> {
        self.plugins.iter()
    }

    pub fn enabled(
        &self,
    ) -> impl Iterator<Item = &PluginRegistration> {
        self.plugins.values().filter(|plugin| plugin.enabled)
    }

    pub fn set_enabled(
        &mut self,
        id: &PluginId,
        enabled: bool,
    ) -> Result<(), String> {
        let plugin = self
            .plugins
            .get_mut(id)
            .ok_or_else(|| format!("plugin '{}' not found", id))?;

        plugin.enabled = enabled;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.plugins.clear();
    }
}
