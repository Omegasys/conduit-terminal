//! Filtering support for flow graphs.

use super::{
    edge::FlowEdgeKind,
    node::{FlowNode, FlowNodeKind},
};

#[derive(Debug, Clone, Default)]
pub struct FlowFilter {
    node_kind: Option<FlowNodeKind>,
    label_contains: Option<String>,
    enabled_only: bool,
}

impl FlowFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn node_kind(mut self, kind: FlowNodeKind) -> Self {
        self.node_kind = Some(kind);
        self
    }

    pub fn label_contains(mut self, text: impl Into<String>) -> Self {
        self.label_contains = Some(text.into());
        self
    }

    pub fn enabled_only(mut self, enabled: bool) -> Self {
        self.enabled_only = enabled;
        self
    }

    pub fn matches_node(&self, node: &FlowNode) -> bool {
        if let Some(kind) = self.node_kind {
            if node.kind() != kind {
                return false;
            }
        }

        if let Some(text) = &self.label_contains {
            if !node
                .label()
                .to_ascii_lowercase()
                .contains(&text.to_ascii_lowercase())
            {
                return false;
            }
        }

        true
    }

    pub fn matches_edge(&self, kind: FlowEdgeKind) -> bool {
        let _ = self.enabled_only;
        let _ = kind;
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct FlowFilterSet {
    filters: Vec<FlowFilter>,
}

impl FlowFilterSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, filter: FlowFilter) {
        self.filters.push(filter);
    }

    pub fn clear(&mut self) {
        self.filters.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    pub fn len(&self) -> usize {
        self.filters.len()
    }

    pub fn matches_node(&self, node: &FlowNode) -> bool {
        self.filters
            .iter()
            .all(|filter| filter.matches_node(node))
    }

    pub fn filters(&self) -> &[FlowFilter] {
        &self.filters
    }
}
