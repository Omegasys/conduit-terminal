use std::path::PathBuf;
use std::time::SystemTime;

use crate::resources::{ResourceId, ResourceKind};

/// Unique identifier for an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EventId(u64);

impl EventId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

/// Origin of an event.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventSource {
    System,
    Core,
    Gui,
    Tui,
    Cli,
    Terminal,
    Window,
    Tab,
    Pane,
    Workspace,
    Configuration,
    Resource,
    Plugin,
    User,
    External(String),
}

/// High-level category of an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventCategory {
    Application,
    Window,
    Tab,
    Pane,
    Workspace,
    Terminal,
    Input,
    Output,
    Configuration,
    Resource,
    Plugin,
    Session,
    Notification,
    Security,
    System,
    Custom,
}

/// Payload carried by an event.
#[derive(Debug, Clone, PartialEq)]
pub enum EventPayload {
    None,
    Text(String),
    Bytes(Vec<u8>),
    Path(PathBuf),
    Resource(ResourceId),
    ResourceChanged {
        resource: ResourceId,
        kind: ResourceKind,
    },
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

/// A complete event flowing through Conduit's event bus.
#[derive(Debug, Clone)]
pub struct Event {
    id: EventId,
    category: EventCategory,
    source: EventSource,
    name: String,
    payload: EventPayload,
    timestamp: SystemTime,
    propagation_stopped: bool,
}

impl Event {
    pub fn new(
        id: EventId,
        category: EventCategory,
        source: EventSource,
        name: impl Into<String>,
        payload: EventPayload,
    ) -> Self {
        Self {
            id,
            category,
            source,
            name: name.into(),
            payload,
            timestamp: SystemTime::now(),
            propagation_stopped: false,
        }
    }

    pub fn id(&self) -> EventId {
        self.id
    }

    pub fn category(&self) -> EventCategory {
        self.category
    }

    pub fn source(&self) -> &EventSource {
        &self.source
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn payload(&self) -> &EventPayload {
        &self.payload
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn is_propagation_stopped(&self) -> bool {
        self.propagation_stopped
    }

    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }

    pub fn with_payload(mut self, payload: EventPayload) -> Self {
        self.payload = payload;
        self
    }
}
