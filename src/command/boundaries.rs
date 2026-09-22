use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandBoundaryKind {
    Started,
    Finished,
    Prompt,
    Cancelled,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct CommandBoundary {
    command_id: u64,
    kind: CommandBoundaryKind,
    timestamp: Instant,
    offset: Option<usize>,
}

impl CommandBoundary {
    pub fn new(command_id: u64, kind: CommandBoundaryKind) -> Self {
        Self {
            command_id,
            kind,
            timestamp: Instant::now(),
            offset: None,
        }
    }

    pub fn command_id(&self) -> u64 {
        self.command_id
    }

    pub fn kind(&self) -> CommandBoundaryKind {
        self.kind
    }

    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }

    pub fn offset(&self) -> Option<usize> {
        self.offset
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

#[derive(Debug, Default)]
pub struct CommandBoundaryDetector {
    current_command: Option<u64>,
    boundaries: Vec<CommandBoundary>,
}

impl CommandBoundaryDetector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, command_id: u64) -> CommandBoundary {
        self.current_command = Some(command_id);

        let boundary =
            CommandBoundary::new(command_id, CommandBoundaryKind::Started);

        self.boundaries.push(boundary.clone());
        boundary
    }

    pub fn finish(&mut self, command_id: u64) -> CommandBoundary {
        if self.current_command == Some(command_id) {
            self.current_command = None;
        }

        let boundary =
            CommandBoundary::new(command_id, CommandBoundaryKind::Finished);

        self.boundaries.push(boundary.clone());
        boundary
    }

    pub fn prompt(&mut self, command_id: u64) -> CommandBoundary {
        let boundary =
            CommandBoundary::new(command_id, CommandBoundaryKind::Prompt);

        self.boundaries.push(boundary.clone());
        boundary
    }

    pub fn cancel(&mut self, command_id: u64) -> CommandBoundary {
        if self.current_command == Some(command_id) {
            self.current_command = None;
        }

        let boundary =
            CommandBoundary::new(command_id, CommandBoundaryKind::Cancelled);

        self.boundaries.push(boundary.clone());
        boundary
    }

    pub fn current_command(&self) -> Option<u64> {
        self.current_command
    }

    pub fn boundaries(&self) -> &[CommandBoundary] {
        &self.boundaries
    }

    pub fn clear(&mut self) {
        self.current_command = None;
        self.boundaries.clear();
    }

    pub fn last(&self) -> Option<&CommandBoundary> {
        self.boundaries.last()
    }
}
