use std::fmt;

/// Exit status of a command executed by a shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandStatus {
    code: i32,
}

impl CommandStatus {
    /// Creates a normal process exit status.
    pub const fn exited(code: i32) -> Self {
        Self { code }
    }

    /// Creates a successful status.
    pub const fn success() -> Self {
        Self::exited(0)
    }

    /// Creates a failed status.
    pub const fn failure() -> Self {
        Self::exited(1)
    }

    /// Returns the numeric exit code.
    pub const fn code(self) -> i32 {
        self.code
    }

    /// Returns true when the command exited successfully.
    pub const fn is_success(self) -> bool {
        self.code == 0
    }

    /// Returns true when the command failed.
    pub const fn is_failure(self) -> bool {
        self.code != 0
    }
}

impl Default for CommandStatus {
    fn default() -> Self {
        Self::success()
    }
}

impl From<i32> for CommandStatus {
    fn from(code: i32) -> Self {
        Self::exited(code)
    }
}

impl From<CommandStatus> for i32 {
    fn from(status: CommandStatus) -> Self {
        status.code
    }
}

impl fmt::Display for CommandStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

/// Tracks the most recent command status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandStatusTracker {
    current: CommandStatus,
    previous: Option<CommandStatus>,
    command_count: u64,
}

impl CommandStatusTracker {
    pub fn new() -> Self {
        Self {
            current: CommandStatus::success(),
            previous: None,
            command_count: 0,
        }
    }

    pub fn current(&self) -> CommandStatus {
        self.current
    }

    pub fn previous(&self) -> Option<CommandStatus> {
        self.previous
    }

    pub fn command_count(&self) -> u64 {
        self.command_count
    }

    /// Records the result of a completed command.
    pub fn record(&mut self, status: CommandStatus) {
        self.previous = Some(self.current);
        self.current = status;
        self.command_count = self.command_count.saturating_add(1);
    }

    pub fn reset(&mut self) {
        self.current = CommandStatus::success();
        self.previous = None;
        self.command_count = 0;
    }
}

impl Default for CommandStatusTracker {
    fn default() -> Self {
        Self::new()
    }
}
