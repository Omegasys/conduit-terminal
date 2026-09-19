use std::collections::BTreeMap;

use super::resource::ResourceKind;

#[derive(Debug, Clone)]
pub struct ResourceManifest {
    name: String,
    version: String,
    kind: ResourceKind,
    description: Option<String>,
    author: Option<String>,
    entrypoint: Option<String>,
    dependencies: Vec<String>,
    capabilities: Vec<String>,
    metadata: BTreeMap<String, String>,
}

impl ResourceManifest {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        kind: ResourceKind,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            kind,
            description: None,
            author: None,
            entrypoint: None,
            dependencies: Vec::new(),
            capabilities: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn kind(&self) -> ResourceKind {
        self.kind
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    pub fn entrypoint(&self) -> Option<&str> {
        self.entrypoint.as_deref()
    }

    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    pub fn set_description(&mut self, value: impl Into<String>) {
        self.description = Some(value.into());
    }

    pub fn set_author(&mut self, value: impl Into<String>) {
        self.author = Some(value.into());
    }

    pub fn set_entrypoint(&mut self, value: impl Into<String>) {
        self.entrypoint = Some(value.into());
    }

    pub fn add_dependency(&mut self, dependency: impl Into<String>) {
        self.dependencies.push(dependency.into());
    }

    pub fn add_capability(&mut self, capability: impl Into<String>) {
        self.capabilities.push(capability.into());
    }

    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.metadata.insert(key.into(), value.into());
    }
}
