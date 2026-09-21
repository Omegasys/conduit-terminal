pub mod discovery;
pub mod manager;
pub mod paths;
pub mod registry;
pub mod resource;

pub mod shells;

pub use shells::{
    ShellDiscovery,
    ShellDiscoveryResult,
    ShellResourceChange,
    ShellResourceEntry,
    ShellResourceLoadResult,
    ShellResourceLoader,
    ShellResourceManifest,
    ShellResourceMetadata,
    ShellResourceRegistry,
    ShellResourceWatcher,
};

pub use discovery::{
    ResourceDiscovery,
    ResourceDiscoveryEvent,
    ResourceDiscoveryResult,
};

pub use manager::ResourceManager;

pub use paths::ResourcePaths;

pub use registry::ResourceRegistry;

pub use resource::{
    Resource,
    ResourceError,
    ResourceId,
    ResourceKind,
    ResourceMetadata,
    ResourceState,
};
pub mod discovery;
pub mod manager;
pub mod manifest;
pub mod paths;
pub mod profile;
pub mod registry;
pub mod resource;
pub mod theme;
pub mod validation;
pub mod workspace;

pub use manifest::ResourceManifest;
pub use profile::{Profile, ProfileValue};
pub use theme::{Theme, ThemeColors, ThemePalette};
pub use validation::{
    ResourceValidator,
    ValidationIssue,
    ValidationResult,
    ValidationSeverity,
};
pub use workspace::{
    WorkspaceLayout,
    WorkspaceResource,
    WorkspaceStartup,
};
