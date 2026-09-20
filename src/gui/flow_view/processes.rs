use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessType {
    Shell,
    Command,
    Application,
    Plugin,
    Background,
    Service,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Starting,
    Running,
    Sleeping,
    Stopped,
    Exited,
    Failed,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pid: u32,
    parent_pid: Option<u32>,
    process_type: ProcessType,
    state: ProcessState,
    command: String,
    executable: Option<String>,
    working_directory: Option<String>,
    started_at: SystemTime,
    cpu_percent: f32,
    memory_bytes: u64,
}

impl ProcessInfo {
    pub fn new(
        pid: u32,
        process_type: ProcessType,
        command: impl Into<String>,
    ) -> Self {
        Self {
            pid,
            parent_pid: None,
            process_type,
            state: ProcessState::Starting,
            command: command.into(),
            executable: None,
            working_directory: None,
            started_at: SystemTime::now(),
            cpu_percent: 0.0,
            memory_bytes: 0,
        }
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn parent_pid(&self) -> Option<u32> {
        self.parent_pid
    }

    pub fn set_parent_pid(&mut self, pid: Option<u32>) {
        self.parent_pid = pid;
    }

    pub fn process_type(&self) -> ProcessType {
        self.process_type
    }

    pub fn state(&self) -> ProcessState {
        self.state
    }

    pub fn set_state(&mut self, state: ProcessState) {
        self.state = state;
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn set_command(&mut self, command: impl Into<String>) {
        self.command = command.into();
    }

    pub fn executable(&self) -> Option<&str> {
        self.executable.as_deref()
    }

    pub fn set_executable(&mut self, executable: Option<String>) {
        self.executable = executable;
    }

    pub fn working_directory(&self) -> Option<&str> {
        self.working_directory.as_deref()
    }

    pub fn set_working_directory(&mut self, directory: Option<String>) {
        self.working_directory = directory;
    }

    pub fn started_at(&self) -> SystemTime {
        self.started_at
    }

    pub fn uptime(&self) -> Option<Duration> {
        SystemTime::now()
            .duration_since(self.started_at)
            .ok()
    }

    pub fn cpu_percent(&self) -> f32 {
        self.cpu_percent
    }

    pub fn set_cpu_percent(&mut self, cpu_percent: f32) {
        self.cpu_percent = cpu_percent.max(0.0);
    }

    pub fn memory_bytes(&self) -> u64 {
        self.memory_bytes
    }

    pub fn set_memory_bytes(&mut self, memory_bytes: u64) {
        self.memory_bytes = memory_bytes;
    }

    pub fn is_running(&self) -> bool {
        matches!(
            self.state,
            ProcessState::Starting
                | ProcessState::Running
                | ProcessState::Sleeping
        )
    }
}

#[derive(Debug, Default)]
pub struct ProcessManager {
    processes: BTreeMap<u32, ProcessInfo>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, process: ProcessInfo) {
        self.processes.insert(process.pid(), process);
    }

    pub fn update(&mut self, process: ProcessInfo) {
        self.processes.insert(process.pid(), process);
    }

    pub fn get(&self, pid: u32) -> Option<&ProcessInfo> {
        self.processes.get(&pid)
    }

    pub fn get_mut(&mut self, pid: u32) -> Option<&mut ProcessInfo> {
        self.processes.get_mut(&pid)
    }

    pub fn remove(&mut self, pid: u32) -> Option<ProcessInfo> {
        self.processes.remove(&pid)
    }

    pub fn processes(&self) -> impl Iterator<Item = &ProcessInfo> {
        self.processes.values()
    }

    pub fn running(&self) -> impl Iterator<Item = &ProcessInfo> {
        self.processes
            .values()
            .filter(|process| process.is_running())
    }

    pub fn children_of(&self, parent_pid: u32) -> Vec<&ProcessInfo> {
        self.processes
            .values()
            .filter(|process| process.parent_pid() == Some(parent_pid))
            .collect()
    }

    pub fn running_count(&self) -> usize {
        self.running().count()
    }

    pub fn len(&self) -> usize {
        self.processes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.processes.is_empty()
    }

    pub fn clear(&mut self) {
        self.processes.clear();
    }

    pub fn prune_exited(&mut self) {
        self.processes.retain(|_, process| {
            !matches!(
                process.state(),
                ProcessState::Exited | ProcessState::Failed
            )
        });
    }
}
