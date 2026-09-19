use std::fs;
use std::path::{Path, PathBuf};

use super::paths::ResourcePaths;
use super::resource::{
    Resource,
    ResourceError,
    ResourceKind,
};
use super::registry::ResourceRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceDiscoveryEvent {
    Added,
    Modified,
    Removed,
}

#[derive(Debug, Default)]
pub struct ResourceDiscoveryResult {
    pub discovered: Vec<Resource>,
    pub ignored: Vec<PathBuf>,
    pub errors: Vec<ResourceError>,
}

impl ResourceDiscoveryResult {
    pub fn is_empty(&self) -> bool {
        self.discovered.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

pub struct ResourceDiscovery {
    paths: ResourcePaths,
}

impl ResourceDiscovery {
    pub fn new(paths: ResourcePaths) -> Self {
        Self { paths }
    }

    pub fn paths(&self) -> &ResourcePaths {
        &self.paths
    }

    pub fn discover_all(&self) -> ResourceDiscoveryResult {
        let mut result = ResourceDiscoveryResult::default();

        for kind in [
            ResourceKind::Theme,
            ResourceKind::Profile,
            ResourceKind::Workspace,
            ResourceKind::Plugin,
            ResourceKind::Layout,
            ResourceKind::Keybinding,
            ResourceKind::Extension,
        ] {
            let directory = self.paths.kind_dir(kind);

            if !directory.exists() {
                continue;
            }

            self.discover_directory(
                &directory,
                kind,
                &mut result,
            );
        }

        result
    }

    pub fn discover_kind(
        &self,
        kind: ResourceKind,
    ) -> ResourceDiscoveryResult {
        let mut result = ResourceDiscoveryResult::default();
        let directory = self.paths.kind_dir(kind);

        if directory.exists() {
            self.discover_directory(
                &directory,
                kind,
                &mut result,
            );
        }

        result
    }

    pub fn discover_directory(
        &self,
        directory: &Path,
        kind: ResourceKind,
        result: &mut ResourceDiscoveryResult,
    ) {
        let entries = match fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(error) => {
                result.errors.push(ResourceError::Io(error));
                return;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    result.errors.push(ResourceError::Io(error));
                    continue;
                }
            };

            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if !self.accepts_file(&path, kind) {
                result.ignored.push(path);
                continue;
            }

            match self.resource_from_path(path, kind) {
                Ok(resource) => result.discovered.push(resource),
                Err(error) => result.errors.push(error),
            }
        }
    }

    pub fn sync_registry(
        &self,
        registry: &mut ResourceRegistry,
    ) -> ResourceDiscoveryResult {
        let result = self.discover_all();

        for resource in &result.discovered {
            if registry.find_by_path(resource.path()).is_none() {
                let _ = registry.insert(resource.clone());
            }
        }

        result
    }

    fn accepts_file(
        &self,
        path: &Path,
        kind: ResourceKind,
    ) -> bool {
        match kind.extension() {
            Some(extension) => {
                path.extension()
                    .and_then(|value| value.to_str())
                    .map(|value| value.eq_ignore_ascii_case(extension))
                    .unwrap_or(false)
            }

            None => true,
        }
    }

    fn resource_from_path(
        &self,
        path: PathBuf,
        kind: ResourceKind,
    ) -> Result<Resource, ResourceError> {
        let name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| ResourceError::InvalidPath(path.clone()))?;

        Ok(Resource::new(
            super::resource::ResourceId::new(0),
            kind,
            name,
            path,
        ))
    }
}
