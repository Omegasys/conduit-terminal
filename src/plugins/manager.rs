use std::collections::HashMap;

use super::api::{
    Plugin,
    PluginContext,
    PluginEvent,
    PluginId,
};
use super::dependencies::DependencyResolver;
use super::lifecycle::LifecycleManager;
use super::loader::{LoadedPlugin, PluginLoader};
use super::permissions::PermissionSet;
use super::registry::{PluginRegistration, PluginRegistry};
use super::sandbox::SandboxPolicy;
use super::updates::PluginUpdateManager;

#[derive(Debug)]
pub enum PluginManagerError {
    PluginNotFound(String),
    PluginDisabled(String),
    PluginAlreadyLoaded(String),
    InvalidManifest(String),
    DependencyFailure(String),
    PermissionDenied(String),
    LifecycleFailure(String),
}

impl std::fmt::Display for PluginManagerError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::PluginNotFound(id) => {
                write!(f, "plugin '{}' not found", id)
            }
            Self::PluginDisabled(id) => {
                write!(f, "plugin '{}' is disabled", id)
            }
            Self::PluginAlreadyLoaded(id) => {
                write!(f, "plugin '{}' is already loaded", id)
            }
            Self::InvalidManifest(error) => {
                write!(f, "invalid plugin manifest: {}", error)
            }
            Self::DependencyFailure(error) => {
                write!(f, "plugin dependency failure: {}", error)
            }
            Self::PermissionDenied(error) => {
                write!(f, "plugin permission denied: {}", error)
            }
            Self::LifecycleFailure(error) => {
                write!(f, "plugin lifecycle failure: {}", error)
            }
        }
    }
}

impl std::error::Error for PluginManagerError {}

