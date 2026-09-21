use super::gpu::{
    GpuAdapter,
    GpuBackend,
    GpuFeature,
    GpuLimits,
};
use super::renderer::{
    Frame,
    RenderBackend,
    RenderError,
    Renderer,
    RendererCapabilities,
    RendererConfig,
    RendererState,
};

/// Vulkan API version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VulkanApiVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
}

impl VulkanApiVersion {
    pub const V1_0: Self = Self {
        major: 1,
        minor: 0,
        patch: 0,
    };

    pub const V1_1: Self = Self {
        major: 1,
        minor: 1,
        patch: 0,
    };

    pub const V1_2: Self = Self {
        major: 1,
        minor: 2,
        patch: 0,
    };

    pub const V1_3: Self = Self {
        major: 1,
        minor: 3,
        patch: 0,
    };

    pub fn new(major: u8, minor: u8, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn supports(&self, other: Self) -> bool {
        *self >= other
    }
}

/// Requirements for initializing a Vulkan renderer.
#[derive(Debug, Clone)]
pub struct VulkanRequirements {
    minimum_api: VulkanApiVersion,
    validation_layers: bool,
    presentation: bool,
    compute: bool,
    sampler_anisotropy: bool,
}

impl Default for VulkanRequirements {
    fn default() -> Self {
        Self {
            minimum_api: VulkanApiVersion::V1_2,
            validation_layers: false,
            presentation: true,
            compute: false,
            sampler_anisotropy: true,
        }
    }
}

impl VulkanRequirements {
    pub fn minimum_api(&self) -> VulkanApiVersion {
        self.minimum_api
    }

    pub fn validation_layers(&self) -> bool {
        self.validation_layers
    }

    pub fn presentation(&self) -> bool {
        self.presentation
    }

    pub fn compute(&self) -> bool {
        self.compute
    }

    pub fn sampler_anisotropy(&self) -> bool {
        self.sampler_anisotropy
    }

    pub fn set_minimum_api(&mut self, version: VulkanApiVersion) {
        self.minimum_api = version;
    }

    pub fn set_validation_layers(&mut self, enabled: bool) {
        self.validation_layers = enabled;
    }

    pub fn set_presentation(&mut self, enabled: bool) {
        self.presentation = enabled;
    }

    pub fn set_compute(&mut self, enabled: bool) {
        self.compute = enabled;
    }

    pub fn set_sampler_anisotropy(&mut self, enabled: bool) {
        self.sampler_anisotropy = enabled;
    }
}

/// Vulkan renderer.
#[derive(Debug)]
pub struct VulkanRenderer {
    config: RendererConfig,
    requirements: VulkanRequirements,
    capabilities: RendererCapabilities,
    state: RendererState,
    api_version: VulkanApiVersion,
    adapter: Option<GpuAdapter>,
}

impl VulkanRenderer {
    pub fn new(config: RendererConfig) -> Self {
        Self {
            config,
            requirements: VulkanRequirements::default(),
            capabilities: RendererCapabilities::software(),
            state: RendererState::Uninitialized,
            api_version: VulkanApiVersion::V1_3,
            adapter: None,
        }
    }

    pub fn requirements(&self) -> &VulkanRequirements {
        &self.requirements
    }

    pub fn requirements_mut(&mut self) -> &mut VulkanRequirements {
        &mut self.requirements
    }

    pub fn api_version(&self) -> VulkanApiVersion {
        self.api_version
    }

    pub fn adapter(&self) -> Option<&GpuAdapter> {
        self.adapter.as_ref()
    }

    fn create_device(&mut self) -> Result<(), RenderError> {
        if !self.api_version.supports(self.requirements.minimum_api()) {
            return Err(RenderError::Unsupported(
                "required Vulkan API version is unavailable".into(),
            ));
        }

        let mut adapter = GpuAdapter::new(
            "Vulkan Adapter",
            "Unknown",
            "Unknown",
            "Unknown",
            GpuBackend::Vulkan,
        );

        let capabilities = adapter.capabilities_mut();

        capabilities.add_feature(GpuFeature::HardwareAcceleration);
        capabilities.add_feature(GpuFeature::Presentation);
        capabilities.add_feature(GpuFeature::TextureCompression);
        capabilities.add_feature(GpuFeature::Multisampling);
        capabilities.add_feature(GpuFeature::Synchronization);
        capabilities.add_feature(GpuFeature::TimelineSemaphores);

        if self.requirements.compute() {
            capabilities.add_feature(GpuFeature::ComputeShaders);
        }

        if self.requirements.sampler_anisotropy() {
            capabilities.add_feature(GpuFeature::AnisotropicFiltering);
        }

        capabilities.set_limits(GpuLimits {
            max_texture_size: 16_384,
            max_samples: 16,
            max_uniform_buffer_size: 65_536,
            max_vertex_attributes: 32,
        });

        self.adapter = Some(adapter);

        Ok(())
    }
}

impl Renderer for VulkanRenderer {
    fn initialize(&mut self) -> Result<(), RenderError> {
        if self.state != RendererState::Uninitialized {
            return Err(RenderError::AlreadyInitialized);
        }

        self.state = RendererState::Initializing;

        if let Err(error) = self.create_device() {
            self.state = RendererState::Failed;
            return Err(error);
        }

        self.state = RendererState::Ready;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), RenderError> {
        self.state = RendererState::ShuttingDown;
        self.adapter = None;
        self.state = RendererState::Shutdown;
        Ok(())
    }

    fn begin_frame(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<Frame, RenderError> {
        if self.state != RendererState::Ready {
            return Err(RenderError::InvalidState);
        }

        if width == 0 || height == 0 {
            return Err(RenderError::InvalidDimensions);
        }

        self.state = RendererState::Rendering;

        Ok(Frame::new(width, height))
    }

    fn render(
        &mut self,
        _frame: &Frame,
    ) -> Result<(), RenderError> {
        if self.state != RendererState::Rendering {
            return Err(RenderError::InvalidState);
        }

        // Actual Vulkan command-buffer recording/submission will be connected
        // here when the platform surface and swapchain layers are implemented.
        Ok(())
    }

    fn end_frame(&mut self) -> Result<(), RenderError> {
        if self.state != RendererState::Rendering {
            return Err(RenderError::InvalidState);
        }

        self.state = RendererState::Ready;
        Ok(())
    }

    fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), RenderError> {
        if width == 0 || height == 0 {
            return Err(RenderError::InvalidDimensions);
        }

        Ok(())
    }

    fn state(&self) -> RendererState {
        self.state
    }

    fn capabilities(&self) -> &RendererCapabilities {
        &self.capabilities
    }

    fn config(&self) -> &RendererConfig {
        &self.config
    }
}
