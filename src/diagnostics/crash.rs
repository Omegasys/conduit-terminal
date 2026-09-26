use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct CrashRecord {
    timestamp: SystemTime,
    message: String,
    component: Option<String>,
    thread: Option<String>,
}

impl CrashRecord {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            timestamp: SystemTime::now(),
            message: message.into(),
            component: None,
            thread: None,
        }
    }

    pub fn set_component(&mut self, component: impl Into<String>) {
        self.component = Some(component.into());
    }

    pub fn set_thread(&mut self, thread: impl Into<String>) {
        self.thread = Some(thread.into());
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn component(&self) -> Option<&str> {
        self.component.as_deref()
    }

    pub fn thread(&self) -> Option<&str> {
        self.thread.as_deref()
    }
}

#[derive(Debug, Default)]
pub struct CrashDiagnostics {
    last_crash: Option<CrashRecord>,
    crash_count: u64,
    recovery_count: u64,
}

impl CrashDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, crash: CrashRecord) {
        self.crash_count = self.crash_count.saturating_add(1);
        self.last_crash = Some(crash);
    }

    pub fn record_message(
        &mut self,
        message: impl Into<String>,
    ) {
        self.record(CrashRecord::new(message));
    }

    pub fn record_recovery(&mut self) {
        self.recovery_count =
            self.recovery_count.saturating_add(1);
    }

    pub fn last_crash(&self) -> Option<&CrashRecord> {
        self.last_crash.as_ref()
    }

    pub fn crash_count(&self) -> u64 {
        self.crash_count
    }

    pub fn recovery_count(&self) -> u64 {
        self.recovery_count
    }

    pub fn has_crashed(&self) -> bool {
        self.last_crash.is_some()
    }

    pub fn clear(&mut self) {
        self.last_crash = None;
    }
}
