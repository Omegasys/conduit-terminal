use std::fmt;

use crate::gui::flow_view::{
    FlowDirection,
    FlowNodeId,
};

/// Actions supported by the `conduit flow` command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowCommandAction {
    Show,
    Hide,
    Toggle,
    Refresh,
    Layout,
    SelectNode,
    SelectEdge,
    FocusTerminal,
    FocusProcesses,
    FocusEvents,
    FocusDependencies,
    FocusPlugins,
    FocusNetwork,
    ResetView,
    Export,
}

impl FlowCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Show => "show",
            Self::Hide => "hide",
            Self::Toggle => "toggle",
            Self::Refresh => "refresh",
            Self::Layout => "layout",
            Self::SelectNode => "select-node",
            Self::SelectEdge => "select-edge",
            Self::FocusTerminal => "terminal",
            Self::FocusProcesses => "processes",
            Self::FocusEvents => "events",
            Self::FocusDependencies => "dependencies",
            Self::FocusPlugins => "plugins",
            Self::FocusNetwork => "network",
            Self::ResetView => "reset",
            Self::Export => "export",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "show" => Some(Self::Show),
            "hide" => Some(Self::Hide),
            "toggle" => Some(Self::Toggle),
            "refresh" => Some(Self::Refresh),
            "layout" => Some(Self::Layout),
            "select-node" => Some(Self::SelectNode),
            "select-edge" => Some(Self::SelectEdge),
            "terminal" => Some(Self::FocusTerminal),
            "processes" => Some(Self::FocusProcesses),
            "events" => Some(Self::FocusEvents),
            "dependencies" => Some(Self::FocusDependencies),
            "plugins" => Some(Self::FocusPlugins),
            "network" => Some(Self::FocusNetwork),
            "reset" => Some(Self::ResetView),
            "export" => Some(Self::Export),
            _ => None,
        }
    }
}

impl fmt::Display for FlowCommandAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// Flow layout modes available to CLI callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowLayoutMode {
    Automatic,
    Compact,
    Detailed,
    Tree,
    Graph,
}

impl FlowLayoutMode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Compact => "compact",
            Self::Detailed => "detailed",
            Self::Tree => "tree",
            Self::Graph => "graph",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "automatic" | "auto" => Some(Self::Automatic),
            "compact" => Some(Self::Compact),
            "detailed" => Some(Self::Detailed),
            "tree" => Some(Self::Tree),
            "graph" => Some(Self::Graph),
            _ => None,
        }
    }
}

impl fmt::Display for FlowLayoutMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A parsed flow-view command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowCommand {
    action: FlowCommandAction,
    node_id: Option<FlowNodeId>,
    edge_id: Option<u64>,
    direction: Option<FlowDirection>,
    layout: Option<FlowLayoutMode>,
    zoom: Option<u16>,
    output_path: Option<String>,
    include_processes: bool,
    include_events: bool,
    include_plugins: bool,
    include_network: bool,
}

impl FlowCommand {
    pub fn new(action: FlowCommandAction) -> Self {
        Self {
            action,
            node_id: None,
            edge_id: None,
            direction: None,
            layout: None,
            zoom: None,
            output_path: None,
            include_processes: true,
            include_events: true,
            include_plugins: true,
            include_network: true,
        }
    }

    pub fn action(&self) -> &FlowCommandAction {
        &self.action
    }

    pub fn node_id(&self) -> Option<FlowNodeId> {
        self.node_id
    }

    pub fn edge_id(&self) -> Option<u64> {
        self.edge_id
    }

    pub fn direction(&self) -> Option<FlowDirection> {
        self.direction
    }

    pub fn layout(&self) -> Option<FlowLayoutMode> {
        self.layout
    }

    pub fn zoom(&self) -> Option<u16> {
        self.zoom
    }

    pub fn output_path(&self) -> Option<&str> {
        self.output_path.as_deref()
    }

    pub fn include_processes(&self) -> bool {
        self.include_processes
    }

    pub fn include_events(&self) -> bool {
        self.include_events
    }

    pub fn include_plugins(&self) -> bool {
        self.include_plugins
    }

    pub fn include_network(&self) -> bool {
        self.include_network
    }

    pub fn set_node_id(&mut self, node_id: FlowNodeId) {
        self.node_id = Some(node_id);
    }

    pub fn set_edge_id(&mut self, edge_id: u64) {
        self.edge_id = Some(edge_id);
    }

    pub fn set_direction(&mut self, direction: FlowDirection) {
        self.direction = Some(direction);
    }

    pub fn set_layout(&mut self, layout: FlowLayoutMode) {
        self.layout = Some(layout);
    }

    pub fn set_zoom(&mut self, zoom: u16) {
        self.zoom = Some(zoom);
    }

    pub fn set_output_path<S>(&mut self, output_path: S)
    where
        S: Into<String>,
    {
        self.output_path = Some(output_path.into());
    }

    pub fn set_include_processes(&mut self, enabled: bool) {
        self.include_processes = enabled;
    }

    pub fn set_include_events(&mut self, enabled: bool) {
        self.include_events = enabled;
    }

    pub fn set_include_plugins(&mut self, enabled: bool) {
        self.include_plugins = enabled;
    }

    pub fn set_include_network(&mut self, enabled: bool) {
        self.include_network = enabled;
    }

    pub fn requires_node_id(&self) -> bool {
        matches!(self.action, FlowCommandAction::SelectNode)
    }

    pub fn requires_edge_id(&self) -> bool {
        matches!(self.action, FlowCommandAction::SelectEdge)
    }

    pub fn requires_output_path(&self) -> bool {
        matches!(self.action, FlowCommandAction::Export)
    }
}
