use super::nodes::FlowNodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeId(u64);

impl EdgeId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowEdgeKind {
    Data,
    Input,
    Output,
    Event,
    Command,
    Process,
    Control,
    Dependency,
    Plugin,
    Network,
    File,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowEdgeStyle {
    Solid,
    Dashed,
    Dotted,
    Animated,
}

#[derive(Debug, Clone)]
pub struct FlowEdge {
    id: EdgeId,
    source: FlowNodeId,
    target: FlowNodeId,
    kind: FlowEdgeKind,
    style: FlowEdgeStyle,
    label: Option<String>,
    active: bool,
    bidirectional: bool,
}

impl FlowEdge {
    pub fn new(
        source: FlowNodeId,
        target: FlowNodeId,
        kind: FlowEdgeKind,
    ) -> Self {
        Self {
            id: EdgeId::new(0),
            source,
            target,
            kind,
            style: FlowEdgeStyle::Solid,
            label: None,
            active: true,
            bidirectional: false,
        }
    }

    pub(crate) fn set_id(&mut self, id: EdgeId) {
        self.id = id;
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

    pub fn style(&self) -> FlowEdgeStyle {
        self.style
    }

    pub fn set_style(&mut self, style: FlowEdgeStyle) {
        self.style = style;
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn bidirectional(&self) -> bool {
        self.bidirectional
    }

    pub fn set_bidirectional(&mut self, bidirectional: bool) {
        self.bidirectional = bidirectional;
    }
}
