//! Security policy for custom shells.
//!
//! Custom shells are executable code and therefore receive an explicit
//! security policy. The policy can later be connected to Conduit's central
//! security subsystem.

use super::errors::{CustomShellError, CustomShellResult};

#[derive(Debug, Clone)]
pub struct CustomShellSecurityPolicy {
    allow_process_spawn: bool,
    allow_filesystem_read: bool,
    allow_filesystem_write: bool,
    allow_environment_access: bool,
    allow_terminal_control: bool,
    allow_clipboard: bool,
    allow_escape_sequences: bool,
    allow_network_access: bool,
}

impl Default for CustomShellSecurityPolicy {
    fn default() -> Self {
        Self {
            allow_process_spawn: true,
            allow_filesystem_read: true,
            allow_filesystem_write: true,
            allow_environment_access: true,
            allow_terminal_control: true,
            allow_clipboard: false,
            allow_escape_sequences: true,
            allow_network_access: true,
        }
    }
}

impl CustomShellSecurityPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hardened() -> Self {
        Self {
            allow_process_spawn: true,
            allow_filesystem_read: true,
            allow_filesystem_write: false,
            allow_environment_access: false,
            allow_terminal_control: true,
            allow_clipboard: false,
            allow_escape_sequences: false,
            allow_network_access: false,
        }
    }

    pub fn allow_process_spawn(&self) -> bool {
        self.allow_process_spawn
    }

    pub fn allow_filesystem_read(&self) -> bool {
        self.allow_filesystem_read
    }

    pub fn allow_filesystem_write(&self) -> bool {
        self.allow_filesystem_write
    }

    pub fn allow_environment_access(&self) -> bool {
        self.allow_environment_access
    }

    pub fn allow_terminal_control(&self) -> bool {
        self.allow_terminal_control
    }

    pub fn allow_clipboard(&self) -> bool {
        self.allow_clipboard
    }

    pub fn allow_escape_sequences(&self) -> bool {
        self.allow_escape_sequences
    }

    pub fn allow_network_access(&self) -> bool {
        self.allow_network_access
    }

    pub fn set_process_spawn(&mut self, value: bool) {
        self.allow_process_spawn = value;
    }

    pub fn set_filesystem_read(&mut self, value: bool) {
        self.allow_filesystem_read = value;
    }

    pub fn set_filesystem_write(&mut self, value: bool) {
        self.allow_filesystem_write = value;
    }

    pub fn set_environment_access(&mut self, value: bool) {
        self.allow_environment_access = value;
    }

    pub fn set_terminal_control(&mut self, value: bool) {
        self.allow_terminal_control = value;
    }

    pub fn set_clipboard(&mut self, value: bool) {
        self.allow_clipboard = value;
    }

    pub fn set_escape_sequences(&mut self, value: bool) {
        self.allow_escape_sequences = value;
    }

    pub fn set_network_access(&mut self, value: bool) {
        self.allow_network_access = value;
    }

    pub fn validate(&self) -> CustomShellResult<()> {
        if !self.allow_process_spawn
            && !self.allow_terminal_control
            && !self.allow_filesystem_read
            && !self.allow_environment_access
        {
            return Err(CustomShellError::InvalidSecurityPolicy(
                "security policy disables every useful shell capability".into(),
            ));
        }

        Ok(())
    }
}
