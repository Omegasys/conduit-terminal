use std::collections::BTreeMap;
use std::path::Path;

use super::{
    errors::{
        CustomProtocolError,
        CustomProtocolResult,
    },
    manifest::CustomProtocolManifest,
    protocol::{
        CustomProtocol,
        CustomProtocolId,
    },
};

pub struct CustomProtocolResource {
    manifest: CustomProtocolManifest,
    protocol: Option<Box<dyn CustomProtocol>>,
}

impl CustomProtocolResource {
    pub fn new(
        manifest: CustomProtocolManifest,
    ) -> Self {
        Self {
            manifest,
            protocol: None,
        }
    }

    pub fn manifest(
        &self,
    ) -> &CustomProtocolManifest {
        &self.manifest
    }

    pub fn id(
        &self,
    ) -> &CustomProtocolId {
        self.manifest.id()
    }

    pub fn root_path(
        &self,
    ) -> &Path {
        self.manifest.root_path().as_path()
    }

    pub fn entrypoint(
        &self,
    ) -> Option<&str> {
        self.manifest.entrypoint()
    }

    pub fn is_loaded(
        &self,
    ) -> bool {
        self.protocol.is_some()
    }

    pub fn set_protocol(
        &mut self,
        protocol: Box<dyn CustomProtocol>,
    ) {
        self.protocol = Some(protocol);
    }

    pub fn protocol(
        &self,
    ) -> Option<&dyn CustomProtocol> {
        self.protocol.as_deref()
    }

    pub fn protocol_mut(
        &mut self,
    ) -> Option<&mut (dyn CustomProtocol + '_)> {
        match self.protocol.as_mut() {
            Some(protocol) => Some(protocol.as_mut()),
            None => None,
        }
    }

    pub fn unload(
        &mut self,
    ) {
        self.protocol = None;
    }
}

pub struct CustomProtocolResourceManager {
    resources: BTreeMap<
        CustomProtocolId,
        CustomProtocolResource,
    >,
}

impl CustomProtocolResourceManager {
    pub fn new() -> Self {
        Self {
            resources: BTreeMap::new(),
        }
    }

    pub fn register(
        &mut self,
        resource: CustomProtocolResource,
    ) -> CustomProtocolResult<()> {
        let id = resource.id().clone();

        if self.resources.contains_key(&id) {
            return Err(
                CustomProtocolError::AlreadyRegistered(
                    id.to_string(),
                ),
            );
        }

        self.resources.insert(
            id,
            resource,
        );

        Ok(())
    }

    pub fn register_or_replace(
        &mut self,
        resource: CustomProtocolResource,
    ) {
        let id = resource.id().clone();

        self.resources.insert(
            id,
            resource,
        );
    }

    pub fn unregister(
        &mut self,
        id: &CustomProtocolId,
    ) -> Option<CustomProtocolResource> {
        self.resources.remove(id)
    }

    pub fn get(
        &self,
        id: &CustomProtocolId,
    ) -> Option<&CustomProtocolResource> {
        self.resources.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &CustomProtocolId,
    ) -> Option<&mut CustomProtocolResource> {
        self.resources.get_mut(id)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &CustomProtocolResource> {
        self.resources.values()
    }

    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut CustomProtocolResource> {
        self.resources.values_mut()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }
}

impl Default for CustomProtocolResourceManager {
    fn default() -> Self {
        Self::new()
    }
}
