//! Relationships between flow nodes.

use std::fmt;

use super::node::FlowNodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeId(u64);

impl EdgeId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for EdgeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "edge-{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowEdgeKind {
    ProcessToPty,
    PtyToTerminal,
    TerminalToPane,
    ProcessToProcess,
    ProcessToTerminal,
    TerminalToTerminal,
    PaneToPane,
    Event,
    Input,
    Output,
    Control,
    Data,
}

#[derive(Debug, Clone)]
pub struct FlowEdge {
    id: EdgeId,
    source: FlowNodeId,
    target: FlowNodeId,
    kind: FlowEdgeKind,
    label: Option<String>,
    enabled: bool,
}

impl FlowEdge {
    pub fn new(
        id: EdgeId,
        source: FlowNodeId,
        target: FlowNodeId,
        kind: FlowEdgeKind,
    ) -> Self {
        Self {
            id,
            source,
            target,
            kind,
            label: None,
            enabled: true,
        }
    }

    pub fn id(&self) -> EdgeId {
        self.id
    }

    pub fn source(&self) -> FlowNodeId {
        self.source
    }

    pub fn target(&self) -> FlowNodeId {
        self.target
    }

    pub fn kind(&self) -> FlowEdgeKind {
        self.kind
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = Some(label.into());
    }

    pub fn clear_label(&mut self) {
        self.label = None;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn connects(&self, node: FlowNodeId) -> bool {
        self.source == node || self.target == node
    }
}
