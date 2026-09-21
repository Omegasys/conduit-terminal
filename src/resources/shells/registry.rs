use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::shell::custom::CustomShellId;

use super::manifest::ShellResourceManifest;

#[derive(Debug, Clone)]
pub struct ShellResourceEntry {
    manifest: ShellResourceManifest,
    enabled: bool,
}

impl ShellResourceEntry {
    pub fn new(manifest: ShellResourceManifest) -> Self {
        Self {
            manifest,
            enabled: true,
        }
    }

    pub fn manifest(&self) -> &ShellResourceManifest {
        &self.manifest
    }

    pub fn id(&self) -> &CustomShellId {
        self.manifest.id()
    }

    pub fn path(&self) -> &PathBuf {
        self.manifest.root_path()
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[derive(Debug, Default)]
pub struct ShellResourceRegistry {
    shells: BTreeMap<CustomShellId, ShellResourceEntry>,
}

impl ShellResourceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        manifest: ShellResourceManifest,
    ) -> Result<(), String> {
        let id = manifest.id().clone();

        if self.shells.contains_key(&id) {
            return Err(format!(
                "Shell resource already registered: {id}"
            ));
        }

        self.shells
            .insert(id, ShellResourceEntry::new(manifest));

        Ok(())
    }

    pub fn register_or_replace(
        &mut self,
        manifest: ShellResourceManifest,
    ) {
        let id = manifest.id().clone();

        let enabled = self
            .shells
            .get(&id)
            .map(ShellResourceEntry::enabled)
            .unwrap_or(true);

        let mut entry = ShellResourceEntry::new(manifest);
        entry.set_enabled(enabled);

        self.shells.insert(id, entry);
    }

    pub fn unregister(
        &mut self,
        id: &CustomShellId,
    ) -> Option<ShellResourceEntry> {
        self.shells.remove(id)
    }

    pub fn get(
        &self,
        id: &CustomShellId,
    ) -> Option<&ShellResourceEntry> {
        self.shells.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &CustomShellId,
    ) -> Option<&mut ShellResourceEntry> {
        self.shells.get_mut(id)
    }

    pub fn enable(
        &mut self,
        id: &CustomShellId,
    ) -> Result<(), String> {
        let shell = self
            .get_mut(id)
            .ok_or_else(|| format!("Shell not found: {id}"))?;

        shell.enable();

        Ok(())
    }

    pub fn disable(
        &mut self,
        id: &CustomShellId,
    ) -> Result<(), String> {
        let shell = self
            .get_mut(id)
            .ok_or_else(|| format!("Shell not found: {id}"))?;

        shell.disable();

        Ok(())
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &ShellResourceEntry> {
        self.shells.values()
    }

    pub fn iter_enabled(
        &self,
    ) -> impl Iterator<Item = &ShellResourceEntry> {
        self.shells.values().filter(|entry| entry.enabled())
    }

    pub fn len(&self) -> usize {
        self.shells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shells.is_empty()
    }

    pub fn clear(&mut self) {
        self.shells.clear();
    }
}
