//! PTY nodes in the flow graph.

use super::node::FlowNodeId;

#[derive(Debug, Clone)]
pub struct FlowPty {
    id: FlowNodeId,
    pty_id: String,
    label: String,
    rows: u16,
    columns: u16,
    active: bool,
}

impl FlowPty {
    pub fn new(
        id: FlowNodeId,
        pty_id: impl Into<String>,
        rows: u16,
        columns: u16,
    ) -> Self {
        let pty_id = pty_id.into();

        Self {
            id,
            label: format!("PTY {pty_id}"),
            pty_id,
            rows,
            columns,
            active: true,
        }
    }

    pub fn id(&self) -> FlowNodeId {
        self.id
    }

    pub fn pty_id(&self) -> &str {
        &self.pty_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn columns(&self) -> u16 {
        self.columns
    }

    pub fn resize(&mut self, rows: u16, columns: u16) {
        self.rows = rows;
        self.columns = columns;
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}