pub struct PluginManager {
    loader: PluginLoader,
    registry: PluginRegistry,
    lifecycle: LifecycleManager,
    dependencies: DependencyResolver,
    sandbox: SandboxPolicy,
    updates: PluginUpdateManager,
    loaded: HashMap<PluginId, LoadedPlugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            loader: PluginLoader::new(),
            registry: PluginRegistry::new(),
            lifecycle: LifecycleManager::new(),
            dependencies: DependencyResolver::new(),
            sandbox: SandboxPolicy::default(),
            updates: PluginUpdateManager::new(),
            loaded: HashMap::new(),
        }
    }

    pub fn loader(&self) -> &PluginLoader {
        &self.loader
    }

    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.registry
    }

    pub fn lifecycle(&self) -> &LifecycleManager {
        &self.lifecycle
    }

    pub fn dependencies(&self) -> &DependencyResolver {
        &self.dependencies
    }

    pub fn dependencies_mut(&mut self) -> &mut DependencyResolver {
        &mut self.dependencies
    }

    pub fn sandbox(&self) -> &SandboxPolicy {
        &self.sandbox
    }

    pub fn sandbox_mut(&mut self) -> &mut SandboxPolicy {
        &mut self.sandbox
    }

    pub fn updates(&self) -> &PluginUpdateManager {
        &self.updates
    }

    pub fn updates_mut(&mut self) -> &mut PluginUpdateManager {
        &mut self.updates
    }

    pub fn register(
        &mut self,
        manifest: super::manifest::PluginManifest,
    ) -> Result<PluginId, PluginManagerError> {
        manifest
            .validate()
            .map_err(PluginManagerError::InvalidManifest)?;

        let id = PluginId::new(manifest.id().to_string());

        let registration = PluginRegistration::new(manifest.clone());

        self.dependencies
            .register(manifest.id(), manifest.version());

        self.lifecycle.register(id.clone());
        self.registry.register(registration);

        Ok(id)
    }

    pub fn load(
        &mut self,
        loaded: LoadedPlugin,
    ) -> Result<PluginId, PluginManagerError> {
        let id = PluginId::new(loaded.manifest.id().to_string());

        if self.loaded.contains_key(&id) {
            return Err(PluginManagerError::PluginAlreadyLoaded(
                id.to_string(),
            ));
        }

        let resolution = self.dependencies.resolve(
            loaded.manifest.id(),
            loaded.manifest.dependencies(),
        );

        if !resolution.is_satisfied() {
            return Err(PluginManagerError::DependencyFailure(
                resolution
                    .missing_required
                    .join(", "),
            ));
        }

        let requested = PermissionSet::from_permissions(
            loaded
                .manifest
                .requested_permissions()
                .iter()
                .copied(),
        );

        let granted = self.sandbox.filter_permissions(&requested);

        if granted.len() != requested.len() {
            return Err(PluginManagerError::PermissionDenied(
                format!(
                    "plugin '{}' requested permissions that are not allowed by the sandbox",
                    id
                ),
            ));
        }

        self.loaded.insert(id.clone(), loaded);

        if let Some(registration) = self.registry.get_mut(&id) {
            registration.state =
                super::lifecycle::LifecycleState::Loaded;
        }

        Ok(id)
    }

    pub fn initialize(
        &mut self,
        id: &PluginId,
    ) -> Result<(), PluginManagerError> {
        let plugin = self
            .loaded
            .get_mut(id)
            .ok_or_else(|| {
                PluginManagerError::PluginNotFound(id.to_string())
            })?;

        let permissions = PermissionSet::from_permissions(
            plugin
                .manifest
                .requested_permissions()
                .iter()
                .copied(),
        );

        let mut context =
            PluginContext::new(id.clone(), permissions);

        let instance = plugin.instance.as_mut().ok_or_else(|| {
            PluginManagerError::LifecycleFailure(
                format!("plugin '{}' has no runtime instance", id),
            )
        })?;

        self.lifecycle
            .initialize(instance.as_mut(), &mut context)
            .map_err(PluginManagerError::LifecycleFailure)
    }

    pub fn dispatch(
        &mut self,
        event: &PluginEvent,
    ) -> Vec<(PluginId, Result<(), String>)> {
        let ids: Vec<PluginId> = self
            .loaded
            .keys()
            .cloned()
            .collect();

        let mut results = Vec::new();

        for id in ids {
            let Some(plugin) = self.loaded.get_mut(&id) else {
                continue;
            };

            let Some(instance) = plugin.instance.as_mut() else {
                continue;
            };

            let permissions = PermissionSet::from_permissions(
                plugin
                    .manifest
                    .requested_permissions()
                    .iter()
                    .copied(),
            );

            let mut context =
                PluginContext::new(id.clone(), permissions);

            let result = self
                .lifecycle
                .dispatch(instance.as_mut(), event, &mut context);

            results.push((id, result));
        }

        results
    }

    pub fn shutdown(
        &mut self,
        id: &PluginId,
    ) -> Result<(), PluginManagerError> {
        let plugin = self
            .loaded
            .get_mut(id)
            .ok_or_else(|| {
                PluginManagerError::PluginNotFound(id.to_string())
            })?;

        let instance = plugin.instance.as_mut().ok_or_else(|| {
            PluginManagerError::LifecycleFailure(
                format!("plugin '{}' has no runtime instance", id),
            )
        })?;

        let permissions = PermissionSet::from_permissions(
            plugin
                .manifest
                .requested_permissions()
                .iter()
                .copied(),
        );

        let mut context =
            PluginContext::new(id.clone(), permissions);

        self.lifecycle
            .shutdown(instance.as_mut(), &mut context)
            .map_err(PluginManagerError::LifecycleFailure)
    }

    pub fn unload(
        &mut self,
        id: &PluginId,
    ) -> Result<LoadedPlugin, PluginManagerError> {
        if !self.loaded.contains_key(id) {
            return Err(PluginManagerError::PluginNotFound(
                id.to_string(),
            ));
        }

        self.lifecycle
            .remove(id);

        self.loaded
            .remove(id)
            .ok_or_else(|| {
                PluginManagerError::PluginNotFound(id.to_string())
            })
    }

    pub fn is_loaded(&self, id: &PluginId) -> bool {
        self.loaded.contains_key(id)
    }

    pub fn loaded_plugins(
        &self,
    ) -> impl Iterator<Item = (&PluginId, &LoadedPlugin)> {
        self.loaded.iter()
    }

    pub fn clear(&mut self) {
        self.loaded.clear();
        self.registry.clear();
        self.lifecycle.clear();
        self.dependencies.clear();
        self.updates.clear();
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
