use crate::colors::{Color, ColorCapabilities, Rgba};

/// Rendering backend selected by Conduit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderBackend {
    Auto,
    Software,
    OpenGL,
    Vulkan,
}

impl Default for RenderBackend {
    fn default() -> Self {
        Self::Auto
    }
}

impl RenderBackend {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Software => "software",
            Self::OpenGL => "opengl",
            Self::Vulkan => "vulkan",
        }
    }
}

/// General rendering mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Normal,
    Debug,
    Minimal,
    Compatibility,
}

impl Default for RenderMode {
    fn default() -> Self {
        Self::Normal
    }
}

/// Logical renderer state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererState {
    Uninitialized,
    Initializing,
    Ready,
    Rendering,
    Suspended,
    Lost,
    Failed,
    ShuttingDown,
    Shutdown,
}

impl Default for RendererState {
    fn default() -> Self {
        Self::Uninitialized
    }
}

/// Renderer configuration shared by all backends.
#[derive(Debug, Clone)]
pub struct RendererConfig {
    backend: RenderBackend,
    mode: RenderMode,
    vsync: bool,
    multisampling: bool,
    sample_count: u8,
    triple_buffering: bool,
    hardware_acceleration: bool,
    allow_fallback: bool,
    wide_gamut: bool,
    transparency: bool,
    color_capabilities: ColorCapabilities,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            backend: RenderBackend::Auto,
            mode: RenderMode::Normal,
            vsync: true,
            multisampling: true,
            sample_count: 4,
            triple_buffering: true,
            hardware_acceleration: true,
            allow_fallback: true,
            wide_gamut: false,
            transparency: true,
            color_capabilities: ColorCapabilities::truecolor_alpha(),
        }
    }
}

impl RendererConfig {
    pub fn backend(&self) -> RenderBackend {
        self.backend
    }

    pub fn mode(&self) -> RenderMode {
        self.mode
    }

    pub fn vsync(&self) -> bool {
        self.vsync
    }

    pub fn multisampling(&self) -> bool {
        self.multisampling
    }

    pub fn sample_count(&self) -> u8 {
        self.sample_count
    }

    pub fn triple_buffering(&self) -> bool {
        self.triple_buffering
    }

    pub fn hardware_acceleration(&self) -> bool {
        self.hardware_acceleration
    }

    pub fn allow_fallback(&self) -> bool {
        self.allow_fallback
    }

    pub fn wide_gamut(&self) -> bool {
        self.wide_gamut
    }

    pub fn transparency(&self) -> bool {
        self.transparency
    }

    pub fn color_capabilities(&self) -> &ColorCapabilities {
        &self.color_capabilities
    }

    pub fn set_backend(&mut self, backend: RenderBackend) {
        self.backend = backend;
    }

    pub fn set_mode(&mut self, mode: RenderMode) {
        self.mode = mode;
    }

    pub fn set_vsync(&mut self, enabled: bool) {
        self.vsync = enabled;
    }

    pub fn set_multisampling(&mut self, enabled: bool) {
        self.multisampling = enabled;
    }

    pub fn set_sample_count(&mut self, count: u8) {
        self.sample_count = count.max(1);
    }

    pub fn set_triple_buffering(&mut self, enabled: bool) {
        self.triple_buffering = enabled;
    }

    pub fn set_hardware_acceleration(&mut self, enabled: bool) {
        self.hardware_acceleration = enabled;
    }

    pub fn set_allow_fallback(&mut self, enabled: bool) {
        self.allow_fallback = enabled;
    }

    pub fn set_wide_gamut(&mut self, enabled: bool) {
        self.wide_gamut = enabled;
    }

    pub fn set_transparency(&mut self, enabled: bool) {
        self.transparency = enabled;
    }

    pub fn set_color_capabilities(&mut self, capabilities: ColorCapabilities) {
        self.color_capabilities = capabilities;
    }
}

/// Renderer capabilities discovered from the active backend.
#[derive(Debug, Clone)]
pub struct RendererCapabilities {
    backend: RenderBackend,
    hardware_accelerated: bool,
    supports_transparency: bool,
    supports_multisampling: bool,
    supports_textures: bool,
    supports_shaders: bool,
    supports_compute: bool,
    supports_hdr: bool,
    supports_wide_gamut: bool,
    max_texture_size: u32,
    max_samples: u8,
    color_capabilities: ColorCapabilities,
}

impl RendererCapabilities {
    pub fn software() -> Self {
        Self {
            backend: RenderBackend::Software,
            hardware_accelerated: false,
            supports_transparency: true,
            supports_multisampling: false,
            supports_textures: true,
            supports_shaders: false,
            supports_compute: false,
            supports_hdr: false,
            supports_wide_gamut: false,
            max_texture_size: 16_384,
            max_samples: 1,
            color_capabilities: ColorCapabilities::truecolor_alpha(),
        }
    }

    pub fn backend(&self) -> RenderBackend {
        self.backend
    }

    pub fn hardware_accelerated(&self) -> bool {
        self.hardware_accelerated
    }

    pub fn supports_transparency(&self) -> bool {
        self.supports_transparency
    }

