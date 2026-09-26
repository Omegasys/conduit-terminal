#[derive(Debug, Clone, Default)]
pub struct PtyDiagnostics {
    active: u64,
    created: u64,
    closed: u64,
    failed: u64,
    bytes_read: u64,
    bytes_written: u64,
    child_processes: u64,
}

impl PtyDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_created(&mut self) {
        self.created = self.created.saturating_add(1);
        self.active = self.active.saturating_add(1);
    }

    pub fn record_closed(&mut self) {
        self.closed = self.closed.saturating_add(1);
        self.active = self.active.saturating_sub(1);
    }

    pub fn record_failure(&mut self) {
        self.failed = self.failed.saturating_add(1);
    }

    pub fn record_read(&mut self, bytes: u64) {
        self.bytes_read = self.bytes_read.saturating_add(bytes);
    }

    pub fn record_written(&mut self, bytes: u64) {
        self.bytes_written =
            self.bytes_written.saturating_add(bytes);
    }

    pub fn set_child_processes(&mut self, count: u64) {
        self.child_processes = count;
    }

    pub fn active(&self) -> u64 {
        self.active
    }

    pub fn created(&self) -> u64 {
        self.created
    }

    pub fn closed(&self) -> u64 {
        self.closed
    }

    pub fn failed(&self) -> u64 {
        self.failed
    }

    pub fn bytes_read(&self) -> u64 {
        self.bytes_read
    }

    pub fn bytes_written(&self) -> u64 {
        self.bytes_written
    }

    pub fn child_processes(&self) -> u64 {
        self.child_processes
    }
}
