use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::terminal_protocols::custom::{
    CustomProtocolCapabilities,
    CustomProtocolId,
    CustomProtocolVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolResourceMetadata {
    pub author: String,
    pub description: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
}

impl ProtocolResourceMetadata {
    pub fn new(
        author: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            author: author.into(),
            description: description.into(),
            homepage: None,
            license: None,
        }
    }

    pub fn homepage(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.homepage = Some(value.into());
        self
    }

    pub fn license(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.license = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolResourceManifest {
    id: CustomProtocolId,
    name: String,
    version: CustomProtocolVersion,

    metadata: ProtocolResourceMetadata,
    capabilities: CustomProtocolCapabilities,

    entrypoint: Option<String>,

    dependencies: Vec<String>,
    settings: BTreeMap<String, String>,

    root_path: PathBuf,
}

impl ProtocolResourceManifest {
    pub fn new(
        id: CustomProtocolId,
        name: impl Into<String>,
        metadata: ProtocolResourceMetadata,
        root_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            version: CustomProtocolVersion::initial(),
            metadata,
            capabilities: CustomProtocolCapabilities::default(),
            entrypoint: None,
            dependencies: Vec::new(),
            settings: BTreeMap::new(),
            root_path: root_path.into(),
        }
    }

    pub fn id(&self) -> &CustomProtocolId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> CustomProtocolVersion {
        self.version
    }

    pub fn metadata(&self) -> &ProtocolResourceMetadata {
        &self.metadata
    }

    pub fn capabilities(
        &self,
    ) -> &CustomProtocolCapabilities {
        &self.capabilities
    }

    pub fn entrypoint(&self) -> Option<&str> {
        self.entrypoint.as_deref()
    }

    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    pub fn settings(
        &self,
    ) -> &BTreeMap<String, String> {
        &self.settings
    }

    pub fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    pub fn set_version(
        &mut self,
        version: CustomProtocolVersion,
    ) {
        self.version = version;
    }

    pub fn set_capabilities(
        &mut self,
        capabilities: CustomProtocolCapabilities,
    ) {
        self.capabilities = capabilities;
    }

    pub fn set_entrypoint(
        &mut self,
        value: impl Into<String>,
    ) {
        self.entrypoint = Some(value.into());
    }

    pub fn add_dependency(
        &mut self,
        value: impl Into<String>,
    ) {
        self.dependencies.push(value.into());
    }

    pub fn set_setting(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.settings.insert(
            key.into(),
            value.into(),
        );
    }
}
