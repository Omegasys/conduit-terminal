use super::permissions::SecurityPermission;
use super::sandbox::{SandboxPolicy, SandboxViolation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeSequenceType {
    WindowTitle,
    Hyperlink,
    Clipboard,
    ShellCommand,
    FileOperation,
    TerminalMode,
    Color,
    Cursor,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeAction {
    Allow,
    Ask,
    Block,
}

#[derive(Debug, Clone)]
pub struct EscapeSequenceSecurityPolicy {
    allow_window_title: bool,
    allow_hyperlinks: bool,
    allow_clipboard: bool,
    allow_shell_commands: bool,
    allow_file_operations: bool,
    allow_terminal_modes: bool,
    require_confirmation: bool,
}

impl Default for EscapeSequenceSecurityPolicy {
    fn default() -> Self {
        Self {
            allow_window_title: true,
            allow_hyperlinks: true,
            allow_clipboard: false,
            allow_shell_commands: false,
            allow_file_operations: false,
            allow_terminal_modes: true,
            require_confirmation: true,
        }
    }
}

impl EscapeSequenceSecurityPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_allow_window_title(&mut self, value: bool) {
        self.allow_window_title = value;
    }

    pub fn set_allow_hyperlinks(&mut self, value: bool) {
        self.allow_hyperlinks = value;
    }

    pub fn set_allow_clipboard(&mut self, value: bool) {
        self.allow_clipboard = value;
    }

    pub fn set_allow_shell_commands(&mut self, value: bool) {
        self.allow_shell_commands = value;
    }

    pub fn set_allow_file_operations(&mut self, value: bool) {
        self.allow_file_operations = value;
    }

    pub fn set_allow_terminal_modes(&mut self, value: bool) {
        self.allow_terminal_modes = value;
    }

    pub fn set_require_confirmation(&mut self, value: bool) {
        self.require_confirmation = value;
    }

    pub fn check(
        &self,
        sequence_type: EscapeSequenceType,
        sandbox: &SandboxPolicy,
    ) -> Result<EscapeAction, SandboxViolation> {
        let allowed = match sequence_type {
            EscapeSequenceType::WindowTitle => {
                sandbox.can(SecurityPermission::ChangeWindowTitle)?;
                self.allow_window_title
            }

            EscapeSequenceType::Hyperlink => {
                sandbox.can(SecurityPermission::OpenHyperlink)?;
                self.allow_hyperlinks
            }

            EscapeSequenceType::Clipboard => {
                sandbox.can(SecurityPermission::ClipboardWrite)?;
                self.allow_clipboard
            }

            EscapeSequenceType::ShellCommand => {
                sandbox.can(SecurityPermission::ExecuteEscapeAction)?;
                self.allow_shell_commands
            }

            EscapeSequenceType::FileOperation => {
                sandbox.can(SecurityPermission::OpenFile)?;
                self.allow_file_operations
            }

            EscapeSequenceType::TerminalMode => {
                sandbox.can(SecurityPermission::SetTerminalProperty)?;
                self.allow_terminal_modes
            }

            EscapeSequenceType::Color
            | EscapeSequenceType::Cursor => true,

            EscapeSequenceType::Unknown => false,
        };

        if !allowed {
            return Ok(EscapeAction::Block);
        }

        if self.require_confirmation {
            match sequence_type {
                EscapeSequenceType::Clipboard
                | EscapeSequenceType::ShellCommand
                | EscapeSequenceType::FileOperation
                | EscapeSequenceType::Unknown => {
                    return Ok(EscapeAction::Ask);
                }

                _ => {}
            }
        }

        Ok(EscapeAction::Allow)
    }
}

#[derive(Debug, Clone)]
pub struct EscapeSequenceSecurity {
    policy: EscapeSequenceSecurityPolicy,
}

impl EscapeSequenceSecurity {
    pub fn new(
        policy: EscapeSequenceSecurityPolicy,
    ) -> Self {
        Self { policy }
    }

    pub fn policy(&self) -> &EscapeSequenceSecurityPolicy {
        &self.policy
    }

    pub fn policy_mut(
        &mut self,
    ) -> &mut EscapeSequenceSecurityPolicy {
        &mut self.policy
    }

    pub fn classify(
        sequence: &[u8],
    ) -> EscapeSequenceType {
        if sequence.is_empty() {
            return EscapeSequenceType::Unknown;
        }

        let text = String::from_utf8_lossy(sequence);

        if text.contains("1337;") || text.contains("8;;") {
            return EscapeSequenceType::Hyperlink;
        }

        if text.contains("52;") {
            return EscapeSequenceType::Clipboard;
        }

        if text.contains("]0;")
            || text.contains("]1;")
            || text.contains("]2;")
        {
            return EscapeSequenceType::WindowTitle;
        }

        if text.contains("!#")
            || text.contains("shell")
            || text.contains("command")
        {
            return EscapeSequenceType::ShellCommand;
        }

        if text.contains("file:")
            || text.contains("File=")
        {
            return EscapeSequenceType::FileOperation;
        }

        if text.contains("[?")
            || text.contains("[>")
        {
            return EscapeSequenceType::TerminalMode;
        }

        if text.contains("[")
            && text.contains("m")
        {
            return EscapeSequenceType::Color;
        }

        if text.contains("[")
            && (text.contains('H') || text.contains('f'))
        {
            return EscapeSequenceType::Cursor;
        }

        EscapeSequenceType::Unknown
    }

    pub fn evaluate(
        &self,
        sequence: &[u8],
        sandbox: &SandboxPolicy,
    ) -> Result<EscapeAction, SandboxViolation> {
        let kind = Self::classify(sequence);

        self.policy.check(kind, sandbox)
    }
}

impl Default for EscapeSequenceSecurity {
    fn default() -> Self {
        Self::new(EscapeSequenceSecurityPolicy::default())
    }
}
