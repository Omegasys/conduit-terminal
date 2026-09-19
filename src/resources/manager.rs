use std::path::Path;

use super::discovery::{
    ResourceDiscovery,
    ResourceDiscoveryResult,
};
use super::paths::ResourcePaths;
use super::registry::ResourceRegistry;
use super::resource::{
    Resource,
    ResourceError,
    ResourceId,
    ResourceKind,
};

pub struct ResourceManager {
    paths: ResourcePaths,
    registry: ResourceRegistry,
    discovery: ResourceDiscovery,
}

impl ResourceManager {
    pub fn new(paths: ResourcePaths) -> Self {
        let discovery = ResourceDiscovery::new(paths.clone());

        Self {
            paths,
            registry: ResourceRegistry::new(),
            discovery,
        }
    }

    pub fn initialize(&mut self) -> ResourceDiscoveryResult {
        let _ = self.paths.ensure_directories();

        self.registry.clear();

        self.discovery.sync_registry(&mut self.registry)
    }

    pub fn paths(&self) -> &ResourcePaths {
        &self.paths
    }

    pub fn registry(&self) -> &ResourceRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut ResourceRegistry {
        &mut self.registry
    }

    pub fn discover(&self) -> ResourceDiscoveryResult {
        self.discovery.discover_all()
    }

    pub fn discover_kind(
        &self,
        kind: ResourceKind,
    ) -> ResourceDiscoveryResult {
        self.discovery.discover_kind(kind)
    }

    pub fn register(
        &mut self,
        kind: ResourceKind,
        name: impl Into<String>,
        path: impl Into<std::path::PathBuf>,
    ) -> Result<ResourceId, ResourceError> {
        self.registry.register(kind, name, path)
    }

    pub fn get(&self, id: ResourceId) -> Option<&Resource> {
        self.registry.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: ResourceId,
    ) -> Option<&mut Resource> {
        self.registry.get_mut(id)
    }

    pub fn find_by_path(
        &self,
        path: &Path,
    ) -> Option<&Resource> {
        self.registry.find_by_path(path)
    }

    pub fn find_by_name(
        &self,
        kind: ResourceKind,
        name: &str,
    ) -> Option<&Resource> {
        self.registry.find_by_name(kind, name)
    }

    pub fn remove(
        &mut self,
        id: ResourceId,
    ) -> Result<Resource, ResourceError> {
        self.registry.remove(id)
    }

    pub fn reload(&mut self) -> ResourceDiscoveryResult {
        self.initialize()
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new(ResourcePaths::default())
    }
}
