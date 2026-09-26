use std::time::Duration;

use super::metadata::RecordingMetadata;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordingEventKind {
    Output,
    Input,
    Resize {
        columns: u16,
        rows: u16,
    },
    Bell,
    TitleChanged(String),
    WorkingDirectoryChanged(String),
    Marker(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordingEvent {
    timestamp: Duration,
    kind: RecordingEventKind,
    data: Vec<u8>,
}

impl RecordingEvent {
    pub fn new(
        timestamp: Duration,
        kind: RecordingEventKind,
        data: Vec<u8>,
    ) -> Self {
        Self {
            timestamp,
            kind,
            data,
        }
    }

    pub fn output(
        timestamp: Duration,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::new(
            timestamp,
            RecordingEventKind::Output,
            data.into(),
        )
    }

    pub fn input(
        timestamp: Duration,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::new(
            timestamp,
            RecordingEventKind::Input,
            data.into(),
        )
    }

    pub fn resize(
        timestamp: Duration,
        columns: u16,
        rows: u16,
    ) -> Self {
        Self::new(
            timestamp,
            RecordingEventKind::Resize { columns, rows },
            Vec::new(),
        )
    }

    pub fn bell(timestamp: Duration) -> Self {
        Self::new(
            timestamp,
            RecordingEventKind::Bell,
            Vec::new(),
        )
    }

    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }

    pub fn kind(&self) -> &RecordingEventKind {
        &self.kind
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Debug, Clone)]
pub struct RecordingFile {
    version: u32,
    metadata: RecordingMetadata,
    events: Vec<RecordingEvent>,
}

impl RecordingFile {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn new(metadata: RecordingMetadata) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            metadata,
            events: Vec::new(),
        }
    }

    pub fn with_events(
        metadata: RecordingMetadata,
        events: Vec<RecordingEvent>,
    ) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            metadata,
            events,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn metadata(&self) -> &RecordingMetadata {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut RecordingMetadata {
        &mut self.metadata
    }

    pub fn events(&self) -> &[RecordingEvent] {
        &self.events
    }

    pub fn events_mut(&mut self) -> &mut Vec<RecordingEvent> {
        &mut self.events
    }

    pub fn push(&mut self, event: RecordingEvent) {
        self.events.push(event);
    }

    pub fn duration(&self) -> Duration {
        self.events
            .last()
            .map(|event| event.timestamp())
            .unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
