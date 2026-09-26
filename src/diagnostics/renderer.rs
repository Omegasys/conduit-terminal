#[derive(Debug, Clone, Default)]
pub struct RendererDiagnostics {
    backend: Option<String>,
    initialized: bool,
    hardware_accelerated: bool,
    frame_count: u64,
    draw_calls: u64,
    texture_count: u64,
    surface_width: u32,
    surface_height: u32,
    last_error: Option<String>,
}

impl RendererDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_backend(&mut self, backend: impl Into<String>) {
        self.backend = Some(backend.into());
    }

    pub fn set_initialized(&mut self, initialized: bool) {
        self.initialized = initialized;
    }

    pub fn set_hardware_accelerated(&mut self, enabled: bool) {
        self.hardware_accelerated = enabled;
    }

    pub fn record_frame(&mut self) {
        self.frame_count = self.frame_count.saturating_add(1);
    }

    pub fn record_draw_call(&mut self) {
        self.draw_calls = self.draw_calls.saturating_add(1);
    }

    pub fn set_texture_count(&mut self, count: u64) {
        self.texture_count = count;
    }

    pub fn set_surface_size(&mut self, width: u32, height: u32) {
        self.surface_width = width;
        self.surface_height = height;
    }

    pub fn set_error(&mut self, error: impl Into<String>) {
        self.last_error = Some(error.into());
    }

    pub fn clear_error(&mut self) {
        self.last_error = None;
    }

    pub fn backend(&self) -> Option<&str> {
        self.backend.as_deref()
    }

    pub fn initialized(&self) -> bool {
        self.initialized
    }

    pub fn hardware_accelerated(&self) -> bool {
        self.hardware_accelerated
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn draw_calls(&self) -> u64 {
        self.draw_calls
    }

    pub fn texture_count(&self) -> u64 {
        self.texture_count
    }

    pub fn surface_size(&self) -> (u32, u32) {
        (self.surface_width, self.surface_height)
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }
}
