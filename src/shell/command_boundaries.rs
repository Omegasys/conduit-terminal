/// The type of boundary detected in a shell command stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandBoundary {
    Start,
    End,
    Prompt,
    Continuation,
    Unknown,
}

/// A boundary marker detected in terminal/shell output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryEvent {
    pub boundary: CommandBoundary,
    pub sequence: u64,
    pub marker: Option<String>,
}

impl BoundaryEvent {
    pub fn new(boundary: CommandBoundary, sequence: u64) -> Self {
        Self {
            boundary,
            sequence,
            marker: None,
        }
    }

    pub fn with_marker(
        boundary: CommandBoundary,
        sequence: u64,
        marker: impl Into<String>,
    ) -> Self {
        Self {
            boundary,
            sequence,
            marker: Some(marker.into()),
        }
    }
}

/// Tracks command boundaries independently of any specific shell.
#[derive(Debug, Clone)]
pub struct CommandBoundaryTracker {
    sequence: u64,
    command_active: bool,
    last_boundary: Option<CommandBoundary>,
}

impl CommandBoundaryTracker {
    pub fn new() -> Self {
        Self {
            sequence: 0,
            command_active: false,
            last_boundary: None,
        }
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn command_active(&self) -> bool {
        self.command_active
    }

    pub fn last_boundary(&self) -> Option<CommandBoundary> {
        self.last_boundary
    }

    pub fn start(&mut self) -> BoundaryEvent {
        self.sequence = self.sequence.saturating_add(1);
        self.command_active = true;
        self.last_boundary = Some(CommandBoundary::Start);

        BoundaryEvent::new(CommandBoundary::Start, self.sequence)
    }

    pub fn end(&mut self) -> BoundaryEvent {
        self.sequence = self.sequence.saturating_add(1);
        self.command_active = false;
        self.last_boundary = Some(CommandBoundary::End);

        BoundaryEvent::new(CommandBoundary::End, self.sequence)
    }

    pub fn prompt(&mut self) -> BoundaryEvent {
        self.sequence = self.sequence.saturating_add(1);
        self.command_active = false;
        self.last_boundary = Some(CommandBoundary::Prompt);

        BoundaryEvent::new(CommandBoundary::Prompt, self.sequence)
    }

    pub fn continuation(&mut self) -> BoundaryEvent {
        self.sequence = self.sequence.saturating_add(1);
        self.command_active = true;
        self.last_boundary = Some(CommandBoundary::Continuation);

        BoundaryEvent::new(CommandBoundary::Continuation, self.sequence)
    }

    pub fn reset(&mut self) {
        self.sequence = 0;
        self.command_active = false;
        self.last_boundary = None;
    }
}

impl Default for CommandBoundaryTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Attempts to recognize simple Conduit integration markers.
#[derive(Debug, Clone)]
pub struct BoundaryParser {
    command_marker: String,
    prompt_marker: String,
    continuation_marker: String,
}

impl BoundaryParser {
    pub fn new(
        command_marker: impl Into<String>,
        prompt_marker: impl Into<String>,
        continuation_marker: impl Into<String>,
    ) -> Self {
        Self {
            command_marker: command_marker.into(),
            prompt_marker: prompt_marker.into(),
            continuation_marker: continuation_marker.into(),
        }
    }

    pub fn parse(&self, input: &str) -> Option<CommandBoundary> {
        if input.contains(&self.command_marker) {
            Some(CommandBoundary::Start)
        } else if input.contains(&self.prompt_marker) {
            Some(CommandBoundary::Prompt)
        } else if input.contains(&self.continuation_marker) {
            Some(CommandBoundary::Continuation)
        } else {
            None
        }
    }
}
