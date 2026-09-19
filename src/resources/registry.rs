use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::resource::{
    Resource,
    ResourceError,
    ResourceId,
    ResourceKind,
};

#[derive(Debug, Default)]
pub struct ResourceRegistry {
    resources: HashMap<ResourceId, Resource>,
    paths: HashMap<PathBuf, ResourceId>,
    next_id: u64,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            paths: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn register(
        &mut self,
        kind: ResourceKind,
        name: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Result<ResourceId, ResourceError> {
        let path = path.into();

        if let Some(existing) = self.paths.get(&path) {
            return Err(ResourceError::AlreadyRegistered(*existing));
        }

        let id = ResourceId::new(self.next_id);
        self.next_id += 1;

        let resource = Resource::new(
            id,
            kind,
            name,
            path.clone(),
        );

        self.resources.insert(id, resource);
        self.paths.insert(path, id);

        Ok(id)
    }

    pub fn insert(
        &mut self,
        resource: Resource,
    ) -> Result<ResourceId, ResourceError> {
        let id = resource.id();
        let path = resource.path().to_path_buf();

        if self.resources.contains_key(&id) {
            return Err(ResourceError::AlreadyRegistered(id));
        }

        if let Some(existing) = self.paths.get(&path) {
            return Err(ResourceError::AlreadyRegistered(*existing));
        }

        self.next_id = self.next_id.max(id.value() + 1);

        self.resources.insert(id, resource);
        self.paths.insert(path, id);

        Ok(id)
    }

    pub fn get(&self, id: ResourceId) -> Option<&Resource> {
        self.resources.get(&id)
    }

    pub fn get_mut(&mut self, id: ResourceId) -> Option<&mut Resource> {
        self.resources.get_mut(&id)
    }

    pub fn find_by_path(&self, path: &Path) -> Option<&Resource> {
        let id = self.paths.get(path)?;
        self.resources.get(id)
    }

    pub fn find_by_name(
        &self,
        kind: ResourceKind,
        name: &str,
    ) -> Option<&Resource> {
        self.resources.values().find(|resource| {
            resource.kind() == kind && resource.name() == name
        })
    }

    pub fn remove(
        &mut self,
        id: ResourceId,
    ) -> Result<Resource, ResourceError> {
        let resource = self
            .resources
            .remove(&id)
            .ok_or(ResourceError::NotFound(id))?;

        self.paths.remove(resource.path());

        Ok(resource)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Resource> {
        self.resources.values()
    }

    pub fn iter_kind(
        &self,
        kind: ResourceKind,
    ) -> impl Iterator<Item = &Resource> {
        self.resources
            .values()
            .filter(move |resource| resource.kind() == kind)
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn clear(&mut self) {
        self.resources.clear();
        self.paths.clear();
    }
}
