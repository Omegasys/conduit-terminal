//! Terminal nodes in the flow graph.

use super::node::FlowNodeId;

#[derive(Debug, Clone)]
pub struct FlowTerminal {
    id: FlowNodeId,
    terminal_id: String,
    label: String,
    rows: u16,
    columns: u16,
    alternate_screen: bool,
    focused: bool,
}

impl FlowTerminal {
    pub fn new(
        id: FlowNodeId,
        terminal_id: impl Into<String>,
        rows: u16,
        columns: u16,
    ) -> Self {
        let terminal_id = terminal_id.into();

        Self {
            id,
            label: format!("Terminal {terminal_id}"),
            terminal_id,
            rows,
            columns,
            alternate_screen: false,
            focused: false,
        }
    }

    pub fn id(&self) -> FlowNodeId {
        self.id
    }

    pub fn terminal_id(&self) -> &str {
        &self.terminal_id
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

    pub fn alternate_screen(&self) -> bool {
        self.alternate_screen
    }

    pub fn set_alternate_screen(&mut self, enabled: bool) {
        self.alternate_screen = enabled;
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
}
