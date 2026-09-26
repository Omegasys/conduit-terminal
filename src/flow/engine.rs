//! Runtime engine for maintaining the flow graph.

use super::{
    edge::{EdgeId, FlowEdge, FlowEdgeKind},
    event::{FlowEvent, FlowEventKind},
    graph::{FlowGraph, FlowGraphError},
    node::{FlowNode, FlowNodeId},
};

#[derive(Debug)]
pub enum FlowEngineError {
    Graph(FlowGraphError),
    InvalidIdentifier,
}

impl From<FlowGraphError> for FlowEngineError {
    fn from(error: FlowGraphError) -> Self {
        Self::Graph(error)
    }
}

pub struct FlowEngine {
    graph: FlowGraph,
    next_node_id: u64,
    next_edge_id: u64,
    next_event_sequence: u64,
    events: Vec<FlowEvent>,
    event_capacity: usize,
    enabled: bool,
}

impl Default for FlowEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl FlowEngine {
    pub fn new() -> Self {
        Self {
            graph: FlowGraph::new(),
            next_node_id: 1,
            next_edge_id: 1,
            next_event_sequence: 1,
            events: Vec::new(),
            event_capacity: 1024,
            enabled: true,
        }
    }

    pub fn graph(&self) -> &FlowGraph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut FlowGraph {
        &mut self.graph
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn allocate_node_id(&mut self) -> FlowNodeId {
        let id = FlowNodeId::new(self.next_node_id);
        self.next_node_id = self.next_node_id.wrapping_add(1);
        id
    }

    pub fn allocate_edge_id(&mut self) -> EdgeId {
        let id = EdgeId::new(self.next_edge_id);
        self.next_edge_id = self.next_edge_id.wrapping_add(1);
        id
    }

    pub fn add_node(
        &mut self,
        node: FlowNode,
    ) -> Result<(), FlowEngineError> {
        let id = node.id();

        self.graph.add_node(node)?;

        self.record(
            FlowEvent::new(
                self.next_sequence(),
                FlowEventKind::NodeCreated,
            )
            .with_source(id),
        );

        Ok(())
    }

    pub fn remove_node(
        &mut self,
        id: FlowNodeId,
    ) -> Result<FlowNode, FlowEngineError> {
        let node = self.graph.remove_node(id)?;

        self.record(
            FlowEvent::new(
                self.next_sequence(),
                FlowEventKind::NodeRemoved,
            )
            .with_source(id),
        );

        Ok(node)
    }

    pub fn connect(
        &mut self,
        source: FlowNodeId,
        target: FlowNodeId,
        kind: FlowEdgeKind,
    ) -> Result<EdgeId, FlowEngineError> {
        let id = self.allocate_edge_id();

        let edge = FlowEdge::new(id, source, target, kind);

        self.graph.add_edge(edge)?;

        self.record(
            FlowEvent::new(
                self.next_sequence(),
                FlowEventKind::EdgeCreated,
            )
            .with_source(source)
            .with_target(target)
            .with_edge(id),
        );

        Ok(id)
    }

    pub fn disconnect(
        &mut self,
        id: EdgeId,
    ) -> Result<FlowEdge, FlowEngineError> {
        let edge = self.graph.remove_edge(id)?;

        self.record(
            FlowEvent::new(
                self.next_sequence(),
                FlowEventKind::EdgeRemoved,
            )
            .with_source(edge.source())
            .with_target(edge.target())
            .with_edge(id),
        );

        Ok(edge)
    }

    pub fn record(&mut self, event: FlowEvent) {
        if !self.enabled {
            return;
        }

        self.events.push(event);

        if self.events.len() > self.event_capacity {
            let excess = self.events.len() - self.event_capacity;
            self.events.drain(0..excess);
        }
    }

    pub fn events(&self) -> &[FlowEvent] {
        &self.events
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    pub fn set_event_capacity(&mut self, capacity: usize) {
        self.event_capacity = capacity.max(1);

        if self.events.len() > self.event_capacity {
            let excess = self.events.len() - self.event_capacity;
            self.events.drain(0..excess);
        }
    }

    fn next_sequence(&mut self) -> u64 {
        let sequence = self.next_event_sequence;
        self.next_event_sequence =
            self.next_event_sequence.wrapping_add(1);

        sequence
    }

    pub fn reset(&mut self) {
        self.graph.clear();
        self.events.clear();
        self.next_node_id = 1;
        self.next_edge_id = 1;
        self.next_event_sequence = 1;
    }
}
