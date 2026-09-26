//! Example custom shell definition.
//!
//! This is intentionally an example factory rather than a shell that is
//! automatically registered. Applications, plugins, and tests can use it
//! as a starting point for their own custom shell manifests.

use super::{
    capabilities::CustomCapabilities,
    manifest::CustomShellManifest,
};

use crate::shell::capabilities::CapabilityLevel;

/// Create an example custom shell manifest.
pub fn example_manifest() -> CustomShellManifest {
    let mut capabilities = CustomCapabilities::default();

    capabilities.interactive = CapabilityLevel::Full;
    capabilities.scripting = CapabilityLevel::Full;
    capabilities.structured_output = CapabilityLevel::Basic;
    capabilities.programmable_prompt = CapabilityLevel::Full;
    capabilities.prompt_hooks = CapabilityLevel::Full;
    capabilities.command_hooks = CapabilityLevel::Full;
    capabilities.directory_hooks = CapabilityLevel::Full;
    capabilities.native_history = CapabilityLevel::Basic;
    capabilities.completion = CapabilityLevel::Full;
    capabilities.job_control = CapabilityLevel::Basic;
    capabilities.aliases = CapabilityLevel::Full;
    capabilities.functions = CapabilityLevel::Full;
    capabilities.environment_modification = CapabilityLevel::Full;
    capabilities.terminal_title = CapabilityLevel::Full;
    capabilities.working_directory_reporting = CapabilityLevel::Full;
    capabilities.command_status_reporting = CapabilityLevel::Full;
    capabilities.command_duration_reporting = CapabilityLevel::Basic;
    capabilities.signal_handling = CapabilityLevel::Basic;
    capabilities.startup_files = CapabilityLevel::Full;
    capabilities.configurable_rc_file = CapabilityLevel::Full;

    CustomShellManifest::new(
        "example-shell",
        "Example Shell",
        "example-shell",
    )
    .with_version("0.1.0")
    .with_description(
        "Example custom shell integration for Conduit.",
    )
    .with_capabilities(capabilities)
    .with_argument("--interactive")
}

/// Create an example manifest using the current user's shell.
pub fn current_shell_manifest() -> Option<CustomShellManifest> {
    let executable = std::env::var("SHELL").ok()?;

    Some(
        CustomShellManifest::new(
            "current-shell",
            "Current Shell",
            executable,
        )
        .with_description(
            "A dynamically generated custom shell manifest.",
        ),
    )
}
