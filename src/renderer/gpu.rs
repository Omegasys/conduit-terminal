use super::renderer::RenderBackend;

/// GPU feature supported by a physical device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuFeature {
    HardwareAcceleration,
    TextureCompression,
    Multisampling,
    AnisotropicFiltering,
    ComputeShaders,
    GeometryShaders,
    Tessellation,
    WideGamut,
    Hdr,
    Presentation,
    Synchronization,
    TimelineSemaphores,
}

/// GPU backend family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuBackend {
    Unknown,
    OpenGL,
    Vulkan,
    Software,
}

impl GpuBackend {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::OpenGL => "opengl",
            Self::Vulkan => "vulkan",
            Self::Software => "software",
        }
    }
}

/// Hardware limits exposed by the GPU.
#[derive(Debug, Clone, Copy)]
pub struct GpuLimits {
    pub max_texture_size: u32,
    pub max_samples: u8,
    pub max_uniform_buffer_size: u64,
    pub max_vertex_attributes: u32,
}

impl Default for GpuLimits {
    fn default() -> Self {
        Self {
            max_texture_size: 16_384,
            max_samples: 1,
            max_uniform_buffer_size: 16 * 1024,
            max_vertex_attributes: 16,
        }
    }
}

/// GPU capabilities.
#[derive(Debug, Clone)]
pub struct GpuCapabilities {
    backend: GpuBackend,
    features: Vec<GpuFeature>,
    limits: GpuLimits,
}

impl GpuCapabilities {
    pub fn new(backend: GpuBackend) -> Self {
        Self {
            backend,
            features: Vec::new(),
            limits: GpuLimits::default(),
        }
    }

    pub fn backend(&self) -> GpuBackend {
        self.backend
    }

    pub fn features(&self) -> &[GpuFeature] {
        &self.features
    }

    pub fn limits(&self) -> GpuLimits {
        self.limits
    }

    pub fn add_feature(&mut self, feature: GpuFeature) {
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
    }

    pub fn supports(&self, feature: GpuFeature) -> bool {
        self.features.contains(&feature)
    }

    pub fn set_limits(&mut self, limits: GpuLimits) {
        self.limits = limits;
    }
}

/// Logical GPU adapter description.
#[derive(Debug, Clone)]
pub struct GpuAdapter {
    name: String,
    vendor: String,
    device: String,
    driver: String,
    backend: GpuBackend,
    capabilities: GpuCapabilities,
}

impl GpuAdapter {
    pub fn new<S1, S2, S3, S4>(
        name: S1,
        vendor: S2,
        device: S3,
        driver: S4,
        backend: GpuBackend,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Self {
            name: name.into(),
            vendor: vendor.into(),
            device: device.into(),
            driver: driver.into(),
            backend,
            capabilities: GpuCapabilities::new(backend),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn vendor(&self) -> &str {
        &self.vendor
    }

    pub fn device(&self) -> &str {
        &self.device
    }

    pub fn driver(&self) -> &str {
        &self.driver
    }

    pub fn backend(&self) -> GpuBackend {
        self.backend
    }

    pub fn capabilities(&self) -> &GpuCapabilities {
        &self.capabilities
    }

    pub fn capabilities_mut(&mut self) -> &mut GpuCapabilities {
        &mut self.capabilities
    }
}

/// Logical GPU device.
#[derive(Debug, Clone)]
pub struct GpuDevice {
    adapter: GpuAdapter,
    initialized: bool,
}

impl GpuDevice {
    pub fn new(adapter: GpuAdapter) -> Self {
        Self {
            adapter,
            initialized: false,
        }
    }

    pub fn adapter(&self) -> &GpuAdapter {
        &self.adapter
    }

    pub fn initialized(&self) -> bool {
        self.initialized
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    pub fn shutdown(&mut self) {
        self.initialized = false;
    }

    pub fn backend(&self) -> GpuBackend {
        self.adapter.backend()
    }

    pub fn matches_renderer_backend(&self, backend: RenderBackend) -> bool {
        matches!(
            (backend, self.backend()),
            (RenderBackend::Auto, _)
                | (RenderBackend::OpenGL, GpuBackend::OpenGL)
                | (RenderBackend::Vulkan, GpuBackend::Vulkan)
        )
    }
}
