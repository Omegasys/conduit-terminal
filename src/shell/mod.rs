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
