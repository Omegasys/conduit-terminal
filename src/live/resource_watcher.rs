use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::resources::{ResourceId, ResourceKind, ResourceManager};

use super::watcher::{ResourceWatcher as FileWatcher, WatchEvent, WatchEventKind};

/// A resource-oriented filesystem event.
#[derive(Debug, Clone)]
pub enum ResourceWatchEvent {
    Added {
        resource: Option<ResourceId>,
        path: PathBuf,
    },
    Modified {
        resource: Option<ResourceId>,
        path: PathBuf,
    },
    Removed {
        resource: Option<ResourceId>,
        path: PathBuf,
    },
    Renamed {
        resource: Option<ResourceId>,
        old_path: PathBuf,
        new_path: PathBuf,
    },
}

impl ResourceWatchEvent {
    pub fn path(&self) -> &Path {
        match self {
            Self::Added { path, .. }
            | Self::Modified { path, .. }
            | Self::Removed { path, .. } => path,
            Self::Renamed { new_path, .. } => new_path,
        }
    }

    pub fn resource_id(&self) -> Option<ResourceId> {
        match self {
            Self::Added { resource, .. }
            | Self::Modified { resource, .. }
            | Self::Removed { resource, .. }
            | Self::Renamed { resource, .. } => *resource,
        }
    }
}

/// Watches resource directories and resolves filesystem events
/// against Conduit's resource registry.
#[derive(Debug)]
pub struct ResourceWatcher {
    watcher: FileWatcher,
    paths: HashMap<PathBuf, ResourceId>,
}

impl Default for ResourceWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceWatcher {
    pub fn new() -> Self {
        Self {
            watcher: FileWatcher::new(),
            paths: HashMap::new(),
        }
    }

    pub fn add_root(&mut self, path: impl Into<PathBuf>) {
        self.watcher.add_root(path);
    }

    pub fn remove_root(&mut self, path: &Path) {
        self.watcher.remove_root(path);
    }

    pub fn roots(&self) -> &[PathBuf] {
        self.watcher.roots()
    }

    pub fn register_resource(&mut self, resource_id: ResourceId, path: impl Into<PathBuf>) {
        self.paths.insert(path.into(), resource_id);
    }

    pub fn unregister_resource(&mut self, path: &Path) -> Option<ResourceId> {
        self.paths.remove(path)
    }

    pub fn resolve(&self, path: &Path) -> Option<ResourceId> {
        self.paths.get(path).copied()
    }

    pub fn sync_registry(&mut self, manager: &ResourceManager) {
        self.paths.clear();

        for resource in manager.registry().iter() {
            self.paths
                .insert(resource.path().to_path_buf(), resource.id());
        }
    }

    pub fn emit(&self, event: WatchEvent) {
        self.watcher.emit(event);
    }

    pub fn poll(&self) -> Vec<ResourceWatchEvent> {
        self.watcher
            .poll()
            .into_iter()
            .map(|event| self.resolve_event(event))
            .collect()
    }

    fn resolve_event(&self, event: WatchEvent) -> ResourceWatchEvent {
        match event.kind {
            WatchEventKind::Created => ResourceWatchEvent::Added {
                resource: self.resolve(&event.path),
                path: event.path,
            },

            WatchEventKind::Modified => ResourceWatchEvent::Modified {
                resource: self.resolve(&event.path),
                path: event.path,
            },

            WatchEventKind::Removed => ResourceWatchEvent::Removed {
                resource: self.resolve(&event.path),
                path: event.path,
            },

            WatchEventKind::Renamed => {
                let resource = event
                    .previous_path
                    .as_deref()
                    .and_then(|path| self.resolve(path))
                    .or_else(|| self.resolve(&event.path));

                ResourceWatchEvent::Renamed {
                    resource,
                    old_path: event.previous_path.unwrap_or_default(),
                    new_path: event.path,
                }
            }
        }
    }

    pub fn is_watched(&self, path: &Path) -> bool {
        self.watcher.is_watched(path)
    }

    pub fn tracked_resources(&self) -> usize {
        self.paths.len()
    }
}

/// Describes a resource watcher target.
#[derive(Debug, Clone)]
pub struct ResourceWatchTarget {
    pub resource_id: ResourceId,
    pub kind: ResourceKind,
    pub path: PathBuf,
    pub modified: Option<SystemTime>,
}

impl ResourceWatchTarget {
    pub fn new(
        resource_id: ResourceId,
        kind: ResourceKind,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            resource_id,
            kind,
            path: path.into(),
            modified: None,
        }
    }
}
