//! Pane nodes in the flow graph.

use super::node::FlowNodeId;

#[derive(Debug, Clone)]
pub struct FlowPane {
    id: FlowNodeId,
    pane_id: String,
    label: String,
    workspace_id: Option<String>,
    tab_id: Option<String>,
    focused: bool,
}

impl FlowPane {
    pub fn new(
        id: FlowNodeId,
        pane_id: impl Into<String>,
    ) -> Self {
        let pane_id = pane_id.into();

        Self {
            id,
            label: format!("Pane {pane_id}"),
            pane_id,
            workspace_id: None,
            tab_id: None,
            focused: false,
        }
    }

    pub fn id(&self) -> FlowNodeId {
        self.id
    }

    pub fn pane_id(&self) -> &str {
        &self.pane_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    pub fn workspace_id(&self) -> Option<&str> {
        self.workspace_id.as_deref()
    }

    pub fn set_workspace_id(&mut self, id: impl Into<String>) {
        self.workspace_id = Some(id.into());
    }

    pub fn tab_id(&self) -> Option<&str> {
        self.tab_id.as_deref()
    }

    pub fn set_tab_id(&mut self, id: impl Into<String>) {
        self.tab_id = Some(id.into());
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
}
