use std::fmt;

use super::{
    capabilities::CustomShellCapabilities,
    commands::{CustomCommand, CustomCommandResult},
    configuration::CustomShellConfiguration,
    environment::CustomShellEnvironment,
    errors::CustomShellResult,
    manifest::CustomShellManifest,
    parser::ShellParseResult,
    prompt::CustomPrompt,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CustomShellId(String);

impl CustomShellId {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for CustomShellId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for CustomShellId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CustomShellId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomShellVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl CustomShellVersion {
    pub const fn new(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub const fn initial() -> Self {
        Self::new(1, 0, 0)
    }
}

impl Default for CustomShellVersion {
    fn default() -> Self {
        Self::initial()
    }
}

impl fmt::Display for CustomShellVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

pub trait CustomShell: Send {
    fn id(&self) -> CustomShellId;

    fn version(&self) -> CustomShellVersion {
        CustomShellVersion::initial()
    }

    fn manifest(&self) -> CustomShellManifest;

    fn capabilities(&self) -> CustomShellCapabilities;

    fn configuration(&self) -> &CustomShellConfiguration;

    fn configuration_mut(&mut self) -> &mut CustomShellConfiguration;

    fn environment(&self) -> &CustomShellEnvironment;

    fn environment_mut(&mut self) -> &mut CustomShellEnvironment;

    fn prompt(&self) -> &CustomPrompt;

    fn reset(&mut self);

    fn initialize(&mut self) -> CustomShellResult<()>;

    fn parse_input(
        &mut self,
        input: &[u8],
    ) -> CustomShellResult<ShellParseResult>;

    fn parse_output(
        &mut self,
        output: &[u8],
    ) -> CustomShellResult<ShellParseResult>;

    fn execute_command(
        &mut self,
        command: CustomCommand,
    ) -> CustomShellResult<CustomCommandResult>;
}
