/// GUI security mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiSecurityMode {
    Normal,
    Restricted,
    Safe,
}

/// Security-sensitive GUI features.
#[derive(Debug, Clone)]
pub struct GuiSecuritySettings {
    mode: GuiSecurityMode,
    allow_clipboard_read: bool,
    allow_clipboard_write: bool,
    allow_hyperlinks: bool,
    allow_external_commands: bool,
    confirm_external_links: bool,
    confirm_clipboard_access: bool,
    show_security_warnings: bool,
}

impl Default for GuiSecuritySettings {
    fn default() -> Self {
        Self {
            mode: GuiSecurityMode::Normal,
            allow_clipboard_read: true,
            allow_clipboard_write: true,
            allow_hyperlinks: true,
            allow_external_commands: false,
            confirm_external_links: true,
            confirm_clipboard_access: false,
            show_security_warnings: true,
        }
    }
}

impl GuiSecuritySettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mode(&self) -> GuiSecurityMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: GuiSecurityMode) {
        self.mode = mode;

        match mode {
            GuiSecurityMode::Normal => {}

            GuiSecurityMode::Restricted => {
                self.allow_external_commands = false;
                self.confirm_external_links = true;
                self.confirm_clipboard_access = true;
            }

            GuiSecurityMode::Safe => {
                self.allow_clipboard_read = false;
                self.allow_clipboard_write = false;
                self.allow_hyperlinks = false;
                self.allow_external_commands = false;
                self.confirm_external_links = true;
                self.confirm_clipboard_access = true;
            }
        }
    }

    pub fn allow_clipboard_read(&self) -> bool {
        self.allow_clipboard_read
    }

    pub fn allow_clipboard_write(&self) -> bool {
        self.allow_clipboard_write
    }

    pub fn allow_hyperlinks(&self) -> bool {
        self.allow_hyperlinks
    }

    pub fn allow_external_commands(&self) -> bool {
        self.allow_external_commands
    }

    pub fn confirm_external_links(&self) -> bool {
        self.confirm_external_links
    }

    pub fn confirm_clipboard_access(&self) -> bool {
        self.confirm_clipboard_access
    }

    pub fn show_security_warnings(&self) -> bool {
        self.show_security_warnings
    }

    pub fn set_clipboard_read(&mut self, enabled: bool) {
        self.allow_clipboard_read = enabled;
    }

    pub fn set_clipboard_write(&mut self, enabled: bool) {
        self.allow_clipboard_write = enabled;
    }

    pub fn set_hyperlinks(&mut self, enabled: bool) {
        self.allow_hyperlinks = enabled;
    }

    pub fn set_external_commands(&mut self, enabled: bool) {
        self.allow_external_commands = enabled;
    }

    pub fn set_confirm_external_links(&mut self, enabled: bool) {
        self.confirm_external_links = enabled;
    }

    pub fn set_confirm_clipboard_access(&mut self, enabled: bool) {
        self.confirm_clipboard_access = enabled;
    }

    pub fn set_show_security_warnings(&mut self, enabled: bool) {
        self.show_security_warnings = enabled;
    }
}

/// Security decision presented to the GUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityDecision {
    Allow,
    Deny,
    Confirm,
}

impl GuiSecuritySettings {
    pub fn hyperlink_decision(&self) -> SecurityDecision {
        if !self.allow_hyperlinks {
            SecurityDecision::Deny
        } else if self.confirm_external_links {
            SecurityDecision::Confirm
        } else {
            SecurityDecision::Allow
        }
    }

    pub fn clipboard_read_decision(&self) -> SecurityDecision {
        if !self.allow_clipboard_read {
            SecurityDecision::Deny
        } else if self.confirm_clipboard_access {
            SecurityDecision::Confirm
        } else {
            SecurityDecision::Allow
        }
    }

    pub fn external_command_decision(&self) -> SecurityDecision {
        if self.allow_external_commands {
            SecurityDecision::Allow
        } else {
            SecurityDecision::Deny
        }
    }
}
