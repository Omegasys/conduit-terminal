use std::time::SystemTime;

use super::panes::FlowPaneId;
use super::pty::PtyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalEventKind {
    PtyCreated,
    PtyStarted,
    PtyOutput,
    PtyInput,
    PtyResized,
    PtyExited,
    PtyFailed,
    PaneCreated,
    PaneActivated,
    PaneClosed,
    PaneResized,
    PaneFocused,
    PaneUnfocused,
    RenderRequested,
    RenderCompleted,
    SelectionChanged,
    CursorMoved,
    Custom,
}

#[derive(Debug, Clone)]
pub enum TerminalEventPayload {
    None,
    Text(String),
    Bytes(Vec<u8>),
    Pty(PtyId),
    Pane(FlowPaneId),
    Resize {
        columns: u16,
        rows: u16,
    },
    Cursor {
        row: usize,
        column: usize,
    },
}

#[derive(Debug, Clone)]
pub struct TerminalEvent {
    id: u64,
    kind: TerminalEventKind,
    timestamp: SystemTime,
    pane_id: Option<FlowPaneId>,
    pty_id: Option<PtyId>,
    payload: TerminalEventPayload,
}

impl TerminalEvent {
    pub fn new(
        id: u64,
        kind: TerminalEventKind,
    ) -> Self {
        Self {
            id,
            kind,
            timestamp: SystemTime::now(),
            pane_id: None,
            pty_id: None,
            payload: TerminalEventPayload::None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn kind(&self) -> TerminalEventKind {
        self.kind
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn pane_id(&self) -> Option<FlowPaneId> {
        self.pane_id
    }

    pub fn set_pane_id(&mut self, id: Option<FlowPaneId>) {
        self.pane_id = id;
    }

    pub fn pty_id(&self) -> Option<PtyId> {
        self.pty_id
    }

    pub fn set_pty_id(&mut self, id: Option<PtyId>) {
        self.pty_id = id;
    }

    pub fn payload(&self) -> &TerminalEventPayload {
        &self.payload
    }

    pub fn set_payload(&mut self, payload: TerminalEventPayload) {
        self.payload = payload;
    }
}

#[derive(Debug, Default)]
pub struct TerminalEventManager {
    events: Vec<TerminalEvent>,
    next_id: u64,
    maximum_events: usize,
}

impl Default for TerminalEventManager {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            next_id: 1,
            maximum_events: 10_000,
        }
    }
}

impl TerminalEventManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit(
        &mut self,
        kind: TerminalEventKind,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.events.push(TerminalEvent::new(id, kind));

        if self.events.len() > self.maximum_events {
            let excess = self.events.len() - self.maximum_events;
            self.events.drain(0..excess);
        }

        id
    }

    pub fn emit_event(&mut self, event: TerminalEvent) {
        self.events.push(event);

        if self.events.len() > self.maximum_events {
            let excess = self.events.len() - self.maximum_events;
            self.events.drain(0..excess);
        }
    }

    pub fn get(&self, id: u64) -> Option<&TerminalEvent> {
        self.events.iter().find(|event| event.id() == id)
    }

    pub fn events(&self) -> impl Iterator<Item = &TerminalEvent> {
        self.events.iter()
    }

    pub fn by_kind(
        &self,
        kind: TerminalEventKind,
    ) -> impl Iterator<Item = &TerminalEvent> {
        self.events
            .iter()
            .filter(move |event| event.kind() == kind)
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn set_maximum_events(&mut self, maximum: usize) {
        self.maximum_events = maximum.max(1);

        if self.events.len() > self.maximum_events {
            let excess = self.events.len() - self.maximum_events;
            self.events.drain(0..excess);
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
