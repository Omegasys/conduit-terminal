pub mod discovery;
pub mod loader;
pub mod manifest;
pub mod registry;
pub mod watcher;

pub use discovery::{
    ShellDiscovery,
    ShellDiscoveryResult,
};

pub use loader::{
    ShellResourceLoader,
    ShellResourceLoadResult,
};

pub use manifest::{
    ShellResourceManifest,
    ShellResourceMetadata,
};

pub use registry::{
    ShellResourceEntry,
    ShellResourceRegistry,
};

pub use watcher::{
    ShellResourceChange,
    ShellResourceWatcher,
};
