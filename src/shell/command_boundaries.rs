use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandBoundary {
    Start,
    End,
}

#[derive(Clone, Debug)]
pub struct CommandBoundaryEvent {
    boundary: CommandBoundary,
    timestamp: Instant,
}

impl CommandBoundaryEvent {
    pub fn start() -> Self {
        Self {
            boundary: CommandBoundary::Start,
            timestamp: Instant::now(),
        }
    }

    pub fn end() -> Self {
        Self {
            boundary: CommandBoundary::End,
            timestamp: Instant::now(),
        }
    }

    pub fn boundary(&self) -> CommandBoundary {
        self.boundary
    }

    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }

    pub fn is_start(&self) -> bool {
        matches!(self.boundary, CommandBoundary::Start)
    }

    pub fn is_end(&self) -> bool {
        matches!(self.boundary, CommandBoundary::End)
    }
}

#[derive(Clone, Debug)]
pub struct CommandBoundaryTracker {
    active: bool,
    last_start: Option<Instant>,
    last_end: Option<Instant>,
}

impl CommandBoundaryTracker {
    pub fn new() -> Self {
        Self {
            active: false,
            last_start: None,
            last_end: None,
        }
    }

    pub fn begin(&mut self) -> CommandBoundaryEvent {
        let event = CommandBoundaryEvent::start();

        self.active = true;
        self.last_start = Some(event.timestamp());

        event
    }

    pub fn finish(&mut self) -> CommandBoundaryEvent {
        let event = CommandBoundaryEvent::end();

        self.active = false;
        self.last_end = Some(event.timestamp());

        event
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn last_start(&self) -> Option<Instant> {
        self.last_start
    }

    pub fn last_end(&self) -> Option<Instant> {
        self.last_end
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.last_start = None;
        self.last_end = None;
    }
}

impl Default for CommandBoundaryTracker {
    fn default() -> Self {
        Self::new()
    }
}
