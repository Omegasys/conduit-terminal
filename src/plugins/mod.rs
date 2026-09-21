pub mod api;
pub mod dependencies;
pub mod lifecycle;
pub mod loader;
pub mod manager;
pub mod manifest;
pub mod permissions;
pub mod registry;
pub mod sandbox;
pub mod updates;

pub use api::{
    Plugin,
    PluginContext,
    PluginEvent,
    PluginId,
    PluginResult,
};

pub use dependencies::{
    DependencyKind,
    PluginDependency,
    DependencyResolver,
    DependencyResolution,
};

pub use lifecycle::{
    LifecycleManager,
    LifecycleState,
};

pub use loader::{
    LoadedPlugin,
    PluginLoader,
    PluginSource,
};

pub use manager::{
    PluginManager,
    PluginManagerError,
};

pub use manifest::{
    PluginManifest,
    PluginMetadata,
    PluginType,
};

pub use permissions::{
    Permission,
    PermissionSet,
    PermissionRequest,
};

pub use registry::{
    PluginRegistry,
    PluginRegistration,
};

pub use sandbox::{
    SandboxConfig,
    SandboxPolicy,
};

pub use updates::{
    PluginUpdate,
    PluginUpdateManager,
    UpdateChannel,
};
