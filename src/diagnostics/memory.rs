#[derive(Debug, Clone, Default)]
pub struct MemoryDiagnostics {
    process_bytes: Option<u64>,
    resident_bytes: Option<u64>,
    allocated_bytes: Option<u64>,
    terminal_buffer_bytes: Option<u64>,
    scrollback_bytes: Option<u64>,
    recording_bytes: Option<u64>,
    texture_bytes: Option<u64>,
}

impl MemoryDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_process_bytes(&mut self, bytes: u64) {
        self.process_bytes = Some(bytes);
    }

    pub fn set_resident_bytes(&mut self, bytes: u64) {
        self.resident_bytes = Some(bytes);
    }

    pub fn set_allocated_bytes(&mut self, bytes: u64) {
        self.allocated_bytes = Some(bytes);
    }

    pub fn set_terminal_buffer_bytes(&mut self, bytes: u64) {
        self.terminal_buffer_bytes = Some(bytes);
    }

    pub fn set_scrollback_bytes(&mut self, bytes: u64) {
        self.scrollback_bytes = Some(bytes);
    }

    pub fn set_recording_bytes(&mut self, bytes: u64) {
        self.recording_bytes = Some(bytes);
    }

    pub fn set_texture_bytes(&mut self, bytes: u64) {
        self.texture_bytes = Some(bytes);
    }

    pub fn process_bytes(&self) -> Option<u64> {
        self.process_bytes
    }

    pub fn resident_bytes(&self) -> Option<u64> {
        self.resident_bytes
    }

    pub fn allocated_bytes(&self) -> Option<u64> {
        self.allocated_bytes
    }

    pub fn terminal_buffer_bytes(&self) -> Option<u64> {
        self.terminal_buffer_bytes
    }

    pub fn scrollback_bytes(&self) -> Option<u64> {
        self.scrollback_bytes
    }

    pub fn recording_bytes(&self) -> Option<u64> {
        self.recording_bytes
    }

    pub fn texture_bytes(&self) -> Option<u64> {
        self.texture_bytes
    }
}
