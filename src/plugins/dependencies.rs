use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyKind {
    Required,
    Optional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub id: String,
    pub version: String,

    #[serde(default)]
    pub kind: DependencyKind,
}

impl PluginDependency {
    pub fn required<S: Into<String>>(
        id: S,
        version: S,
    ) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            kind: DependencyKind::Required,
        }
    }

    pub fn optional<S: Into<String>>(
        id: S,
        version: S,
    ) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            kind: DependencyKind::Optional,
        }
    }
}

impl Default for DependencyKind {
    fn default() -> Self {
        Self::Required
    }
}

#[derive(Debug, Clone)]
pub struct DependencyResolution {
    pub plugin_id: String,
    pub dependencies: Vec<PluginDependency>,
    pub missing_required: Vec<String>,
    pub missing_optional: Vec<String>,
}

impl DependencyResolution {
    pub fn is_satisfied(&self) -> bool {
        self.missing_required.is_empty()
    }
}

#[derive(Debug, Default)]
pub struct DependencyResolver {
    versions: HashMap<String, String>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<S: Into<String>>(
        &mut self,
        id: S,
        version: S,
    ) {
        self.versions.insert(id.into(), version.into());
    }

    pub fn unregister(&mut self, id: &str) {
        self.versions.remove(id);
    }

    pub fn version(&self, id: &str) -> Option<&str> {
        self.versions.get(id).map(String::as_str)
    }

    pub fn resolve(
        &self,
        plugin_id: &str,
        dependencies: &[PluginDependency],
    ) -> DependencyResolution {
        let mut missing_required = Vec::new();
        let mut missing_optional = Vec::new();

        for dependency in dependencies {
            let present = self.versions.contains_key(&dependency.id);

            if present {
                continue;
            }

            match dependency.kind {
                DependencyKind::Required => {
                    missing_required.push(dependency.id.clone());
                }
                DependencyKind::Optional => {
                    missing_optional.push(dependency.id.clone());
                }
            }
        }

        DependencyResolution {
            plugin_id: plugin_id.to_string(),
            dependencies: dependencies.to_vec(),
            missing_required,
            missing_optional,
        }
    }

    pub fn clear(&mut self) {
        self.versions.clear();
    }
}
