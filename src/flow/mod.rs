//! Conduit information-flow graph.
//!
//! The flow subsystem models relationships between processes, PTYs,
//! terminals, panes, and runtime events. It is intentionally separate
//! from the actual process and PTY implementations.
//!
//! A flow graph answers questions such as:
//!
//! - Which process is connected to this terminal?
//! - Which PTY feeds this pane?
//! - Where did an event originate?
//! - Which nodes are connected to this process?
//! - What information is flowing through the current workspace?
//!
//! The flow subsystem does not own processes, PTYs, or panes.

pub mod edge;
pub mod engine;
pub mod event;
pub mod filters;
pub mod graph;
pub mod node;
pub mod pane;
pub mod process;
pub mod pty;
pub mod terminal;

pub use edge::{EdgeId, FlowEdge, FlowEdgeKind};
pub use engine::{FlowEngine, FlowEngineError};
pub use event::{FlowEvent, FlowEventKind};
pub use filters::{FlowFilter, FlowFilterSet};
pub use graph::{FlowGraph, FlowGraphError};
pub use node::{FlowNode, FlowNodeId, FlowNodeKind};
pub use pane::FlowPane;
pub use process::FlowProcess;
pub use pty::FlowPty;
pub use terminal::FlowTerminal;
