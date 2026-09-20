use crate::gui::flow_view::{
    FlowDirection,
    FlowNodeId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiFlowMode {
    Overview,
    Processes,
    Events,
    Dependencies,
    Plugins,
    Network,
    Terminal,
}

impl TuiFlowMode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Processes => "Processes",
            Self::Events => "Events",
            Self::Dependencies => "Dependencies",
            Self::Plugins => "Plugins",
            Self::Network => "Network",
            Self::Terminal => "Terminal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowDisplayStyle {
    Compact,
    Detailed,
    Tree,
    Graph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlowViewport {
    pub width: u16,
    pub height: u16,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl FlowViewport {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn pan(&mut self, x: i32, y: i32) {
        self.offset_x = self.offset_x.saturating_add(x);
        self.offset_y = self.offset_y.saturating_add(y);
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
}

pub struct TuiFlowView {
    visible: bool,
    mode: TuiFlowMode,
    style: FlowDisplayStyle,
    direction: FlowDirection,

    viewport: FlowViewport,
    zoom: u16,

    selected_node: Option<FlowNodeId>,

    show_processes: bool,
    show_events: bool,
    show_plugins: bool,
    show_network: bool,
    show_terminal: bool,

    auto_layout: bool,
    follow_selection: bool,
}

impl Default for TuiFlowView {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiFlowView {
    pub fn new() -> Self {
        Self {
            visible: false,
            mode: TuiFlowMode::Overview,
            style: FlowDisplayStyle::Compact,
            direction: FlowDirection::LeftToRight,

            viewport: FlowViewport::new(80, 24),
            zoom: 100,

            selected_node: None,

            show_processes: true,
            show_events: true,
            show_plugins: true,
            show_network: false,
            show_terminal: true,

            auto_layout: true,
            follow_selection: true,
        }
    }

    pub fn open(&mut self) {
        self.visible = true;
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn mode(&self) -> TuiFlowMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: TuiFlowMode) {
        self.mode = mode;
    }

    pub fn style(&self) -> FlowDisplayStyle {
        self.style
    }

    pub fn set_style(&mut self, style: FlowDisplayStyle) {
        self.style = style;
    }

    pub fn direction(&self) -> FlowDirection {
        self.direction
    }

    pub fn set_direction(&mut self, direction: FlowDirection) {
        self.direction = direction;
    }

    pub fn viewport(&self) -> FlowViewport {
        self.viewport
    }

    pub fn set_viewport(&mut self, viewport: FlowViewport) {
        self.viewport = viewport;
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.viewport.resize(width, height);
    }

    pub fn pan(&mut self, x: i32, y: i32) {
        self.viewport.pan(x, y);
    }

    pub fn reset_pan(&mut self) {
        self.viewport.offset_x = 0;
        self.viewport.offset_y = 0;
    }

    pub fn zoom(&self) -> u16 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: u16) {
        self.zoom = zoom.clamp(25, 400);
    }

    pub fn zoom_in(&mut self) {
        self.set_zoom(self.zoom.saturating_add(10));
    }

    pub fn zoom_out(&mut self) {
        self.set_zoom(self.zoom.saturating_sub(10));
    }

    pub fn reset_zoom(&mut self) {
        self.zoom = 100;
    }

    pub fn selected_node(&self) -> Option<FlowNodeId> {
        self.selected_node
    }

    pub fn select_node(&mut self, node: FlowNodeId) {
        self.selected_node = Some(node);
    }

    pub fn clear_selection(&mut self) {
        self.selected_node = None;
    }

    pub fn show_processes(&self) -> bool {
        self.show_processes
    }

    pub fn set_show_processes(&mut self, show: bool) {
        self.show_processes = show;
    }

    pub fn show_events(&self) -> bool {
        self.show_events
    }

    pub fn set_show_events(&mut self, show: bool) {
        self.show_events = show;
    }

    pub fn show_plugins(&self) -> bool {
        self.show_plugins
    }

    pub fn set_show_plugins(&mut self, show: bool) {
        self.show_plugins = show;
    }

    pub fn show_network(&self) -> bool {
        self.show_network
    }

    pub fn set_show_network(&mut self, show: bool) {
        self.show_network = show;
    }

    pub fn show_terminal(&self) -> bool {
        self.show_terminal
    }

    pub fn set_show_terminal(&mut self, show: bool) {
        self.show_terminal = show;
    }

    pub fn auto_layout(&self) -> bool {
        self.auto_layout
    }

    pub fn set_auto_layout(&mut self, enabled: bool) {
        self.auto_layout = enabled;
    }

    pub fn follow_selection(&self) -> bool {
        self.follow_selection
    }

    pub fn set_follow_selection(&mut self, enabled: bool) {
        self.follow_selection = enabled;
    }

    pub fn reset_view(&mut self) {
        self.zoom = 100;
        self.viewport.offset_x = 0;
        self.viewport.offset_y = 0;
        self.selected_node = None;
    }

    pub fn clear(&mut self) {
        self.selected_node = None;
        self.reset_view();
    }
}
