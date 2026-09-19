pub mod discovery;
pub mod manager;
pub mod paths;
pub mod registry;
pub mod resource;

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
