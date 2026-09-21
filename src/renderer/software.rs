use crate::colors::Rgba;

use super::renderer::{
    BlendState,
    ClearOptions,
    Frame,
    RenderBackend,
    RenderCommand,
    RenderError,
    Renderer,
    RendererCapabilities,
    RendererConfig,
    RendererState,
};

/// CPU-backed pixel surface.
#[derive(Debug, Clone)]
pub struct SoftwareSurface {
    width: u32,
    height: u32,
    pixels: Vec<Rgba>,
}

impl SoftwareSurface {
    pub fn new(width: u32, height: u32) -> Result<Self, RenderError> {
        if width == 0 || height == 0 {
            return Err(RenderError::InvalidDimensions);
        }

        let count = width
            .checked_mul(height)
            .ok_or(RenderError::InvalidDimensions)?;

        Ok(Self {
            width,
            height,
            pixels: vec![Rgba::TRANSPARENT; count as usize],
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[Rgba] {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut [Rgba] {
        &mut self.pixels
    }

    pub fn clear(&mut self, color: Rgba) {
        self.pixels.fill(color);
    }

    pub fn set_pixel(
        &mut self,
        x: u32,
        y: u32,
        color: Rgba,
    ) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = (y * self.width + x) as usize;
        self.pixels[index] = color;
    }

    pub fn pixel(&self, x: u32, y: u32) -> Option<Rgba> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.pixels[(y * self.width + x) as usize])
    }

    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), RenderError> {
        let replacement = Self::new(width, height)?;
        *self = replacement;
        Ok(())
    }
}

/// CPU/software renderer.
#[derive(Debug)]
pub struct SoftwareRenderer {
    config: RendererConfig,
    capabilities: RendererCapabilities,
    state: RendererState,
    surface: Option<SoftwareSurface>,
    blend: BlendState,
}

impl SoftwareRenderer {
    pub fn new(config: RendererConfig) -> Self {
        Self {
            config,
            capabilities: RendererCapabilities::software(),
            state: RendererState::Uninitialized,
            surface: None,
            blend: BlendState::Alpha,
        }
    }

    pub fn surface(&self) -> Option<&SoftwareSurface> {
        self.surface.as_ref()
    }

    pub fn surface_mut(&mut self) -> Option<&mut SoftwareSurface> {
        self.surface.as_mut()
    }

    pub fn blend_state(&self) -> BlendState {
        self.blend
    }

    fn execute_command(
        &mut self,
        command: &RenderCommand,
    ) -> Result<(), RenderError> {
        match command {
            RenderCommand::Clear(options) => {
                self.clear(options)?;
            }

            RenderCommand::SetBlend(blend) => {
                self.blend = *blend;
            }

            RenderCommand::SetViewport(_)
            | RenderCommand::SetScissor(_)
            | RenderCommand::DrawVertices { .. }
            | RenderCommand::DrawTexture { .. }
            | RenderCommand::DrawText { .. } => {
                // Geometry, texture, and text rasterization are deliberately
                // backend operations. The software backend can be expanded
                // here without changing the Renderer interface.
            }
        }

        Ok(())
    }

    fn clear(&mut self, options: &ClearOptions) -> Result<(), RenderError> {
        let surface = self
            .surface
            .as_mut()
            .ok_or(RenderError::NotInitialized)?;

        surface.clear(options.color);
        Ok(())
    }
}

impl Renderer for SoftwareRenderer {
    fn initialize(&mut self) -> Result<(), RenderError> {
        if self.state != RendererState::Uninitialized {
            return Err(RenderError::AlreadyInitialized);
        }

        self.state = RendererState::Initializing;
        self.state = RendererState::Ready;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), RenderError> {
        self.state = RendererState::ShuttingDown;
        self.surface = None;
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

        if self.surface.is_none() {
            self.surface = Some(SoftwareSurface::new(width, height)?);
        } else if let Some(surface) = self.surface.as_mut() {
            if surface.width() != width || surface.height() != height {
                surface.resize(width, height)?;
            }
        }

        self.state = RendererState::Rendering;

        Ok(Frame::new(width, height))
    }

    fn render(
        &mut self,
        frame: &Frame,
    ) -> Result<(), RenderError> {
        if self.state != RendererState::Rendering {
            return Err(RenderError::InvalidState);
        }

        for command in frame.commands() {
            self.execute_command(command)?;
        }

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

        if let Some(surface) = self.surface.as_mut() {
            surface.resize(width, height)?;
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
