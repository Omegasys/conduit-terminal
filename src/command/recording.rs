use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandRecordingEventKind {
    CommandStarted,
    Input,
    Output,
    CommandFinished,
    Marker,
}

#[derive(Debug, Clone)]
pub struct CommandRecordingEvent {
    timestamp: Instant,
    kind: CommandRecordingEventKind,
    data: Vec<u8>,
}

impl CommandRecordingEvent {
    pub fn new(
        kind: CommandRecordingEventKind,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            timestamp: Instant::now(),
            kind,
            data: data.into(),
        }
    }

    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }

    pub fn kind(&self) -> CommandRecordingEventKind {
        self.kind
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Debug, Clone)]
pub struct CommandRecording {
    id: u64,
    command: String,
    started_at: Instant,
    finished_at: Option<Instant>,
    events: Vec<CommandRecordingEvent>,
}

impl CommandRecording {
    pub fn new(id: u64, command: impl Into<String>) -> Self {
        Self {
            id,
            command: command.into(),
            started_at: Instant::now(),
            finished_at: None,
            events: Vec::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn started_at(&self) -> Instant {
        self.started_at
    }

    pub fn finished_at(&self) -> Option<Instant> {
        self.finished_at
    }

    pub fn duration(&self) -> Duration {
        match self.finished_at {
            Some(end) => end.duration_since(self.started_at),
            None => self.started_at.elapsed(),
        }
    }

    pub fn events(&self) -> &[CommandRecordingEvent] {
        &self.events
    }

    pub fn record(
        &mut self,
        kind: CommandRecordingEventKind,
        data: impl Into<Vec<u8>>,
    ) {
        self.events.push(CommandRecordingEvent::new(kind, data));
    }

    pub fn finish(&mut self) {
        self.finished_at = Some(Instant::now());
    }

    pub fn is_finished(&self) -> bool {
        self.finished_at.is_some()
    }
}

#[derive(Debug, Default)]
pub struct CommandRecorder {
    active: Option<CommandRecording>,
    next_id: u64,
}

impl CommandRecorder {
    pub fn new() -> Self {
        Self {
            active: None,
            next_id: 1,
        }
    }

    pub fn start(&mut self, command: impl Into<String>) -> Option<u64> {
        if self.active.is_some() {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        self.active = Some(CommandRecording::new(id, command));

        Some(id)
    }

    pub fn record(
        &mut self,
        kind: CommandRecordingEventKind,
        data: impl Into<Vec<u8>>,
    ) -> bool {
        let Some(recording) = self.active.as_mut() else {
            return false;
        };

        recording.record(kind, data);
        true
    }

    pub fn finish(&mut self) -> Option<CommandRecording> {
        let mut recording = self.active.take()?;
        recording.finish();
        Some(recording)
    }

    pub fn active(&self) -> Option<&CommandRecording> {
        self.active.as_ref()
    }

    pub fn is_recording(&self) -> bool {
        self.active.is_some()
    }

    pub fn cancel(&mut self) -> Option<CommandRecording> {
        self.active.take()
    }
}