    pub fn supports_multisampling(&self) -> bool {
        self.supports_multisampling
    }

    pub fn supports_textures(&self) -> bool {
        self.supports_textures
    }

    pub fn supports_shaders(&self) -> bool {
        self.supports_shaders
    }

    pub fn supports_compute(&self) -> bool {
        self.supports_compute
    }

    pub fn supports_hdr(&self) -> bool {
        self.supports_hdr
    }

    pub fn supports_wide_gamut(&self) -> bool {
        self.supports_wide_gamut
    }

    pub fn max_texture_size(&self) -> u32 {
        self.max_texture_size
    }

    pub fn max_samples(&self) -> u8 {
        self.max_samples
    }

    pub fn color_capabilities(&self) -> &ColorCapabilities {
        &self.color_capabilities
    }
}

/// Logical viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Viewport {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    pub fn with_position(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 {
            0.0
        } else {
            self.width as f32 / self.height as f32
        }
    }
}

/// Scissor rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScissorRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Vertex used by backend-neutral geometry.
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
    pub texture: [f32; 2],
    pub color: Rgba,
}

impl Vertex {
    pub fn new(position: [f32; 2], color: Rgba) -> Self {
        Self {
            position,
            texture: [0.0, 0.0],
            color,
        }
    }

    pub fn textured(
        position: [f32; 2],
        texture: [f32; 2],
        color: Rgba,
    ) -> Self {
        Self {
            position,
            texture,
            color,
        }
    }
}

/// Basic blending modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendState {
    Disabled,
    Alpha,
    PremultipliedAlpha,
    Additive,
}

impl Default for BlendState {
    fn default() -> Self {
        Self::Alpha
    }
}

/// Texture handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureId(pub u64);

/// Framebuffer description.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub samples: u8,
    pub has_alpha: bool,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            samples: 1,
            has_alpha: true,
        }
    }
}

/// Frame being rendered.
#[derive(Debug, Clone)]
pub struct Frame {
    framebuffer: Framebuffer,
    viewport: Viewport,
    clear_color: Color,
    commands: Vec<RenderCommand>,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            framebuffer: Framebuffer::new(width, height),
            viewport: Viewport::new(width, height),
            clear_color: Color::rgb(0, 0, 0),
            commands: Vec::new(),
        }
    }

    pub fn framebuffer(&self) -> Framebuffer {
        self.framebuffer
    }

    pub fn viewport(&self) -> Viewport {
        self.viewport
    }

    pub fn clear_color(&self) -> &Color {
        &self.clear_color
    }

    pub fn commands(&self) -> &[RenderCommand] {
        &self.commands
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn push(&mut self, command: RenderCommand) {
        self.commands.push(command);
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }
}

/// Backend-neutral render commands.
#[derive(Debug, Clone)]
pub enum RenderCommand {
    Clear(ClearOptions),
    SetViewport(Viewport),
    SetScissor(Option<ScissorRect>),
    SetBlend(BlendState),
    DrawVertices {
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
    DrawTexture {
        texture: TextureId,
        position: [f32; 2],
        size: [f32; 2],
        tint: Rgba,
    },
    DrawText {
        position: [f32; 2],
        text: String,
        color: Rgba,
        size: f32,
    },
}

/// Clear operation.
#[derive(Debug, Clone, Copy)]
pub struct ClearOptions {
    pub color: Rgba,
    pub depth: bool,
    pub stencil: bool,
}

impl ClearOptions {
    pub fn color(color: Rgba) -> Self {
        Self {
            color,
            depth: false,
            stencil: false,
        }
    }
}

/// Renderer errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    NotInitialized,
    AlreadyInitialized,
    BackendUnavailable(RenderBackend),
    DeviceLost,
    InvalidDimensions,
    InvalidState,
    Unsupported(String),
    Backend(String),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInitialized => write!(formatter, "renderer is not initialized"),
            Self::AlreadyInitialized => write!(formatter, "renderer is already initialized"),
            Self::BackendUnavailable(backend) => {
                write!(formatter, "renderer backend unavailable: {}", backend.name())
            }
            Self::DeviceLost => write!(formatter, "rendering device was lost"),
            Self::InvalidDimensions => write!(formatter, "invalid render dimensions"),
            Self::InvalidState => write!(formatter, "invalid renderer state"),
            Self::Unsupported(message) => write!(formatter, "unsupported operation: {message}"),
            Self::Backend(message) => write!(formatter, "backend error: {message}"),
        }
    }
}

impl std::error::Error for RenderError {}

/// Backend-neutral renderer interface.
pub trait Renderer {
    fn initialize(&mut self) -> Result<(), RenderError>;
    fn shutdown(&mut self) -> Result<(), RenderError>;

    fn begin_frame(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<Frame, RenderError>;

    fn render(
        &mut self,
        frame: &Frame,
    ) -> Result<(), RenderError>;

    fn end_frame(&mut self) -> Result<(), RenderError>;

    fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), RenderError>;

    fn state(&self) -> RendererState;
    fn capabilities(&self) -> &RendererCapabilities;
    fn config(&self) -> &RendererConfig;
}
