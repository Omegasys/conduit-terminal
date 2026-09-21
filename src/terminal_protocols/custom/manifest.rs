use std::collections::BTreeMap;

use super::{
    capabilities::CustomProtocolCapabilities,
    protocol::{ProtocolId, ProtocolVersion},
};

#[derive(Clone, Debug)]
pub struct CustomProtocolMetadata {
    pub author: String,
    pub homepage: Option<String>,
    pub description: String,
    pub license: Option<String>,
}

impl CustomProtocolMetadata {
    pub fn new<S1, S2>(author: S1, description: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            author: author.into(),
            homepage: None,
            description: description.into(),
            license: None,
        }
    }

    pub fn homepage<S: Into<String>>(mut self, value: S) -> Self {
        self.homepage = Some(value.into());
        self
    }

    pub fn license<S: Into<String>>(mut self, value: S) -> Self {
        self.license = Some(value.into());
        self
    }
}

#[derive(Clone, Debug)]
pub struct CustomProtocolManifest {
    id: ProtocolId,
    name: String,
    version: ProtocolVersion,
    metadata: CustomProtocolMetadata,
    capabilities: CustomProtocolCapabilities,
    entrypoint: Option<String>,
    dependencies: Vec<ProtocolId>,
    settings: BTreeMap<String, String>,
}

impl CustomProtocolManifest {
    pub fn new<S1, S2>(
        id: ProtocolId,
        name: S1,
        metadata: CustomProtocolMetadata,
    ) -> Self
    where
        S1: Into<String>,
    {
        Self {
            id,
            name: name.into(),
            version: ProtocolVersion::initial(),
            metadata,
            capabilities: CustomProtocolCapabilities::default(),
            entrypoint: None,
            dependencies: Vec::new(),
            settings: BTreeMap::new(),
        }
    }

    pub fn id(&self) -> &ProtocolId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &ProtocolVersion {
        &self.version
    }

    pub fn metadata(&self) -> &CustomProtocolMetadata {
        &self.metadata
    }

    pub fn capabilities(&self) -> &CustomProtocolCapabilities {
        &self.capabilities
    }

    pub fn entrypoint(&self) -> Option<&str> {
        self.entrypoint.as_deref()
    }

    pub fn dependencies(&self) -> &[ProtocolId] {
        &self.dependencies
    }

    pub fn settings(&self) -> &BTreeMap<String, String> {
        &self.settings
    }

    pub fn set_version(&mut self, version: ProtocolVersion) {
        self.version = version;
    }

    pub fn set_capabilities(&mut self, capabilities: CustomProtocolCapabilities) {
        self.capabilities = capabilities;
    }

    pub fn set_entrypoint<S: Into<String>>(&mut self, entrypoint: S) {
        self.entrypoint = Some(entrypoint.into());
    }

    pub fn add_dependency(&mut self, dependency: ProtocolId) {
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }

    pub fn set_setting<S1, S2>(&mut self, key: S1, value: S2)
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.settings.insert(key.into(), value.into());
    }
}
