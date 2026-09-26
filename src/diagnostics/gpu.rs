#[derive(Debug, Clone, Default)]
pub struct GpuDiagnostics {
    available: bool,
    backend: Option<String>,
    adapter: Option<String>,
    vendor: Option<String>,
    driver: Option<String>,
    dedicated_memory_bytes: Option<u64>,
    used_memory_bytes: Option<u64>,
    max_texture_size: Option<u32>,
    features: Vec<String>,
    last_error: Option<String>,
}

impl GpuDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_available(&mut self, available: bool) {
        self.available = available;
    }

    pub fn set_backend(&mut self, backend: impl Into<String>) {
        self.backend = Some(backend.into());
    }

    pub fn set_adapter(&mut self, adapter: impl Into<String>) {
        self.adapter = Some(adapter.into());
    }

    pub fn set_vendor(&mut self, vendor: impl Into<String>) {
        self.vendor = Some(vendor.into());
    }

    pub fn set_driver(&mut self, driver: impl Into<String>) {
        self.driver = Some(driver.into());
    }

    pub fn set_memory(
        &mut self,
        dedicated: u64,
        used: u64,
    ) {
        self.dedicated_memory_bytes = Some(dedicated);
        self.used_memory_bytes = Some(used);
    }

    pub fn set_max_texture_size(&mut self, size: u32) {
        self.max_texture_size = Some(size);
    }

    pub fn add_feature(&mut self, feature: impl Into<String>) {
        self.features.push(feature.into());
    }

    pub fn set_error(&mut self, error: impl Into<String>) {
        self.last_error = Some(error.into());
    }

    pub fn available(&self) -> bool {
        self.available
    }

    pub fn backend(&self) -> Option<&str> {
        self.backend.as_deref()
    }

    pub fn adapter(&self) -> Option<&str> {
        self.adapter.as_deref()
    }

    pub fn vendor(&self) -> Option<&str> {
        self.vendor.as_deref()
    }

    pub fn driver(&self) -> Option<&str> {
        self.driver.as_deref()
    }

    pub fn dedicated_memory_bytes(&self) -> Option<u64> {
        self.dedicated_memory_bytes
    }

    pub fn used_memory_bytes(&self) -> Option<u64> {
        self.used_memory_bytes
    }

    pub fn max_texture_size(&self) -> Option<u32> {
        self.max_texture_size
    }

    pub fn features(&self) -> &[String] {
        &self.features
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }
}
