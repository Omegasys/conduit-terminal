use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::terminal_protocols::custom::CustomProtocolId;

use super::manifest::ProtocolResourceManifest;

#[derive(Debug, Clone)]
pub struct ProtocolResourceEntry {
    manifest: ProtocolResourceManifest,
    enabled: bool,
}

impl ProtocolResourceEntry {
    pub fn new(
        manifest: ProtocolResourceManifest,
    ) -> Self {
        Self {
            manifest,
            enabled: true,
        }
    }

    pub fn manifest(
        &self,
    ) -> &ProtocolResourceManifest {
        &self.manifest
    }

    pub fn id(
        &self,
    ) -> &CustomProtocolId {
        self.manifest.id()
    }

    pub fn path(
        &self,
    ) -> &PathBuf {
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

    pub fn set_enabled(
        &mut self,
        enabled: bool,
    ) {
        self.enabled = enabled;
    }
}

#[derive(Debug, Default)]
pub struct ProtocolResourceRegistry {
    protocols: BTreeMap<
        CustomProtocolId,
        ProtocolResourceEntry,
    >,
}

impl ProtocolResourceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        manifest: ProtocolResourceManifest,
    ) -> Result<(), String> {
        let id = manifest.id().clone();

        if self.protocols.contains_key(&id) {
            return Err(format!(
                "Protocol resource already registered: {id}"
            ));
        }

        self.protocols.insert(
            id,
            ProtocolResourceEntry::new(
                manifest,
            ),
        );

        Ok(())
    }

    pub fn register_or_replace(
        &mut self,
        manifest: ProtocolResourceManifest,
    ) {
        let id = manifest.id().clone();

        let enabled = self
            .protocols
            .get(&id)
            .map(ProtocolResourceEntry::enabled)
            .unwrap_or(true);

        let mut entry =
            ProtocolResourceEntry::new(
                manifest,
            );

        entry.set_enabled(enabled);

        self.protocols.insert(
            id,
            entry,
        );
    }

    pub fn unregister(
        &mut self,
        id: &CustomProtocolId,
    ) -> Option<ProtocolResourceEntry> {
        self.protocols.remove(id)
    }

    pub fn get(
        &self,
        id: &CustomProtocolId,
    ) -> Option<&ProtocolResourceEntry> {
        self.protocols.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &CustomProtocolId,
    ) -> Option<&mut ProtocolResourceEntry> {
        self.protocols.get_mut(id)
    }

    pub fn enable(
        &mut self,
        id: &CustomProtocolId,
    ) -> Result<(), String> {
        let protocol = self
            .get_mut(id)
            .ok_or_else(|| {
                format!("Protocol not found: {id}")
            })?;

        protocol.enable();

        Ok(())
    }

    pub fn disable(
        &mut self,
        id: &CustomProtocolId,
    ) -> Result<(), String> {
        let protocol = self
            .get_mut(id)
            .ok_or_else(|| {
                format!("Protocol not found: {id}")
            })?;

        protocol.disable();

        Ok(())
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = &ProtocolResourceEntry,
    > {
        self.protocols.values()
    }

    pub fn iter_enabled(
        &self,
    ) -> impl Iterator<
        Item = &ProtocolResourceEntry,
    > {
        self.protocols
            .values()
            .filter(|entry| entry.enabled())
    }

    pub fn len(&self) -> usize {
        self.protocols.len()
    }

    pub fn is_empty(&self) -> bool {
        self.protocols.is_empty()
    }

    pub fn clear(&mut self) {
        self.protocols.clear();
    }
}
