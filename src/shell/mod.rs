pub mod bash;
pub mod fish;
pub mod integration;
pub mod zsh;

pub use bash::BashIntegration;
pub use fish::FishIntegration;
pub use integration::{
    ShellCommand,
    ShellEnvironment,
    ShellIntegration,
    ShellIntegrationManager,
    ShellIntegrationResult,
};
pub use zsh::ZshIntegration;
pub mod bash;
pub mod command_boundaries;
pub mod command_duration;
pub mod command_status;
pub mod directory;
pub mod environment;
pub mod fish;
pub mod history;
pub mod integration;
pub mod powershell;
pub mod prompt;
pub mod zsh;

pub use bash::BashIntegration;

pub use command_boundaries::{
    CommandBoundary,
    CommandBoundaryEvent,
    CommandBoundaryTracker,
};

pub use command_duration::CommandDuration;

pub use command_status::{
    CommandStatus,
    CommandStatusInfo,
};

pub use directory::{
    normalize_directory,
    DirectoryState,
};

pub use environment::{
    EnvironmentChange,
    EnvironmentState,
};

pub use fish::FishIntegration;

pub use history::{
    HistoryEntry,
    ShellHistory,
};

pub use integration::{
    ShellCommand,
    ShellEnvironment,
    ShellIntegration,
    ShellIntegrationManager,
    ShellIntegrationResult,
};

pub use powershell::PowerShellIntegration;

pub use prompt::{
    PromptDetector,
    PromptInfo,
    PromptState,
};

pub use zsh::ZshIntegration;
pub mod custom;

pub use custom::{
    CustomCommand,
    CustomCommandResult,
    CustomPrompt,
    CustomPromptState,
    CustomShell,
    CustomShellCapabilities,
    CustomShellConfiguration,
    CustomShellEnvironment,
    CustomShellError,
    CustomShellHistory,
    CustomShellId,
    CustomShellLoader,
    CustomShellManifest,
    CustomShellMetadata,
    CustomShellParser,
    CustomShellRegistry,
    CustomShellSecurityPolicy,
    CustomShellValidator,
    CustomShellVersion,
    EnvironmentOperation,
    LoadedCustomShell,
    ShellColorSupport,
    ShellExecutionModel,
    ShellPermission,
    ShellRegistration,
    ShellValidationIssue,
    ShellValidationResult,
};
