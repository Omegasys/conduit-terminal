//! Flow graph nodes.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FlowNodeId(u64);

impl FlowNodeId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for FlowNodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "node-{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowNodeKind {
    Process,
    Pty,
    Terminal,
    Pane,
}

#[derive(Debug, Clone)]
pub enum FlowNode {
    Process(super::process::FlowProcess),
    Pty(super::pty::FlowPty),
    Terminal(super::terminal::FlowTerminal),
    Pane(super::pane::FlowPane),
}

impl FlowNode {
    pub fn id(&self) -> FlowNodeId {
        match self {
            Self::Process(node) => node.id(),
            Self::Pty(node) => node.id(),
            Self::Terminal(node) => node.id(),
            Self::Pane(node) => node.id(),
        }
    }

    pub fn kind(&self) -> FlowNodeKind {
        match self {
            Self::Process(_) => FlowNodeKind::Process,
            Self::Pty(_) => FlowNodeKind::Pty,
            Self::Terminal(_) => FlowNodeKind::Terminal,
            Self::Pane(_) => FlowNodeKind::Pane,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Process(node) => node.label(),
            Self::Pty(node) => node.label(),
            Self::Terminal(node) => node.label(),
            Self::Pane(node) => node.label(),
        }
    }
}
