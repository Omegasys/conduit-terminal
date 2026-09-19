use std::collections::{
    HashMap,
    HashSet,
};

use crate::resources::ResourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DependencyNode {
    pub resource: ResourceId,
}

impl DependencyNode {
    pub fn new(resource: ResourceId) -> Self {
        Self { resource }
    }
}

#[derive(Debug, Default)]
pub struct DependencyGraph {
    dependencies: HashMap<ResourceId, HashSet<ResourceId>>,
    dependents: HashMap<ResourceId, HashSet<ResourceId>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_dependency(
        &mut self,
        resource: ResourceId,
        dependency: ResourceId,
    ) {
        self.dependencies
            .entry(resource)
            .or_default()
            .insert(dependency);

        self.dependents
            .entry(dependency)
            .or_default()
            .insert(resource);
    }

    pub fn remove_dependency(
        &mut self,
        resource: ResourceId,
        dependency: ResourceId,
    ) {
        if let Some(values) = self.dependencies.get_mut(&resource) {
            values.remove(&dependency);

            if values.is_empty() {
                self.dependencies.remove(&resource);
            }
        }

        if let Some(values) = self.dependents.get_mut(&dependency) {
            values.remove(&resource);

            if values.is_empty() {
                self.dependents.remove(&dependency);
            }
        }
    }

    pub fn dependencies_of(
        &self,
        resource: ResourceId,
    ) -> impl Iterator<Item = &ResourceId> {
        self.dependencies
            .get(&resource)
            .into_iter()
            .flat_map(|values| values.iter())
    }

    pub fn dependents_of(
        &self,
        resource: ResourceId,
    ) -> impl Iterator<Item = &ResourceId> {
        self.dependents
            .get(&resource)
            .into_iter()
            .flat_map(|values| values.iter())
    }

    pub fn affected_by(
        &self,
        resource: ResourceId,
    ) -> Vec<ResourceId> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = vec![resource];

        while let Some(current) = queue.pop() {
            if !visited.insert(current) {
                continue;
            }

            result.push(current);

            if let Some(dependents) = self.dependents.get(&current) {
                queue.extend(dependents.iter().copied());
            }
        }

        result
    }

    pub fn remove_resource(
        &mut self,
        resource: ResourceId,
    ) {
        if let Some(dependencies) = self.dependencies.remove(&resource) {
            for dependency in dependencies {
                if let Some(dependents) =
                    self.dependents.get_mut(&dependency)
                {
                    dependents.remove(&resource);
                }
            }
        }

        if let Some(dependents) = self.dependents.remove(&resource) {
            for dependent in dependents {
                if let Some(dependencies) =
                    self.dependencies.get_mut(&dependent)
                {
                    dependencies.remove(&resource);
                }
            }
        }
    }

    pub fn contains(&self, resource: ResourceId) -> bool {
        self.dependencies.contains_key(&resource)
            || self.dependents.contains_key(&resource)
    }

    pub fn clear(&mut self) {
        self.dependencies.clear();
        self.dependents.clear();
    }
}
