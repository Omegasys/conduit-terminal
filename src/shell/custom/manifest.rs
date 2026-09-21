use std::collections::BTreeMap;

use super::{
    capabilities::CustomShellCapabilities,
    shell::{CustomShellId, CustomShellVersion},
};

#[derive(Clone, Debug)]
pub struct CustomShellMetadata {
    pub author: String,
    pub description: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
}

impl CustomShellMetadata {
    pub fn new<S1, S2>(
        author: S1,
        description: S2,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            author: author.into(),
            description: description.into(),
            homepage: None,
            license: None,
        }
    }

    pub fn homepage<S: Into<String>>(
        mut self,
        value: S,
    ) -> Self {
        self.homepage = Some(value.into());
        self
    }

    pub fn license<S: Into<String>>(
        mut self,
        value: S,
    ) -> Self {
        self.license = Some(value.into());
        self
    }
}

#[derive(Clone, Debug)]
pub struct CustomShellManifest {
    id: CustomShellId,
    name: String,
    version: CustomShellVersion,
    metadata: CustomShellMetadata,
    capabilities: CustomShellCapabilities,
    executable: Option<String>,
    entrypoint: Option<String>,
    dependencies: Vec<String>,
    settings: BTreeMap<String, String>,
}

impl CustomShellManifest {
    pub fn new<S: Into<String>>(
        id: CustomShellId,
        name: S,
        metadata: CustomShellMetadata,
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
        }
    }

    pub fn id(&self) -> &CustomShellId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &CustomShellVersion {
        &self.version
    }

    pub fn metadata(&self) -> &CustomShellMetadata {
        &self.metadata
    }

    pub fn capabilities(
        &self,
    ) -> &CustomShellCapabilities {
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

    pub fn settings(
        &self,
    ) -> &BTreeMap<String, String> {
        &self.settings
    }

    pub fn set_version(
        &mut self,
        version: CustomShellVersion,
    ) {
        self.version = version;
    }

    pub fn set_capabilities(
        &mut self,
        capabilities: CustomShellCapabilities,
    ) {
        self.capabilities = capabilities;
    }

    pub fn set_executable<S: Into<String>>(
        &mut self,
        executable: S,
    ) {
        self.executable = Some(executable.into());
    }

    pub fn set_entrypoint<S: Into<String>>(
        &mut self,
        entrypoint: S,
    ) {
        self.entrypoint = Some(entrypoint.into());
    }

    pub fn add_dependency<S: Into<String>>(
        &mut self,
        dependency: S,
    ) {
        let dependency = dependency.into();

        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }

    pub fn set_setting<S1, S2>(
        &mut self,
        key: S1,
        value: S2,
    ) where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.settings.insert(
            key.into(),
            value.into(),
        );
    }
}
