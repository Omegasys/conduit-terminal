//! Conduit's shell integration subsystem.
//!
//! The shell subsystem provides a common abstraction over different command
//! shells. Individual shell adapters live in family-specific modules and
//! implement the common capabilities defined here.

pub mod capabilities;
pub mod configuration;
pub mod detection;
pub mod directory;
pub mod environment;
pub mod integration;
pub mod manifest;
pub mod prompt;
pub mod registry;

pub use capabilities::{
    CapabilityLevel, ShellCapabilities, ShellCapability,
};
pub use configuration::{ShellConfiguration, ShellLaunchMode};
pub use detection::{DetectedShell, ShellDetector, ShellKind};
pub use directory::{DirectoryTracker, WorkingDirectory};
pub use environment::{Environment, EnvironmentChange};
pub use integration::{
    ShellError, ShellEvent, ShellId, ShellIntegration, ShellState,
};
pub use manifest::{ShellManifest, ShellManifestError};
pub use prompt::{PromptIntegration, PromptMode};
pub use registry::{ShellRegistry, ShellRegistryError};

/// Result type used by the shell subsystem.
pub type ShellResult<T> = Result<T, ShellError>;
