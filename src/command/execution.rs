use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandExecutionState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct CommandExecution {
    id: u64,
    command: String,
    state: CommandExecutionState,
    started_at: Option<Instant>,
    completed_at: Option<Instant>,
    exit_code: Option<i32>,
    working_directory: Option<String>,
}

impl CommandExecution {
    pub fn new(id: u64, command: impl Into<String>) -> Self {
        Self {
            id,
            command: command.into(),
            state: CommandExecutionState::Pending,
            started_at: None,
            completed_at: None,
            exit_code: None,
            working_directory: None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn state(&self) -> CommandExecutionState {
        self.state
    }

    pub fn started_at(&self) -> Option<Instant> {
        self.started_at
    }

    pub fn completed_at(&self) -> Option<Instant> {
        self.completed_at
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn working_directory(&self) -> Option<&str> {
        self.working_directory.as_deref()
    }

    pub fn set_working_directory(&mut self, directory: impl Into<String>) {
        self.working_directory = Some(directory.into());
    }

    pub fn start(&mut self) {
        self.state = CommandExecutionState::Running;
        self.started_at = Some(Instant::now());
    }

    pub fn complete(&mut self, exit_code: i32) {
        self.state = if exit_code == 0 {
            CommandExecutionState::Completed
        } else {
            CommandExecutionState::Failed
        };

        self.exit_code = Some(exit_code);
        self.completed_at = Some(Instant::now());
    }

    pub fn cancel(&mut self) {
        self.state = CommandExecutionState::Cancelled;
        self.completed_at = Some(Instant::now());
    }

    pub fn duration(&self) -> Option<Duration> {
        let start = self.started_at?;

        match self.completed_at {
            Some(end) => Some(end.duration_since(start)),
            None => Some(start.elapsed()),
        }
    }

    pub fn is_finished(&self) -> bool {
        matches!(
            self.state,
            CommandExecutionState::Completed
                | CommandExecutionState::Failed
                | CommandExecutionState::Cancelled
        )
    }
}

#[derive(Debug, Default)]
pub struct CommandExecutionManager {
    executions: HashMap<u64, CommandExecution>,
    next_id: u64,
}

impl CommandExecutionManager {
    pub fn new() -> Self {
        Self {
            executions: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create(&mut self, command: impl Into<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.executions
            .insert(id, CommandExecution::new(id, command));

        id
    }

    pub fn start(&mut self, id: u64) -> bool {
        let Some(execution) = self.executions.get_mut(&id) else {
            return false;
        };

        execution.start();
        true
    }

    pub fn complete(&mut self, id: u64, exit_code: i32) -> bool {
        let Some(execution) = self.executions.get_mut(&id) else {
            return false;
        };

        execution.complete(exit_code);
        true
    }

    pub fn cancel(&mut self, id: u64) -> bool {
        let Some(execution) = self.executions.get_mut(&id) else {
            return false;
        };

        execution.cancel();
        true
    }

    pub fn get(&self, id: u64) -> Option<&CommandExecution> {
        self.executions.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut CommandExecution> {
        self.executions.get_mut(&id)
    }

    pub fn remove(&mut self, id: u64) -> Option<CommandExecution> {
        self.executions.remove(&id)
    }

    pub fn running(&self) -> Vec<&CommandExecution> {
        self.executions
            .values()
            .filter(|execution| {
                execution.state() == CommandExecutionState::Running
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.executions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.executions.is_empty()
    }

    pub fn clear_finished(&mut self) {
        self.executions.retain(|_, execution| !execution.is_finished());
    }
}
