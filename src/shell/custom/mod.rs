//! Custom shell support.
//!
//! Custom shells allow users and plugins to describe shells that are not
//! built into Conduit. The custom-shell system is intentionally modular:
//! manifests describe the shell, configuration controls how it is launched,
//! and optional adapters provide parsing, prompt, history, and environment
//! integration.

pub mod capabilities;
pub mod commands;
pub mod configuration;
pub mod environment;
pub mod errors;
pub mod example;
pub mod history;
pub mod loader;
pub mod manifest;
pub mod parser;
pub mod prompt;
pub mod registry;
pub mod security;
pub mod shell;
pub mod validation;

pub use capabilities::CustomCapabilities;
pub use commands::{CustomCommand, CustomCommandRegistry};
pub use configuration::CustomShellConfiguration;
pub use environment::CustomEnvironment;
pub use errors::{CustomShellError, CustomShellResult};
pub use history::{CustomHistory, CustomHistoryEntry};
pub use loader::CustomShellLoader;
pub use manifest::CustomShellManifest;
pub use parser::{CustomCommandParser, CustomParseResult};
pub use prompt::{CustomPrompt, CustomPromptIntegration};
pub use registry::CustomShellRegistry;
pub use security::CustomShellSecurityPolicy;
pub use shell::CustomShell;
pub use validation::CustomShellValidator;
