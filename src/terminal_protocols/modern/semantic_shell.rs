#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellSemanticEventKind {
    PromptStart,
    PromptEnd,
    CommandStart,
    CommandEnd,
    CommandOutput,
    ExitStatus,
    WorkingDirectory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellSemanticEvent {
    pub kind: ShellSemanticEventKind,
    pub value: Option<String>,
}

impl ShellSemanticEvent {
    pub fn new(
        kind: ShellSemanticEventKind,
        value: Option<String>,
    ) -> Self {
        Self { kind, value }
    }

    pub fn prompt_start() -> Self {
        Self::new(ShellSemanticEventKind::PromptStart, None)
    }

    pub fn prompt_end() -> Self {
        Self::new(ShellSemanticEventKind::PromptEnd, None)
    }

    pub fn command_start() -> Self {
        Self::new(ShellSemanticEventKind::CommandStart, None)
    }

    pub fn command_end() -> Self {
        Self::new(ShellSemanticEventKind::CommandEnd, None)
    }

    pub fn exit_status(status: i32) -> Self {
        Self::new(
            ShellSemanticEventKind::ExitStatus,
            Some(status.to_string()),
        )
    }

    pub fn working_directory(path: impl Into<String>) -> Self {
        Self::new(
            ShellSemanticEventKind::WorkingDirectory,
            Some(path.into()),
        )
    }
}
