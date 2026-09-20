use std::collections::BTreeMap;

use super::edges::{EdgeId, FlowEdge};
use super::nodes::{FlowNode, FlowNodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl Default for FlowDirection {
    fn default() -> Self {
        Self::LeftToRight
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlowPosition {
    pub x: f32,
    pub y: f32,
}

impl FlowPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn offset(self, x: f32, y: f32) -> Self {
        Self::new(self.x + x, self.y + y)
    }
}

impl Default for FlowPosition {
    fn default() -> Self {
        Self::origin()
    }
}

#[derive(Debug, Clone)]
pub struct FlowLayout {
    direction: FlowDirection,
    horizontal_spacing: f32,
    vertical_spacing: f32,
    node_width: f32,
    node_height: f32,
}

impl Default for FlowLayout {
    fn default() -> Self {
        Self {
            direction: FlowDirection::LeftToRight,
            horizontal_spacing: 80.0,
            vertical_spacing: 40.0,
            node_width: 180.0,
            node_height: 80.0,
        }
    }
}

impl FlowLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn direction(&self) -> FlowDirection {
        self.direction
    }

    pub fn set_direction(&mut self, direction: FlowDirection) {
        self.direction = direction;
    }

    pub fn horizontal_spacing(&self) -> f32 {
        self.horizontal_spacing
    }

    pub fn set_horizontal_spacing(&mut self, spacing: f32) {
        self.horizontal_spacing = spacing.max(0.0);
    }

    pub fn vertical_spacing(&self) -> f32 {
        self.vertical_spacing
    }

    pub fn set_vertical_spacing(&mut self, spacing: f32) {
        self.vertical_spacing = spacing.max(0.0);
    }

    pub fn node_width(&self) -> f32 {
        self.node_width
    }

    pub fn node_height(&self) -> f32 {
        self.node_height
    }

    pub fn set_node_size(&mut self, width: f32, height: f32) {
        self.node_width = width.max(1.0);
        self.node_height = height.max(1.0);
    }

    pub fn calculate_positions(
        &self,
        nodes: &mut BTreeMap<FlowNodeId, FlowNode>,
    ) {
        let mut x = 0.0;
        let mut y = 0.0;

        for node in nodes.values_mut() {
            node.set_position(FlowPosition::new(x, y));

            match self.direction {
                FlowDirection::LeftToRight
                | FlowDirection::RightToLeft => {
                    x += self.node_width + self.horizontal_spacing;
                }
                FlowDirection::TopToBottom
                | FlowDirection::BottomToTop => {
                    y += self.node_height + self.vertical_spacing;
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Flow {
    nodes: BTreeMap<FlowNodeId, FlowNode>,
    edges: BTreeMap<EdgeId, FlowEdge>,
    selected_node: Option<FlowNodeId>,
    selected_edge: Option<EdgeId>,
    next_node_id: u64,
    next_edge_id: u64,
    layout: FlowLayout,
}

impl Flow {
    pub fn new() -> Self {
        Self {
            next_node_id: 1,
            next_edge_id: 1,
            ..Default::default()
        }
    }

    pub fn add_node(&mut self, mut node: FlowNode) -> FlowNodeId {
        let id = FlowNodeId::new(self.next_node_id);
        self.next_node_id += 1;

        node.set_id(id);
        self.nodes.insert(id, node);

        id
    }

    pub fn add_edge(
        &mut self,
        mut edge: FlowEdge,
    ) -> Option<EdgeId> {
        if !self.nodes.contains_key(&edge.source())
            || !self.nodes.contains_key(&edge.target())
        {
            return None;
        }

        let id = EdgeId::new(self.next_edge_id);
        self.next_edge_id += 1;

        edge.set_id(id);
        self.edges.insert(id, edge);

        Some(id)
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

    pub fn remove_node(&mut self, id: FlowNodeId) -> Option<FlowNode> {
        let removed = self.nodes.remove(&id);

        if removed.is_some() {
            self.edges.retain(|_, edge| {
                edge.source() != id && edge.target() != id
            });

            if self.selected_node == Some(id) {
                self.selected_node = None;
            }
        }

        removed
    }

    pub fn remove_edge(&mut self, id: EdgeId) -> Option<FlowEdge> {
        let removed = self.edges.remove(&id);

        if removed.is_some() && self.selected_edge == Some(id) {
            self.selected_edge = None;
        }

        removed
    }

    pub fn select_node(&mut self, id: Option<FlowNodeId>) {
        self.selected_node = id;

        if id.is_some() {
            self.selected_edge = None;
        }
    }

    pub fn select_edge(&mut self, id: Option<EdgeId>) {
        self.selected_edge = id;

        if id.is_some() {
            self.selected_node = None;
        }
    }

    pub fn selected_node(&self) -> Option<FlowNodeId> {
        self.selected_node
    }

    pub fn selected_edge(&self) -> Option<EdgeId> {
        self.selected_edge
    }

    pub fn layout(&self) -> &FlowLayout {
        &self.layout
    }

    pub fn layout_mut(&mut self) -> &mut FlowLayout {
        &mut self.layout
    }

    pub fn relayout(&mut self) {
        self.layout.calculate_positions(&mut self.nodes);
    }

    pub fn clear_selection(&mut self) {
        self.selected_node = None;
        self.selected_edge = None;
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.clear_selection();
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

#[derive(Debug, Default)]
pub struct FlowViewState {
    flow: Flow,
    visible: bool,
    zoom: f32,
    pan: FlowPosition,
    auto_layout: bool,
    show_processes: bool,
    show_events: bool,
    show_plugins: bool,
}

impl FlowViewState {
    pub fn new() -> Self {
        Self {
            flow: Flow::new(),
            visible: true,
            zoom: 1.0,
            pan: FlowPosition::origin(),
            auto_layout: true,
            show_processes: true,
            show_events: true,
            show_plugins: true,
        }
    }

    pub fn flow(&self) -> &Flow {
        &self.flow
    }

    pub fn flow_mut(&mut self) -> &mut Flow {
        &mut self.flow
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.1, 8.0);
    }

    pub fn zoom_in(&mut self) {
        self.set_zoom(self.zoom * 1.1);
    }

    pub fn zoom_out(&mut self) {
        self.set_zoom(self.zoom / 1.1);
    }

    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
    }

    pub fn pan(&self) -> FlowPosition {
        self.pan
    }

    pub fn set_pan(&mut self, pan: FlowPosition) {
        self.pan = pan;
    }

    pub fn auto_layout(&self) -> bool {
        self.auto_layout
    }

    pub fn set_auto_layout(&mut self, enabled: bool) {
        self.auto_layout = enabled;
    }

    pub fn show_processes(&self) -> bool {
        self.show_processes
    }

    pub fn set_show_processes(&mut self, enabled: bool) {
        self.show_processes = enabled;
    }

    pub fn show_events(&self) -> bool {
        self.show_events
    }

    pub fn set_show_events(&mut self, enabled: bool) {
        self.show_events = enabled;
    }

    pub fn show_plugins(&self) -> bool {
        self.show_plugins
    }

    pub fn set_show_plugins(&mut self, enabled: bool) {
        self.show_plugins = enabled;
    }
}
