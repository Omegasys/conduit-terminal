#[derive(Debug, Clone, Default)]
pub struct ResourceDiagnostics {
    resources_loaded: u64,
    resources_active: u64,
    resources_failed: u64,
    resources_missing: u64,
    hot_reload_enabled: bool,
}

impl ResourceDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_loaded(&mut self, count: u64) {
        self.resources_loaded = count;
    }

    pub fn set_active(&mut self, count: u64) {
        self.resources_active = count;
    }

    pub fn set_failed(&mut self, count: u64) {
        self.resources_failed = count;
    }

    pub fn set_missing(&mut self, count: u64) {
        self.resources_missing = count;
    }

    pub fn set_hot_reload(&mut self, enabled: bool) {
        self.hot_reload_enabled = enabled;
    }

    pub fn loaded(&self) -> u64 {
        self.resources_loaded
    }

    pub fn active(&self) -> u64 {
        self.resources_active
    }

    pub fn failed(&self) -> u64 {
        self.resources_failed
    }

    pub fn missing(&self) -> u64 {
        self.resources_missing
    }

    pub fn hot_reload_enabled(&self) -> bool {
        self.hot_reload_enabled
    }
}
