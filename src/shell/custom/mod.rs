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

pub use capabilities::{
    CustomShellCapabilities,
    ShellColorSupport,
    ShellExecutionModel,
};

pub use commands::{
    CustomCommand,
    CustomCommandResult,
};

pub use configuration::{
    CustomShellConfiguration,
    CustomShellOption,
};

pub use environment::{
    CustomShellEnvironment,
    EnvironmentOperation,
};

pub use errors::{
    CustomShellError,
    CustomShellResult,
};

pub use history::{
    CustomHistoryEntry,
    CustomShellHistory,
};

pub use loader::{
    CustomShellLoader,
    LoadedCustomShell,
};

pub use manifest::{
    CustomShellManifest,
    CustomShellMetadata,
};

pub use parser::{
    CustomShellParser,
    ShellParseResult,
};

pub use prompt::{
    CustomPrompt,
    CustomPromptState,
};

pub use registry::{
    CustomShellRegistry,
    ShellRegistration,
};

pub use security::{
    CustomShellSecurityPolicy,
    ShellPermission,
};

pub use shell::{
    CustomShell,
    CustomShellId,
    CustomShellVersion,
};

pub use validation::{
    CustomShellValidator,
    ShellValidationIssue,
    ShellValidationResult,
};
