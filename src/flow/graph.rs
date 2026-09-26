//! Directed graph containing Conduit's information-flow model.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    edge::{EdgeId, FlowEdge, FlowEdgeKind},
    node::{FlowNode, FlowNodeId, FlowNodeKind},
};

#[derive(Debug)]
pub enum FlowGraphError {
    NodeNotFound(FlowNodeId),
    EdgeNotFound(EdgeId),
    DuplicateNode(FlowNodeId),
    DuplicateEdge(EdgeId),
    SelfConnection(FlowNodeId),
}

pub struct FlowGraph {
    nodes: BTreeMap<FlowNodeId, FlowNode>,
    edges: BTreeMap<EdgeId, FlowEdge>,
    outgoing: BTreeMap<FlowNodeId, BTreeSet<EdgeId>>,
    incoming: BTreeMap<FlowNodeId, BTreeSet<EdgeId>>,
}

impl Default for FlowGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl FlowGraph {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
            outgoing: BTreeMap::new(),
            incoming: BTreeMap::new(),
        }
    }

    pub fn add_node(
        &mut self,
        node: FlowNode,
    ) -> Result<(), FlowGraphError> {
        let id = node.id();

        if self.nodes.contains_key(&id) {
            return Err(FlowGraphError::DuplicateNode(id));
        }

        self.nodes.insert(id, node);
        self.outgoing.entry(id).or_default();
        self.incoming.entry(id).or_default();

        Ok(())
    }

    pub fn remove_node(
        &mut self,
        id: FlowNodeId,
    ) -> Result<FlowNode, FlowGraphError> {
        if !self.nodes.contains_key(&id) {
            return Err(FlowGraphError::NodeNotFound(id));
        }

        let edges = self
            .outgoing
            .get(&id)
            .into_iter()
            .flat_map(|edges| edges.iter().copied())
            .chain(
                self.incoming
                    .get(&id)
                    .into_iter()
                    .flat_map(|edges| edges.iter().copied()),
            )
            .collect::<Vec<_>>();

        for edge_id in edges {
            let _ = self.remove_edge(edge_id);
        }

        self.outgoing.remove(&id);
        self.incoming.remove(&id);

        self.nodes
            .remove(&id)
            .ok_or(FlowGraphError::NodeNotFound(id))
    }

    pub fn add_edge(
        &mut self,
        edge: FlowEdge,
    ) -> Result<(), FlowGraphError> {
        let id = edge.id();

        if self.edges.contains_key(&id) {
            return Err(FlowGraphError::DuplicateEdge(id));
        }

        if !self.nodes.contains_key(&edge.source()) {
            return Err(FlowGraphError::NodeNotFound(edge.source()));
        }

        if !self.nodes.contains_key(&edge.target()) {
            return Err(FlowGraphError::NodeNotFound(edge.target()));
        }

        if edge.source() == edge.target() {
            return Err(FlowGraphError::SelfConnection(edge.source()));
        }

        self.outgoing
            .entry(edge.source())
            .or_default()
            .insert(id);

        self.incoming
            .entry(edge.target())
            .or_default()
            .insert(id);

        self.edges.insert(id, edge);

        Ok(())
    }

    pub fn remove_edge(
        &mut self,
        id: EdgeId,
    ) -> Result<FlowEdge, FlowGraphError> {
        let edge = self
            .edges
            .remove(&id)
            .ok_or(FlowGraphError::EdgeNotFound(id))?;

        if let Some(edges) = self.outgoing.get_mut(&edge.source()) {
            edges.remove(&id);
        }

        if let Some(edges) = self.incoming.get_mut(&edge.target()) {
            edges.remove(&id);
        }

        Ok(edge)
    }

    pub fn node(&self, id: FlowNodeId) -> Option<&FlowNode> {
        self.nodes.get(&id)
    }

    pub fn node_mut(&mut self, id: FlowNodeId) -> Option<&mut FlowNode> {
        self.nodes.get_mut(&id)
    }

    pub fn edge(&self, id: EdgeId) -> Option<&FlowEdge> {
        self.edges.get(&id)
    }

    pub fn edge_mut(&mut self, id: EdgeId) -> Option<&mut FlowEdge> {
        self.edges.get_mut(&id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &FlowNode> {
        self.nodes.values()
    }

    pub fn edges(&self) -> impl Iterator<Item = &FlowEdge> {
        self.edges.values()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn outgoing_edges(
        &self,
        node: FlowNodeId,
    ) -> impl Iterator<Item = &FlowEdge> {
        self.outgoing
            .get(&node)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|id| self.edges.get(id))
    }

    pub fn incoming_edges(
        &self,
        node: FlowNodeId,
    ) -> impl Iterator<Item = &FlowEdge> {
        self.incoming
            .get(&node)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|id| self.edges.get(id))
    }

    pub fn nodes_of_kind(
        &self,
        kind: FlowNodeKind,
    ) -> impl Iterator<Item = &FlowNode> {
        self.nodes.values().filter(move |node| node.kind() == kind)
    }

    pub fn connected_to(
        &self,
        node: FlowNodeId,
    ) -> impl Iterator<Item = FlowNodeId> + '_ {
        self.outgoing_edges(node)
            .map(|edge| edge.target())
            .chain(self.incoming_edges(node).map(|edge| edge.source()))
    }

    pub fn has_connection(
        &self,
        source: FlowNodeId,
        target: FlowNodeId,
    ) -> bool {
        self.outgoing_edges(source)
            .any(|edge| edge.target() == target)
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.outgoing.clear();
        self.incoming.clear();
    }
}
