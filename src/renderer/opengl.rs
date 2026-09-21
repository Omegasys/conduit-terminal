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

/// OpenGL version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OpenGlVersion {
    pub major: u8,
    pub minor: u8,
}

impl OpenGlVersion {
    pub const GL_3_3: Self = Self { major: 3, minor: 3 };
    pub const GL_4_1: Self = Self { major: 4, minor: 1 };
    pub const GL_4_5: Self = Self { major: 4, minor: 5 };
    pub const GL_4_6: Self = Self { major: 4, minor: 6 };

    pub fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }

    pub fn supports(&self, other: Self) -> bool {
        *self >= other
    }
}

/// Logical OpenGL context description.
#[derive(Debug, Clone)]
pub struct OpenGlContext {
    version: OpenGlVersion,
    core_profile: bool,
    debug_context: bool,
    adapter: GpuAdapter,
}

impl OpenGlContext {
    pub fn new(version: OpenGlVersion, adapter: GpuAdapter) -> Self {
        Self {
            version,
            core_profile: true,
            debug_context: false,
            adapter,
        }
    }

    pub fn version(&self) -> OpenGlVersion {
        self.version
    }

    pub fn core_profile(&self) -> bool {
        self.core_profile
    }

    pub fn debug_context(&self) -> bool {
        self.debug_context
    }

    pub fn adapter(&self) -> &GpuAdapter {
        &self.adapter
    }

    pub fn set_core_profile(&mut self, enabled: bool) {
        self.core_profile = enabled;
    }

    pub fn set_debug_context(&mut self, enabled: bool) {
        self.debug_context = enabled;
    }
}

/// OpenGL renderer.
#[derive(Debug)]
pub struct OpenGlRenderer {
    config: RendererConfig,
    capabilities: RendererCapabilities,
    state: RendererState,
    context: Option<OpenGlContext>,
}

impl OpenGlRenderer {
    pub fn new(config: RendererConfig) -> Self {
        Self {
            config,
            capabilities: RendererCapabilities::software(),
            state: RendererState::Uninitialized,
            context: None,
        }
    }

    pub fn context(&self) -> Option<&OpenGlContext> {
        self.context.as_ref()
    }

    fn create_context(&mut self) -> Result<(), RenderError> {
        let mut adapter = GpuAdapter::new(
            "OpenGL Adapter",
            "Unknown",
            "Unknown",
            "Unknown",
            GpuBackend::OpenGL,
        );

        let mut capabilities = adapter.capabilities().clone();

        capabilities.add_feature(GpuFeature::HardwareAcceleration);
        capabilities.add_feature(GpuFeature::Presentation);
        capabilities.add_feature(GpuFeature::TextureCompression);
        capabilities.add_feature(GpuFeature::Multisampling);
        capabilities.add_feature(GpuFeature::AnisotropicFiltering);
        capabilities.set_limits(GpuLimits {
            max_texture_size: 16_384,
            max_samples: 16,
            max_uniform_buffer_size: 65_536,
            max_vertex_attributes: 16,
        });

        *adapter.capabilities_mut() = capabilities;

        let context = OpenGlContext::new(OpenGlVersion::GL_4_5, adapter);
        self.context = Some(context);

        self.capabilities = RendererCapabilities::software();

        Ok(())
    }
}

impl Renderer for OpenGlRenderer {
    fn initialize(&mut self) -> Result<(), RenderError> {
        if self.state != RendererState::Uninitialized {
            return Err(RenderError::AlreadyInitialized);
        }

        self.state = RendererState::Initializing;

        if let Err(error) = self.create_context() {
            self.state = RendererState::Failed;
            return Err(error);
        }

        self.state = RendererState::Ready;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), RenderError> {
        self.state = RendererState::ShuttingDown;
        self.context = None;
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

        // Actual OpenGL command submission belongs here once the platform
        // window/context layer is connected.
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
