#[derive(Debug, Clone, Default)]
pub struct PluginDiagnostics {
    discovered: u64,
    loaded: u64,
    active: u64,
    disabled: u64,
    failed: u64,
    sandboxed: u64,
}

impl PluginDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_discovered(&mut self, count: u64) {
        self.discovered = count;
    }

    pub fn set_loaded(&mut self, count: u64) {
        self.loaded = count;
    }

    pub fn set_active(&mut self, count: u64) {
        self.active = count;
    }

    pub fn set_disabled(&mut self, count: u64) {
        self.disabled = count;
    }

    pub fn set_failed(&mut self, count: u64) {
        self.failed = count;
    }

    pub fn set_sandboxed(&mut self, count: u64) {
        self.sandboxed = count;
    }

    pub fn discovered(&self) -> u64 {
        self.discovered
    }

    pub fn loaded(&self) -> u64 {
        self.loaded
    }

    pub fn active(&self) -> u64 {
        self.active
    }

    pub fn disabled(&self) -> u64 {
        self.disabled
    }

    pub fn failed(&self) -> u64 {
        self.failed
    }

    pub fn sandboxed(&self) -> u64 {
        self.sandboxed
    }
}
