use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::dependencies::PluginDependency;
use super::permissions::Permission;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginType {
    Native,
    Wasm,
    Script,
    BuiltIn,
}

impl Default for PluginType {
    fn default() -> Self {
        Self::Native
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
}

impl PluginMetadata {
    pub fn new<S: Into<String>>(
        id: S,
        name: S,
        version: S,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            description: None,
            author: None,
            homepage: None,
            license: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub metadata: PluginMetadata,

    #[serde(default)]
    pub plugin_type: PluginType,

    pub entrypoint: Option<PathBuf>,

    #[serde(default)]
    pub permissions: Vec<Permission>,

    #[serde(default)]
    pub dependencies: Vec<PluginDependency>,

    #[serde(default)]
    pub configuration: HashMap<String, String>,
}

impl PluginManifest {
    pub fn id(&self) -> &str {
        &self.metadata.id
    }

    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    pub fn version(&self) -> &str {
        &self.metadata.version
    }

    pub fn plugin_type(&self) -> PluginType {
        self.plugin_type
    }

    pub fn requested_permissions(&self) -> &[Permission] {
        &self.permissions
    }

    pub fn dependencies(&self) -> &[PluginDependency] {
        &self.dependencies
    }

    pub fn entrypoint(&self) -> Option<&PathBuf> {
        self.entrypoint.as_ref()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.metadata.id.trim().is_empty() {
            return Err("plugin ID cannot be empty".to_string());
        }

        if self.metadata.name.trim().is_empty() {
            return Err("plugin name cannot be empty".to_string());
        }

        if self.metadata.version.trim().is_empty() {
            return Err("plugin version cannot be empty".to_string());
        }

        match self.plugin_type {
            PluginType::BuiltIn => {}
            _ if self.entrypoint.is_none() => {
                return Err(format!(
                    "plugin '{}' requires an entrypoint",
                    self.metadata.id
                ));
            }
            _ => {}
        }

        Ok(())
    }
}
