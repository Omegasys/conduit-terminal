#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandStatus {
    Unknown,
    Running,
    Success,
    Failed,
    Signaled,
    Interrupted,
}

impl CommandStatus {
    pub fn from_exit_code(code: i32) -> Self {
        if code == 0 {
            Self::Success
        } else {
            Self::Failed
        }
    }

    pub fn is_finished(&self) -> bool {
        matches!(
            self,
            Self::Success
                | Self::Failed
                | Self::Signaled
                | Self::Interrupted
        )
    }

    pub fn succeeded(&self) -> bool {
        matches!(self, Self::Success)
    }

    pub fn failed(&self) -> bool {
        matches!(
            self,
            Self::Failed
                | Self::Signaled
                | Self::Interrupted
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandStatusInfo {
    status: CommandStatus,
    exit_code: Option<i32>,
    signal: Option<i32>,
}

impl CommandStatusInfo {
    pub fn new(status: CommandStatus) -> Self {
        Self {
            status,
            exit_code: None,
            signal: None,
        }
    }

    pub fn status(&self) -> CommandStatus {
        self.status
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn signal(&self) -> Option<i32> {
        self.signal
    }

    pub fn set_exit_code(&mut self, code: i32) {
        self.exit_code = Some(code);
        self.status = CommandStatus::from_exit_code(code);
    }

    pub fn set_signal(&mut self, signal: i32) {
        self.signal = Some(signal);
        self.status = CommandStatus::Signaled;
    }

    pub fn set_status(&mut self, status: CommandStatus) {
        self.status = status;
    }

    pub fn succeeded(&self) -> bool {
        self.status.succeeded()
    }

    pub fn failed(&self) -> bool {
        self.status.failed()
    }
}

impl Default for CommandStatusInfo {
    fn default() -> Self {
        Self::new(CommandStatus::Unknown)
    }
}
