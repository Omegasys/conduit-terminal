use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::shell::custom::{
    CustomShellCapabilities,
    CustomShellId,
    CustomShellVersion,
    ShellExecutionModel,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellResourceMetadata {
    pub author: String,
    pub description: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
}

impl ShellResourceMetadata {
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

    pub fn homepage(mut self, homepage: impl Into<String>) -> Self {
        self.homepage = Some(homepage.into());
        self
    }

    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellResourceManifest {
    id: CustomShellId,
    name: String,
    version: CustomShellVersion,
    metadata: ShellResourceMetadata,
    capabilities: CustomShellCapabilities,
    executable: Option<String>,
    entrypoint: Option<String>,
    dependencies: Vec<String>,
    settings: BTreeMap<String, String>,
    root_path: PathBuf,
}

impl ShellResourceManifest {
    pub fn new(
        id: CustomShellId,
        name: impl Into<String>,
        metadata: ShellResourceMetadata,
        root_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            version: CustomShellVersion::initial(),
            metadata,
            capabilities: CustomShellCapabilities::default(),
            executable: None,
            entrypoint: None,
            dependencies: Vec::new(),
            settings: BTreeMap::new(),
            root_path: root_path.into(),
        }
    }

    pub fn id(&self) -> &CustomShellId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> CustomShellVersion {
        self.version
    }

    pub fn metadata(&self) -> &ShellResourceMetadata {
        &self.metadata
    }

    pub fn capabilities(&self) -> &CustomShellCapabilities {
        &self.capabilities
    }

    pub fn executable(&self) -> Option<&str> {
        self.executable.as_deref()
    }

    pub fn entrypoint(&self) -> Option<&str> {
        self.entrypoint.as_deref()
    }

    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    pub fn settings(&self) -> &BTreeMap<String, String> {
        &self.settings
    }

    pub fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    pub fn set_version(&mut self, version: CustomShellVersion) {
        self.version = version;
    }

    pub fn set_capabilities(&mut self, capabilities: CustomShellCapabilities) {
        self.capabilities = capabilities;
    }

    pub fn set_executable(&mut self, executable: impl Into<String>) {
        self.executable = Some(executable.into());
    }

    pub fn set_entrypoint(&mut self, entrypoint: impl Into<String>) {
        self.entrypoint = Some(entrypoint.into());
    }

    pub fn add_dependency(&mut self, dependency: impl Into<String>) {
        self.dependencies.push(dependency.into());
    }

    pub fn set_setting(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.settings.insert(key.into(), value.into());
    }

    pub fn execution_model(&self) -> ShellExecutionModel {
        self.capabilities().execution_model.clone()
    }
}
