//! Process nodes in the flow graph.

use super::node::FlowNodeId;

#[derive(Debug, Clone)]
pub struct FlowProcess {
    id: FlowNodeId,
    process_id: u32,
    executable: String,
    command: Option<String>,
    label: String,
    running: bool,
    exit_status: Option<i32>,
}

impl FlowProcess {
    pub fn new(
        id: FlowNodeId,
        process_id: u32,
        executable: impl Into<String>,
    ) -> Self {
        let executable = executable.into();

        Self {
            id,
            process_id,
            label: executable.clone(),
            executable,
            command: None,
            running: true,
            exit_status: None,
        }
    }

    pub fn id(&self) -> FlowNodeId {
        self.id
    }

    pub fn process_id(&self) -> u32 {
        self.process_id
    }

    pub fn executable(&self) -> &str {
        &self.executable
    }

    pub fn command(&self) -> Option<&str> {
        self.command.as_deref()
    }

    pub fn set_command(&mut self, command: impl Into<String>) {
        self.command = Some(command.into());
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn exit_status(&self) -> Option<i32> {
        self.exit_status
    }

    pub fn mark_exited(&mut self, status: Option<i32>) {
        self.running = false;
        self.exit_status = status;
    }
}
